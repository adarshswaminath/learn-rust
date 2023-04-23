use std::io;

fn main() {
    // create an array of strings representing the days of the week
    let month = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

    // print out each day of the week
    for i in month {
        println!("{}", i);
    }

    // prompt the user to enter their name
    let mut name = String::new();
    println!("Please Enter Your Name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Cannot accept the input");

    // trim the user's name to remove any whitespace and newline characters
    let name = name.trim();

    // prompt the user to enter a number between 0 and 6
    let mut index = String::new();
    println!("Please Enter a Number Between 0-6 ");
    io::stdin()
        .read_line(&mut index)
        .expect("Cannot accept the input");

    // parse the user's input as a usize, which represents an index in the month array
    let index: usize = index
        .trim()
        .parse()
        .expect("Invalid conversion: please enter a number between 0 and 6");

    // use string interpolation to print out the user's name and the day of the week they selected
    println!(
        "User {} 🥳 selected the {} as the Day 🎉",
        name, month[index]
    );
}
