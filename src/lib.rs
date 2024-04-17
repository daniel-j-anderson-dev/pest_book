use std::{io::{stdin, stdout, Error as IoError, Write}, str::FromStr};

pub fn print(text: &str) -> Result<(), IoError> {
    let mut stdout = stdout();
    stdout.write_all(text.as_bytes())?;
    stdout.flush()?;
    return Ok(());
}

pub fn get_input(prompt: &str) -> Result<String, IoError> {
    print(prompt)?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    input.truncate(input.trim_end().len());

    return Ok(input);
}

pub fn get_parsed_input<T>(prompt: &str) -> Result<T, IoError>
where
    T: FromStr,
    T::Err: std::error::Error,
{
    loop {
        let input = get_input(prompt)?;
        match input.parse::<T>() {
            Ok(parsed_input) => return Ok(parsed_input),
            Err(parse_error) => print(format!("\nInvalid Input\n{}\n", parse_error).as_str())?,
        }
    }
}