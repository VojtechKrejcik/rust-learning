use::slug::slugify;
use std::io;
use std::env;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("Please specify one of the following operations as argument:
    -lowercase
    -uppercase
    -no-spaces
    -slugify");
    return Ok(());
    };
    let mut user_input = String::new();
    let stdin = io::stdin(); 
    let _ = stdin.read_line(&mut user_input);

    match args[1].as_str(){
        "lowercase" => println!("{}",user_input.to_lowercase()),
        "uppercase" => println!("{}",user_input.to_uppercase()),
        "no-spaces" => println!("{}",user_input.replace(" ", "")),
        "slugify" => println!("{}",slugify(user_input)),
        _ => println!("Unsupported operation")

   }


 
    Ok(())
}
