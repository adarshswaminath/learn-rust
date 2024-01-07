fn main(){
    let mut age = 2016;
    println!("Before increment {}",age);
    age += 1;
    assert_eq!(age,2017);
    println!("After increment {}",age);
}