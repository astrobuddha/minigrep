use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() != 3 {
            return Err("incorrect number of carguments");
        }

        let query = &args[1].clone();
        let file_path = &args[2].clone();

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string()
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();


    for line in _contents.lines() {
        if line.contains(_query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query_lower = query.to_string().to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            results.push(line);
        }
    }

    results
}