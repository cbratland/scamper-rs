use std::env;
use std::io::Read;
use std::process;

use interpreter::Output;
use scamper_rs::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn usage() -> ! {
    eprintln!("Usage: scamper [run <file>]");
    process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return repl();
    }

    if args.len() != 3 || args[1] != "run" {
        usage();
    }

    // read input file
    let file_name = args.last().unwrap();
    let mut src = String::new();
    let mut input_file = std::fs::File::open(file_name).expect("no file found");
    input_file
        .read_to_string(&mut src)
        .expect("failed to read file");

    // println!("{}", src);

    let engine = Engine::new();

    match engine.run(&src) {
        Ok(_) => (),
        Err(err) => {
            err.emit(file_name, &src);
        }
    }
}

fn repl() {
    println!("Welcome to Scamper (Rust) v0.1");

    let engine = Engine::new();

    let mut rl = Editor::<(), rustyline::history::DefaultHistory>::new()
        .expect("Failed to create line editor");

    // _ = rl.load_history("history.txt");

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                // Add entry to history
                _ = rl.add_history_entry(line.as_str());

                match engine.run(&line) {
                    Ok(items) => {
                        for item in items {
                            match item {
                                Output::Value(value) => println!("{}", value),
                                Output::Error(err) => {
                                    err.emit(&line);
                                }
                            };
                        }
                    }
                    Err(err) => {
                        err.emit("repl", &line);
                    }
                }
            }
            // ctrl-c or ctrl-d
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // if let Err(err) = rl.save_history("history.txt") {
    //     println!("Error saving history: {}", err);
    // }
}
