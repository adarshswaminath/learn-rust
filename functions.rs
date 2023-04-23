// learned functions statement and expressions
use std::io;
fn main() {
    let dash = "-".repeat(15);
    let mut num1 = String::new();
    let mut num2 = String::new();
    println!("Enter The number One: ");
    io::stdin().read_line(&mut num1).expect("InputFailed");
    let num1: u32 = num1.trim().parse().expect("Invalid Convertion");
    println!("Enter The Second Number: ");
    io::stdin().read_line(&mut num2).expect("Input Failed");
    let num2: u32 = num2.trim().parse().expect("Invalid convertion");
    let add = do_add(num1,num2);
    let mul = do_mul(num1,num2);
    let div = do_div(num1,num2);
    let sub = do_sub(num1,num2);
    println!("{}",&dash[..]);
    println!("{num1} + {num2} = {add}");
    println!("{num1} * {num2} = {mul}");
    println!("{num1} / {num2} = {div}");
    println!("{num1} - {num2} = {sub}");
    println!("{}",&dash[..]);


}
fn do_add(x: u32,y:u32) -> u32 {
    x+y
}
fn do_mul(x: u32,y:u32) -> u32 {
    x*y
}
fn do_div(x: u32,y:u32) -> u32 {
    x/y
}
fn do_sub(x: u32,y:u32) -> u32 {
    x-y
}
