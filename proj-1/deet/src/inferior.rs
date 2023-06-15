use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::process::{Child, Command};
use std::os::unix::process::CommandExt;
use std::mem::size_of;

use std::collections::HashMap;
use crate::debugger::Breakpoint;

pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

fn align_addr_to_word(addr: usize) -> usize {
    addr & (-(size_of::<usize>() as isize) as usize)
}

pub struct Inferior {
    child: Child,
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>, breakpoints: &mut HashMap<usize, Option<Breakpoint>>) -> Option<Inferior> {
        let mut cmd = Command::new(target);
        cmd.args(args);
        // The unsafe block acts as a warning to avoid allocating memory or accessing 
        // shared data in the presence of threads
        unsafe {
            cmd.pre_exec(child_traceme);
        }

        if let Ok(child) = cmd.spawn() {
            // delivery the breakpoints information to child process
            let mut inferior = Inferior { child: child };
            for (addr, breakpoint ) in breakpoints {
                match inferior.write_byte(*addr, 0xcc as u8) {
                    Ok(orig_byte) => {
                        *breakpoint = Some(
                            Breakpoint {
                                addr: *addr,
                                orig_byte: orig_byte,
                            }
                        );
                    },
                    Err(err) => println!("Inferior::write_byte for breakpoint error {}", err),
                }
            }
            Some(inferior) // return the child process
        } else {
            None
        }
    }

    pub fn continue_exec(&mut self, breakpoints: &HashMap<usize, Option<Breakpoint>>) -> Result<Status, nix::Error> {
        // we need to update regs as long as we update rip
        let mut regs = ptrace::getregs(self.pid())?;
        let rip = regs.rip as usize;
        if let Some(breakpoint_wrapper) = breakpoints.get(&(rip - 1)) {
            if let Some(breakpoint) = breakpoint_wrapper {
                let addr = breakpoint.addr;
                let orig_byte = breakpoint.orig_byte;
                // restore the first byte of the instruction we replaced
                // afterwards we update the instruction pointer address content
                self.write_byte(addr, orig_byte).unwrap();
                regs.rip = (rip - 1) as u64;
                ptrace::setregs(self.pid(), regs).unwrap();
                // step to next instruction
                ptrace::step(self.pid(), None).unwrap();
                match self.wait(None) {
                    Ok(status) => {
                        match status {
                            Status::Exited(code) => return Ok(Status::Exited(code)),
                            Status::Signaled(sig) => return Ok(Status::Signaled(sig)),
                            _ => {
                                // restore 0xcc in the breakpoint position
                                self.write_byte(addr, 0xcc).unwrap();
                            },
                        }
                    },
                    Err(err) => println!("Inferior::continue_exec wait error {}", err),
                }
            }
        }
        ptrace::cont(self.pid(), None)?; // Restart the stopped trace process
        self.wait(None)
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }

    pub fn kill(&mut self) {
        println!("Killing running inferior (pid {})", self.pid());
        match self.child.try_wait() {
            Ok(None) => self.child.kill().expect("Child has already exited before you call 'kill'"),
            _ => (), // child has exited or no child need to wait
        }
    }

    pub fn write_byte(&mut self, addr: usize, val: u8) -> Result<u8, nix::Error> {
        let aligned_addr = align_addr_to_word(addr);
        let byte_offset = addr - aligned_addr;
        let word = ptrace::read(self.pid(), aligned_addr as ptrace::AddressType)? as u64;
        let orig_byte = (word >> 8 * byte_offset) & 0xff;
        let masked_word = word & !(0xff << 8 * byte_offset);
        let updated_word = masked_word | ((val as u64) << 8 * byte_offset);
        unsafe {
            ptrace::write(
                self.pid(),
                aligned_addr as ptrace::AddressType,
                updated_word as *mut std::ffi::c_void,
            )?;
        }
        Ok(orig_byte as u8)
    }

}
