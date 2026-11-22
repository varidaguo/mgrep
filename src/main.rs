use std::env;
use std::process;

use mgrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("need more 3: {err}");
        process::exit(1);
    });

    if let Err(e) = mgrep::run(config) {
        println!("err: {e}");
        process::exit(1);
    }


}