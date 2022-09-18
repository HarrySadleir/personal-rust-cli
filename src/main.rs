use std::env;
use std::process;
use cli::tasks;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);    
    let command: &[String] = &args[1..];

    match command {
        [] => {
            eprintln!("No commands provided. Please provide at least one command");
            process::exit(exitcode::USAGE);
        },
        // -------------------------- Minigrep ---------------------------------
        [task, ..] if task.eq("minigrep") => {
            let config = tasks::minigrep::Config::new(&command[1..]).unwrap_or_else(|e: String| {
                eprint!("Problem parsing arguments: {}", e);
                process::exit(exitcode::USAGE);
            });

            if let Err(e) = tasks::minigrep::run(config) {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        },
        // ----------------------Command did not match -------------------------
        _ => {
            eprintln!("Command did not match available commands. Please provide at least one command");
            process::exit(exitcode::USAGE);
        },
    }
}