use ::slug::slugify;
use std::env;
use std::error::Error;
use std::io;
use std::io::Read;

mod my_csv;
use my_csv::MyCsv;

fn lowercase() -> Result<String, Box<dyn Error>> {
    println!("Enter input:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    Ok(format!("{}", user_input.replace(" ", "")))
}

fn uppercase() -> Result<String, Box<dyn Error>> {
    println!("Enter input:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    Ok(format!("{}", user_input.replace(" ", "")))
}

fn no_spaces() -> Result<String, Box<dyn Error>> {
    println!("Enter input:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    Ok(format!("{}", user_input.replace(" ", "")))
}

fn my_slugify() -> Result<String, Box<dyn Error>> {
    println!("Enter input:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    Ok(format!("{}", slugify(user_input)))
}

fn csv() -> Result<String, Box<dyn Error>> {
    println!("Enter input (press control+d, when done):");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let parsed_csv = MyCsv::new(input);
    match parsed_csv {
        Ok(csv) => Ok(format!("{}", csv)),
        Err(e) => Err(e),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Please specify one of the following operations as argument:
    -lowercase
    -uppercase
    -no-spaces
    -slugify
    -csv"
        );
        return Ok(());
    }

    let result = match args[1].as_str() {
        "lowercase" => lowercase(),
        "uppercase" => uppercase(),
        "no-spaces" => no_spaces(),
        "slugif" => my_slugify(),
        "csv" => csv(),
        _ => Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is unsupported operation", args[1]),
        )) as Box<dyn std::error::Error>),
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
