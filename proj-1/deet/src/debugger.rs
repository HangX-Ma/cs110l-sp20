use crate::debugger_command::DebuggerCommand;
use crate::inferior::{Inferior, Status};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use rustyline::history::FileHistory;
use nix::sys::ptrace;
// debugging symbols
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Breakpoint {
    pub addr: usize,
    pub orig_byte: u8,
}


pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<(), FileHistory>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: HashMap<usize, Option<Breakpoint>>,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // TODO (milestone 3): initialize the DwarfData
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };
        debug_data.print();

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<(), FileHistory>::new().expect("Create Editor fail");
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints: HashMap::new(),
        }
    }

    fn inferior_release_try(&mut self) {
        if let Some(old_inferior) = self.inferior.as_mut() {
            old_inferior.kill();
        }
    }

    fn inferior_continue_exec(&mut self, breakpoints: &HashMap<usize, Option<Breakpoint>>) {
        if let Some(inferior) = self.inferior.as_mut() {
            match inferior.continue_exec(breakpoints) {
                Ok(status) => match status {
                    Status::Stopped(sig, ptr) => {
                        println!("Child stopped (signal {}, address {:#x})", sig, ptr);
                        let regs = ptrace::getregs(inferior.pid()).unwrap();
                        let line_t = DwarfData::get_line_from_addr(&self.debug_data, regs.rip as usize).unwrap();
                        let func_name = DwarfData::get_function_from_addr(&self.debug_data, regs.rip as usize).unwrap();
                        println!("Stopped at {} ({})", func_name, line_t);
                    },
                    Status::Signaled(sig) => println!("Child exited (signal {})", sig),
                    Status::Exited(ret) => println!("Child exited (status {})", ret),
                },
                Err(err) => print!("Child error ({})", err),
            }
        } else {
            println!("No inferior found");
        }
    }

    fn print_backtrace(&mut self) -> Result<(), nix::Error> {
        // Starter code: Ok(println!("Hello world"))
        if let Some(inferior) = self.inferior.as_mut() {
            let regs = ptrace::getregs(inferior.pid()).unwrap();
            let mut instruction_ptr = regs.rip;
            let mut base_ptr = regs.rbp;
            loop {
                let line_t = DwarfData::get_line_from_addr(&self.debug_data, instruction_ptr as usize).unwrap();
                let func_name = DwarfData::get_function_from_addr(&self.debug_data, instruction_ptr as usize).unwrap();
                println!("{} ({})", func_name, line_t);
                if func_name == "main".to_string() {
                    break;
                }
                instruction_ptr = ptrace::read(inferior.pid(), (base_ptr + 8) as ptrace::AddressType)? as u64;
                base_ptr = ptrace::read(inferior.pid(), base_ptr as ptrace::AddressType)? as u64;
            }
            Ok(())
        } else {
            Err(nix::Error::ECHILD)
        }
    }

    pub fn parse_address(&mut self, addr: &str) -> Option<usize> {
        let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
            &addr[2..]
        } else {
            &addr
        };
        usize::from_str_radix(addr_without_0x, 16).ok()
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    // kill the previous inferior if it exists
                    self.inferior_release_try();

                    // Create new inferior
                    if let Some(inferior) = Inferior::new(&self.target, &args, &mut self.breakpoints) {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                        self.inferior_continue_exec(&self.breakpoints.clone());
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Quit => {
                    self.inferior_release_try();
                    return;
                },
                DebuggerCommand::Continue => {
                    if let Some(_) = self.inferior.as_mut() {
                        self.inferior_continue_exec(&self.breakpoints.clone());
                    } else {
                        // press 'c' before 'r'
                        println!("No process running error")
                    }
                },
                DebuggerCommand::Backtrace => {
                    self.print_backtrace().expect("Nothing");
                },
                DebuggerCommand::Breakpoint(addr_wrapper) => {
                    match addr_wrapper {
                        Some(addr) => {
                            // parse the addr string which has no prefix symbol '*'
                            if let Some(parsed_addr) = self.parse_address(addr.as_str()) {
                                // set breakpoints if inferior exists
                                if let Some(inferior) = self.inferior.as_mut() {
                                    match inferior.write_byte(parsed_addr, 0xcc as u8) {
                                        Ok(orig_byte) => {
                                            self.breakpoints.insert(
                                                parsed_addr, 
                                                Some(Breakpoint{
                                                    addr: parsed_addr,
                                                    orig_byte: orig_byte
                                                }));
                                        },
                                        Err(err) => println!("Inferior::write_byte for breakpoint error {}", err),
                                    }
                                } else {
                                    self.breakpoints.insert(parsed_addr, None);
                                }
                                println!("Set breakpoint {} at {:#x}", self.breakpoints.len() - 1, parsed_addr);
                            } else {
                                println!("fail to parse a usize from a hexadecimal string");
                            }
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str()).expect("Can not add history entry");
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
