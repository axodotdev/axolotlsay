#![allow(clippy::uninlined_format_args)]
use clap::Parser;
use unicode_width::UnicodeWidthStr;

use cue::cd::CD;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    msg: String,
}

fn main() {
    CD::parse(
        r#"FILE "basic_image.bin" BINARY
          TRACK 01 MODE1/2352
            INDEX 01 00:00:00
    "#
        .to_owned(),
    )
    .unwrap();

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
