//use minigrep::search;

#[cfg(test)]
mod tests {

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents));
    }

    #[test]
    fn case_sensitive_when_uppercase_expect_false() {
        let query = "dUCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(minigrep::search(query, contents).len(), 0);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], minigrep::search_case_insensitive(query, contents));
    }
}