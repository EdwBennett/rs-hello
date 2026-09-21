use clap::Parser;

/// Print a greeting.
#[derive(Parser)]
struct Args {
    /// Who to greet.
    #[arg(short, long, default_value = "")]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", hello_core::greet(&args.name));
}
