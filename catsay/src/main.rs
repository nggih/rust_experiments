extern crate structopt;
use colored::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value= "Meow!")]
    message: String, // [1]
    #[structopt(short= "d", long = "dead")]
    // What does the cat say?
    dead: bool,
}
fn main() {
    // let message = std::env::args().nth(1)
    //     .expect("Missing the message. Usage: catsay < message >");
    let options = Options::from_args(); // [2]
    let message = options.message;

    let eye = if options.dead { "x" } else { "o" };
    println!("{}", message.bright_yellow().underline().on_purple());
    // .. print the cat
    println!("\\");
    println!(" \\ ");
    println!("  /\\_____/\\ ");
    println!(" /  {eye}   {eye}  \\ ", eye=eye.red().bold());
    println!("( ==  ^  == )");
}
