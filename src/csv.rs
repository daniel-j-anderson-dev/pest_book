// Trait to facilitate parsing text of a known grammar
use pest::Parser;

// Procedural Macro used to implement the Parse trait based on a Parsing Expression Grammars (.peg) files
// It also generates a enum for the Rules in the .peg file
use pest_derive::Parser;

/// This struct implements [pest::Parser] based on the grammar from grammar/csv.peg
#[derive(Parser)]
#[grammar = "grammar/csv.peg"]
pub struct CsvParser;

/// computes the sum of these fields and counts the number of records.
#[test]
fn sum_fields_and_count_records() {
    const CSV_DATA: &str = include_str!("../data/data.csv");

    let file = CsvParser::parse(Rule::File, CSV_DATA).expect("data.csv is formatted correctly");

    let mut sum = 0.0;
    let mut record_count = 0;

    for record in file {
        match record.as_rule() {
            Rule::Record => {
                record_count += 1;

                for field in record.into_inner() {
                    sum += field
                        .as_str()
                        .parse::<f64>()
                        .expect("A field consists of ASCII_DIGITs");
                }
            }
            Rule::EOI => break,
            Rule::Field => unreachable!("A file consists of records"),
            Rule::File => unreachable!("A file consists of records"),
        }
    }

    assert_eq!(sum, 2643429302.327908);
    assert_eq!(record_count, 5);
}

#[test]
fn parse_one_value() {
    dbg!(CsvParser::parse(Rule::Field, "-273.15").unwrap());
    dbg!(CsvParser::parse(Rule::Field, "this is not a number").unwrap_err());
}
