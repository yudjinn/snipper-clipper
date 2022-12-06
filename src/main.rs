pub use crate::structs::{Config, Persist, Snippet, Snippets};
pub use crate::utils::constants::HELP_MESSAGE;
pub use crate::utils::error::{Error, Result};
use std::io::{BufRead, Read};
mod structs;
mod utils;

pub fn main() -> Result<()> {
    println!("make config obj");
    let mut config = Config::default()?;
    println!("try to update with stored vals");
    config = config.load()?;
    println!("make snippets obj");
    let mut snippets = Snippets::default()?;
    println!("try to update with stored vals");
    snippets = snippets.load()?;
    println!("{:?}", snippets);

    // TODO change how this handles. I feel like the flow is dumb
    // if let mut stdin = std::io::stdin() {
    //     let mut input = String::new();
    //     while let Some(line) = stdin.lines().next() {
    //         let last_input = line.unwrap();
    //         if last_input.len() == 0 {
    //             break;
    //         }

    //         if input.len() > 0 {
    //             input.push_str("\n");
    //         }
    //         input.push_str(&last_input);
    //     }
    //     // let input: Vec<std::result::Result<String, std::io::Error>> = stdin.lines().collect();
    //     let snippet = Snippet::from_stdin(&input);
    //     return;
    // }
    let args: Vec<String> = std::env::args().collect();
    println!("collected args: {:?} size: {}", args, args.len());
    if let Some(stdin) = read_stdin() {
        println!("Found some stdin");
        // let snippet = Snippet::from_stdin(&stdin);
        let snippet = Snippet::from_string(&args[2], &stdin);
        snippets.add(snippet);
        return Ok(());
    }

    if args.len() > 1 {
        match args[1].as_str() {
            "list" => {
                if args.len() > 2 && args[2].as_str() != "" {
                    snippets.list_by_lang(&args[2].as_str())?;
                } else {
                    snippets.list_all();
                }
            }
            "get" => {
                if let Some(snip) = snippets.get(&args[2]) {
                    println!("{}", snip.body);
                } else {
                    println!("No snippet found with the name: {}", &args[2]);
                }
            }
            "find" => {
                if let Some(snips) = snippets.find(&args[2]) {
                    println!("{:#?}", snips);
                } else {
                    println!("No snippet found with the name: {}", &args[2]);
                }
            }
            "help" => {
                println!("{}", HELP_MESSAGE);
            }
            "new" => {
                let name = &args[2];
                let body = &args[3];
                let snippet = Snippet::from_string(name, body);
                println!("Snippet made: {:?}", snippet);
                snippets.add(snippet);
            }
            _ => {
                println!("{}", HELP_MESSAGE);
            }
        }
        return Ok(());
    }
    match run_interactive(config, snippets) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::msg(format!("An error occurred: {}", e))),
    }
}

fn run_interactive(config: Config, snippets: Snippets) -> Result<()> {
    todo!()
}

fn read_stdin() -> Option<String> {
    println!("reading stdin");
    let stdin = std::io::stdin();
    let mut lines = stdin.lines();
    let mut input = String::new();
    while let Some(line) = lines.next() {
        let last = line.unwrap();
        if last.len() == 0 {
            break;
        }

        if input.len() > 0 {
            input.push_str("\r");
        }

        input.push_str(&last);
    }
    if input.len() > 0 {
        println!("{}", input);
        return Some(input);
    }
    return None;
}
