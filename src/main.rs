use std::{
    env, io::{
        Write, stdin, stdout
    }, path::Path, process::{
        Child, Command, Stdio
    }
};


fn main() {
    // Stacks for storing previously executed commands
    let mut previous_stack: Vec<String> = Vec::new();
    let mut next_stack: Vec<String> = Vec::new();

    loop {
        // Print current directory with shell input
        let current_dir = env::current_dir();
        match current_dir {
            Ok(current_dir) => print!("{}> ", current_dir.display()),
            Err(e) => eprintln!("{}", e)
        }
        let _ = stdout().flush();

        // Read input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Prepare for a piped input
        let mut commands = input.trim().split("|").peekable();
        let mut previous_command = None;

        // Log last executed command
        previous_stack.push(input.clone());
    
        while let Some(command) = commands.next() {

            // Split input string into primary command and arguments
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            // Execute the command input, builtins are defined independently
            match command {
                // Terminate shell instance
                "exit" => return ,

                // Change directory
                "cd" => {
                  let new_dir = args.peekable().peek().map_or("/", |x| *x);
                  let root = Path::new(new_dir);
                  if let Err(e) = env::set_current_dir(&root) {
                      eprintln!("{}", e);
                  }

                  previous_command = None;
                },

                // Execution of non-builtin commands
                command => {
                    let stdin = previous_command.map_or(
                        Stdio::inherit(), 
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let stdout = if commands.peek().is_some() {
                        // Prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // Send output to shell stdout
                        Stdio::inherit()
                    };
                
                    let mut output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    
                    match output {
                        Ok(mut ouput) => { previous_command = Some(ouput); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
            if let Some(ref mut final_command) = previous_command {
                // Block thread until the final command has finished
                let _ = final_command.wait();
            }
        }        
    }
}
