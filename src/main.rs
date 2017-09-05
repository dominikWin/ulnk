mod args;

fn main() {
    println!("Hello World!");
    let _ = args::Args::parse();
}
