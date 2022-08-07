use std::{env, fs::File, io::prelude::*};
fn main() {
    let args:Vec<String> = env::args().collect();
    let config = Config::new(&args); 
    println!("searching for {}",config.query);
    println!("In file {}",config.filename);
    
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:^n{}",contents);
}
struct Config{
    query:String,
    filename:String,
}
impl Config{
    fn new(args:&[String])->Config{
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename}
    }
}