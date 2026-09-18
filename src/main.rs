use std::{env, io::{Write, stdin, stdout}, path::Path, process::Command};

fn main() {
    loop {
        let current_dir = env::current_dir();
        match current_dir {
            Ok(current_dir) => print!("{}> ", current_dir.display()),
            Err(e) => eprintln!("{}", e)
        }
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "exit" => return ,

            "cd" => {
              let new_dir = args.peekable().peek().map_or("/", |x| *x);
              let root = Path::new(new_dir);
              if let Err(e) = env::set_current_dir(&root) {
                  eprintln!("{}", e);
              }
            },

            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn();
                
                match child {
                    Ok(mut child) => { let _ = child.wait(); },
                    Err(e) => eprintln!("{}", e)
                };
            }
        }
    }
}
