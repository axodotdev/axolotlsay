#![allow(clippy::uninlined_format_args)]
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    msg: String,
}

fn main() {
    let args = Args::parse();

    let msg = &args.msg;
    let dashes = "-".repeat(args.msg.len() + 2);

    println!("         +{}+", dashes);
    println!("         | {} |", msg);
    println!("         +{}+", dashes);
    println!("        /");
    println!("≽(◕ ᴗ ◕)≼");
}
