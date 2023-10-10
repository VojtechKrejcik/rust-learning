fn part1() {
    println!("Hello, world!");
    let number = 3;

    let result = if number == 3 { "pokus 2" } else { "pokus 2" };

    println!("{}", result);

    match result {
        pokus2 @ "pokus 2" => println!("As expected {pokus2}"),
        _ => todo!(),
    };
}

fn part2() {
    let mut count = 0;

    let result = loop {
        if count > 5 {
            break count;
        };
        count += 1;
    };
    println!("{}", result);
    for i in 0..count {
        println!("{}", i);
    }

    for i in 0..=count {
        println!("{}", i);
    }
}

fn part3() {
    let fruits = vec!["apple", "banana"];
    for fruit in &fruits {
        println!("{}", fruit);
        // Rust is automatically dereferncing
        println!("{}", *fruit);
    }
}
fn part4(a: i32, b: i32) -> i32 {
    return a + b;
}
fn takeOwnership(data: String) {
    println!("{}", data);
}

fn dontTakeOwnersship(data: &String) {
    println!("{}", data);

    // or I can dereference manually - println!("{}", *data);
}

mod outerModule {
    pub mod innerModule {
        pub fn mu_module_fn() {
            println!("my module");
        }
    }
}
fn main() {
    let my_data = String::from("hello");
    takeOwnership(my_data); // my_data will be freed inside func

    let my_data = String::from("hello");
    takeOwnership(my_data.clone()); //now i pass only copy, so i can still use it later, but takes double the memory
    println!("{}", my_data);
    dontTakeOwnersship(&my_data);
    part4(1, 2);

    outerModule::innerModule::mu_module_fn();
    use outerModule::innerModule::mu_module_fn;
    mu_module_fn();
}
