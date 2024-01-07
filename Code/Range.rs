// create a program to print multiplication table
fn main(){
    // it represnt the range 1..10  (1,9)
    // 1..=10 equal sign will include 10 also (1,10)
    let value:i32 = 2;
    for i in 1..=10 {
        println!("{} * {} = {}",i,value,i*value);
    }
    // print alphabets
    for c in 'a'..='z' {
        // will print it as ascii code
        println!("{}",c as u8);
    }
}
