use std::fs;
use std::error::Error;

pub struct Config {
    filepath: String,
    query: String
}

impl Config {

    // `args` is a collection of the the command line arguments passed to the program.
    pub fn new(args: &[String]) -> Result<Config, String> {

        if args.len() < 3 {
            return Err(String::from("Not enough arguments"));
        }

        // The first argument is ignored since it's the name of the binary.
        let filepath = &args[1];
        let query = &args[2];
    
        Ok(Config {
            filepath: filepath.to_owned(),
            query: query.to_owned()
        })
    }    
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {

    let filepath = &config.filepath;
    let query = &config.query;

    println!("Searching {} for lines containing: '{}'", filepath, query);

    let file_contents = fs::read_to_string(filepath)?;
    let lines = search(query, &file_contents);

    println!("Found {} lines!", lines.len());

    for line in lines {
        println!("-> {}", line);
    }
    
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn search_returns_matches() {
        
        let query = "you";
        let contents = "
Hello,
How are you?
I hope you are doing ok
        ";

        assert_eq!(vec!["How are you?", "I hope you are doing ok"], search(query, contents));
    }

    #[test]
    pub fn parse_config() -> Result<(), Box<dyn Error>> {

        let args = vec!["./name-of-binary".to_owned(), "test.txt".to_owned(), "today".to_owned()];
        let config = Config::new(&args)?;

        assert_eq!("test.txt", &config.filepath);
        assert_eq!("today", &config.query);

        Ok(())
    }

    #[test]
    pub fn run_with_config() -> Result<(), Box<dyn Error>> {

        let config = Config {
            filepath: "test.txt".to_owned(),
            query: "today".to_owned()
        };

        run(config)?;

        Ok(())
    }
}
