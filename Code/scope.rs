fn main() {
    // main scope || outer scope
    let x: i32 = 10;
    let y: i32 = 5;
    // inner scope
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and the value of y is {}", x, y);
    callme();
}

fn callme(){
    let name = "Rust";
    println!("Iam {}",name);
}