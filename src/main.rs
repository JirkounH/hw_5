use comfy_table::{Table, Row, Cell};
use csv::ReaderBuilder;
use slug::slugify;
use std::env;
use std::error::Error;
use std::io::{self, Read};

/// Type alias for operation functions
pub type OpsFn = fn(&str) -> Result<String, Box<dyn Error>>;

/// Runs the main processing logic of the program
fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: <program> <operation>");
        eprintln!(
            "Available operations: lowercase, uppercase, trimspaces, slugify, reverse, novowels, csv"
        );
        return Err("Missing operation".into());
    }
    
    let operation = args[1].as_str();
    eprintln!("Selected operation: {operation}");
    let input = read_input()?;
    
    let operations = list();
    match operations.iter().find(|(name, _)| name == &operation) {
        Some((_name, handler)) => match handler(&input) {
            Ok(result) => println!("\nResult:\n{result}\n"),
            Err(e) => eprintln!("Error executing operation: {e}"),
        },
        None => eprintln!("Unknown operation: {operation}"),
    }
    Ok(())
}

/// Reads input from stdin
fn read_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    Ok(input.trim().to_string())
}

/// Converts input to lowercase
fn lowercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_lowercase())
}

/// Converts input to uppercase
fn uppercase(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.to_uppercase())
}

/// Removes all spaces from input
fn trimspaces(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(" ", ""))
}

/// Reverses the characters in the input string
fn reverse(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.chars().rev().collect())
}

/// Removes all vowels from the input string
fn novowels(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(input
        .chars()
        .filter(|c| !"aeiouAEIOU".contains(*c))
        .collect())
}

/// Processes input as CSV and formats it into a table
fn csv_operation(input: &str) -> Result<String, Box<dyn Error>> {
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(input.as_bytes());
    let headers = csv_reader.headers()?.clone();
    
    let mut table = Table::new();
    table.set_header(headers.iter().map(|h| Cell::new(h)).collect::<Vec<_>>());
    
    for result in csv_reader.records() {
        match result {
            Ok(record) => {
                let mut row = Row::new();
                for field in record.iter() {
                    row.add_cell(Cell::new(field));
                }
                table.add_row(row);
            }
            Err(e) => {
                eprintln!("Error reading CSV: {e}");
                return Err(e.into());
            }
        }
    }
    Ok(format!("{}", table))
}

/// Returns a list of available operations
fn list() -> [(&'static str, OpsFn); 7] {
    [
        ("lowercase", lowercase),
        ("uppercase", uppercase),
        ("trimspaces", trimspaces),
        ("slugify", |input| Ok(slugify(input))),
        ("reverse", reverse),
        ("novowels", novowels),
        ("csv", csv_operation),
    ]
}

/// Entry point of the program
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
