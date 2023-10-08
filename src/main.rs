use std::env;
use std::process;
use minigrep::Config;
fn main() {
    
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        // print errors to the standard error stream, will not be written in output.txt
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    
    if let Err(e) = minigrep::run(config){
        // print errors to the standard error stream, will not be written in output.txt
        eprintln!("Application error: {e}");
        process::exit(2);
    };
}

