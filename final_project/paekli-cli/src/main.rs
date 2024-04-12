use clap::Parser;

/// send and receive pure distilled awesomeness 🍸
#[derive(Parser)]
#[clap(version)]
struct Cli;

fn main() {
    let _args = Cli::parse();
    println!("Paekli LLC is currentli closed 😢");
}