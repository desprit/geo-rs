use env_logger;
use geo_rs;

fn main() {
    env_logger::init();
    let parser = geo_rs::Parser::new();
    let location = std::env::args().nth(1).expect("no location given");
    let output = parser.parse_location(&location);
    println!(">> {}", output);
}
