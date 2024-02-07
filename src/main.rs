use std::fs;

fn main() {
    let file = fs::read_to_string("resources/bom.1.4.json").unwrap();
    let cyclonedx = cyclonedx_bom::prelude::Bom::parse_from_json(file.as_bytes()).expect("Could not parse cyclonedx");
    println!("File successfully parsed. SpecVersion: {}", cyclonedx.version);
}
