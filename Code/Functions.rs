#[allow(unused_variables)]
fn main() {
    let (age, year) = (21, 2023);
    println!("{}", age);
    assert_eq!(true,check_dates(21));
    println!("Sucess");
}

fn check_dates(age: i32) -> bool {
    age > 18
}
