
fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter().skip(1) {
        println!("Linking argument: {}", arg);
    }
    std::process::exit(1);
}