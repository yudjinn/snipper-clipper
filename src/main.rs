fn main() {
    let config = Config::read();
    let snippets = Snippets::read();
    
    // TODO change how this handles. I feel like the flow is dumb
    if let Some(stdin) = read_stdin() {
        let snippet = Snippet::from_stdin(stdin);
        return;
    }
    let args Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match &args.nth(1) {
            "list" => {
                snippets.list_all();
            }
            "find" => {
                let snippet = snippets.find(&args.nth(2));
                snippet.print();
            }
            "help" => {
                println!("{}", HELP_MESSAGE);
            }
            "new" => {
                let name = parse_name(&args.nth(2));
                let body = &args.nth(3);
                let snippet = Snippet::from_string(name, body);
                snippets.add(snippet);
            }
            _ => {
                println!("{}", HELP_MESSAGE);
            }
        }
        return;
    }
    match run_interactive(config, snippets) {
        Err(e) => {
            eprintln!("An error occurred: {}",e);
        }
    }
}


fn read_stdin() -> Option(String) {
    match std::os::Stdin::stat() {
        Ok(stat) => {
            if stat.Mode() & std::os::ModeCharDevice != 0 {
                return None;
            }
            let mut body: Vec<String> = Vec::new(); // use string builder??
            // not sure how to handle this yet, def !TODO
            body = std::os::stdin().collect();
            return Some(body);
        }
        Err(e) => {
            eprintln!("Could not read stdin: {}",e);
            return None;
        } 
    }
 }

fn run_interactive(config: Config, snippets: Snippets) -> Result<(), Error::Generic> {
    todo!()
}