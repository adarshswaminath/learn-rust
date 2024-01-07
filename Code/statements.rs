fn main(){
    let x: u32 = 5u32;
    let y: u32 = {
        let x_squared = x * x;
        // this expression will assigned to y
        3 + x_squared + x
    };
    let z: () = {
        // This semicolon suppresses this expression and '()' is assigned to 'z'
        let _ =2 * x;
    };
    println!("x is {:?}",x);
    println!("y is {:?}",y);
    println!("z is {:?}",z);
}