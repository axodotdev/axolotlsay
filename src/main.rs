use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    msg: String,
}

fn main() {
    let args = Args::parse();

    println!("         +----------------+");
    println!("         | {} |", args.msg);
    println!("         +----------------+");
    println!("        /");
    println!("≽(◕ ᴗ ◕)≼");
}
