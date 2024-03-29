use::std::env;
use::std::process;
use::minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| { // this is a closure that takes an error and returns a value
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    // eprintln! is like println! but prints to the standard error stream
    // so if we redirect the output of the program to a file, the error messages will still be displayed on the screen
    // eg. cargo run > output.txt
    // if error, we can see the error message on the screen/terminal
    // if no error, we can see the output in the file
}
