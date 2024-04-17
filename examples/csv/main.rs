use pest_book::get_input;

// Trait to facilitate parsing text of a known grammar
use pest::Parser;

// Procedural Macro used to implement the Parse trait based on a Parsing Expression Grammars (.peg) files
use pest_derive::Parser;

/// This struct implements [pest::Parser] based on the grammar from /examples/csv/csv.peg.
#[derive(Parser)]
#[grammar = "examples/csv/csv.peg"]
pub struct CsvParser;

/// computes the sum of these fields and counts the number of records.
fn main() {
    const CSV_DATA: &str = include_str!("data.csv");

    let file = CsvParser::parse(Rule::File, CSV_DATA).expect("data.csv is formatted correctly");

    let mut sum = 0.0;
    let mut record_count = 0;

    for record in file {
        match record.as_rule() {
            Rule::Record => {
                record_count += 1;

                for field in record.into_inner() {
                    sum += field.as_str().parse::<f64>().expect("A field consists of ASCII_DIGITs");
                }
            },
            Rule::EOI => break,
            Rule::Field => unreachable!("A file consists of records"),
            Rule::File => unreachable!("A file consists of records"),
        }
    }

    println!("")
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_one_value() {
        dbg!(CsvParser::parse(Rule::Field, "-273.15").unwrap());
        dbg!(CsvParser::parse(Rule::Field, "this is not a number").unwrap_err());
    }
    
    #[test]
    fn sum_fields() {
        let file = CsvParser::parse(Rule::File, CSV_DATA)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(CsvParser::sum_fields(file), 2643429302.327908);
    }
    
    #[test]
    fn count_records() {
        let file = CsvParser::parse(Rule::File, CSV_DATA)
            .unwrap()
            .next()
            .unwrap();
        assert_eq!(CsvParser::count_records(file), 5);
    }
}