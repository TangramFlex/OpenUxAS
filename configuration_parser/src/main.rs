extern crate configuration_parser;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("../examples/02_Example_WaterwaySearch/cfg_WaterwaySearch.xml").unwrap();
    let file = BufReader::new(file);

    println!("Parsed: {:?}", configuration_parser::UxAS::from_xml(file));

/*
    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
*/
}
