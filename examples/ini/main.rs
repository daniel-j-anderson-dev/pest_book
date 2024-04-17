use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "examples/ini/ini.peg"]
struct IniParser;

const INI_DATA: &str = include_str!("data.ini");

pub fn main() {
    let file = IniParser::parse(Rule::File, INI_DATA).expect("Failed to parse").next().expect("If Rule::File was parsed");

    let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    let mut current_section_name = "";

    // for line in file {

    // }
}