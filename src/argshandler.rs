use std::env;

pub fn handle_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        //Initalize GUI Interface
        println!("TODO: Implement GUI");
        return run_gui();
    }
    else {
        let mut gui = true;
        for arg in args {
            match &arg as &str {
                "--cli" => if gui { gui = false; return run_cli() } else { return run_cli() },
                "--help" => {
                    println!("Welcome to Task App!\n
Run in CLI with args:
--cli      <- Runs the program in CLI mode
--gui      <- Runs the program in GUI mode (enabled by default)
--help, -h <- Displays help
                    ");
                },
                "-h" => {
                    println!("Welcome to Task App!\n
                    Run in CLI with args:
                    --cli      <- Runs the program in CLI mode
                    --gui      <- Runs the program in GUI mode (enabled by default)
                    --help, -h <- Displays help
                    ");
                },
                "--gui" => if gui { return run_gui() } else { println!("CLI already selected!") },
                _ => (),
            }
        }
    }
    run_gui()
}

fn run_cli() -> String {
    let ret: String = String::from("cli");
    ret
}

fn run_gui() -> String {
    let ret: String = String::from("gui");
    ret
}
