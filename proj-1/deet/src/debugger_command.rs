pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Continue,
    Backtrace,
    Breakpoint(Option<String>),
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            },
            "c" | "cont" | "continue" => Some(DebuggerCommand::Continue),
            "bt" | "back" | "backtrace" => Some(DebuggerCommand::Backtrace),
            "b" | "break" | "breakpoint" => {
                if tokens.len() >= 2 {
                    let addr = tokens[1].to_string();
                    if addr.len() >= 2 && addr.chars().nth(0).unwrap() == "*".to_string().chars().nth(0).unwrap() {
                        return Some(DebuggerCommand::Breakpoint(Some(addr[1..].to_string())))
                    }
                }
                println!("Usage: b|break|breakpoint *address");
                // command length not satisfy the requirement
                Some(DebuggerCommand::Breakpoint(None))
            }
            // Default case:
            _ => None,
        }
    }
}