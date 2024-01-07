use std::mem::size_of_val;

// no issue if the function is not called
fn _msg_maker(name:&str){
    println!("Hello {} You are Rust developer",name)
}

fn main(){
    let ch: char = 'r';
    println!("Size of {} is {}",ch,size_of_val(&ch));
    // msg_maker("Cargo");
}