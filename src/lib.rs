use std::{env, error::Error, fs::File, io::prelude::*};
///オプション情報
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args)-> Result<Config, &'static str> {
        args.next();
        let query = match args.next(){
            Some(arg) =>arg,
            None => return Err("Didn't get a puery string"),
        };
        let filename = match args.next(){
            Some(arg)=>arg,
            None => return Err("Didn't get a file name"),
        };
        let ignore_case = env::var("CASE_IGNORE").is_ok();
        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}
/// 指定されたテキストを読み、指定されたキーワードがある行を表示する
/// 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
///文字列(contents)内の指定文字列（query）が存在する行を返す
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
///文字列(contents)内の指定文字列（query）が存在する行を返す
///大文字小文字の区別をしない
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "/
Rust:
safe,fast,productive.
pick three.";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "/
Rust:
safe,fast,productive.
pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
