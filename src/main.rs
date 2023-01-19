use std::env;
use std::fs;
use std::process;

// Prints and returns the value of a given expression for quick and dirty debugging.
// dbg!(args);

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });




    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}


fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String
}


impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() != 3 {
            Return Err(&"incorrect number of arguments") // todo fix this
        }

        let query = &args[1].clone();
        let file_path = &args[2].clone();

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string()
        })
    }
}
