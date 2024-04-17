use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/ini.peg"]
struct IniParser;

const INI_DATA: &str = include_str!("../data/data.ini");

#[test]
pub fn main() {
    let file = IniParser::parse(Rule::File, INI_DATA)
        .expect("data.ini is valid")
        .next()
        .expect("Rule::File was parsed successfully");

    let mut properties = HashMap::<&str, HashMap<&str, &str>>::new();

    let mut current_section_name = "";

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::Section => {
                let mut inner_rules = line.into_inner(); // { name }
                current_section_name = inner_rules
                    .next()
                    .expect("Rule::Section only has one inner rule")
                    .as_str();
            }
            Rule::Property => {
                let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

                let name = inner_rules
                    .next()
                    .expect("Rule::Property has a rule before equal sign")
                    .as_str();
                let value = inner_rules
                    .next()
                    .expect("Rule::Property has a rule after equal sign")
                    .as_str();

                // Insert an empty inner hash map if the outer hash map hasn't
                // seen this section name before.
                let section = properties.entry(current_section_name).or_default();
                section.insert(name, value);
            }
            Rule::EOI => break,
            Rule::WHITESPACE | Rule::Character | Rule::Name | Rule::Value | Rule::File => {
                unreachable!()
            }
        }
    }

    println!("{}", INI_DATA);
    dbg!(properties);
}
