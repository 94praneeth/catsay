extern crate colored;
extern crate exitfailure;
extern crate failure;
extern crate structopt;

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;
use std::io::{self, Read};

// derive(StructOpt) tells Rust to generate a command line parser,
// and the various structopt attributes are simply used for additional parameters.
#[derive(StructOpt)]
struct Options {
    // User wouldn't know what <message> means in the help message.
    // Hence the below macro.
    #[structopt(default_value = "Meow!")]
    // Below is rust document comment. StructOpt will show this in help.
    /// What does the cat say?
    message: String,

    // Switches make customized output. To add a switch do as below.
    #[structopt(short = "d", long = "dead")]
    /// Makes the cat appear dead.
    dead: bool,

    // Adding custom cat image from a file.
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from a specific file
    catfile: Option<std::path::PathBuf>,

    // Accepting standered input
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of arguments
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    // let message = options.message;
    let mut message = String::new();
    let eye = if options.dead { "x" } else { "o" };

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("Cat shouldn't bark like a dog");
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("Could not read file {:?}", path))?;
            // .expect(&format!("Could not read file: {:?}", path));
            println!("{}", message.yellow().bold());
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
        }
        None => {
            println!("{}", message.yellow());
            println!(" \\");
            println!("  \\");
            println!("      /\\_/\\");
            println!("     ( {eye} {eye} )", eye = eye.red().bold());
            println!("    ==\\ I /==");
        }
    }
    Ok(())
}
