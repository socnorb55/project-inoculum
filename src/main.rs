use clap::Parser;

#[derive(Parser)]
struct Cli {
    test: String
}

fn main() {

    let args: Cli = Cli::parse();

    println!("Hello, {:?}!", args.test);
}
