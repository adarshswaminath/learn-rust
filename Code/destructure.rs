#[allow(unused_variables)]
fn main(){
    // let (x,y) insted of let x; let y;
    let (mut age,mut year) = (20,2020);
    println!("Age {} and year {}",age,year);
    age += 10;
    year += 10;
    println!("Age {} and year {}",age,year);
    println!("-----------------------------");
    let (x,y): (i32,i32);
    (x,..) = (21,22);
    println!("x is {}",x);
    let (a,b);
    (a,..) = (3,4);
    [..,b] = [5,6];
    assert_eq!([a,b],[3,6]);
    println!("Test Passed!");
}