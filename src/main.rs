#![allow(clippy::uninlined_format_args)]
use clap::Parser;
use unicode_width::UnicodeWidthStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    msg: String,
}

fn main() {
    let args = Args::parse();
    let msg = &args.msg;
    let count = UnicodeWidthStr::width(args.msg.as_str());
    let dashes = "-".repeat(count + 2);
    println!("         +{}+", dashes);
    println!("         | {} |", msg);
    println!("         +{}+", dashes);
    println!("        /");
    println!("≽(◕ ᴗ ◕)≼");
}
