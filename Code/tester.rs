// #[allow(unused_parens)]
// fn main() {
//     let v:i32 = {
//         let x = 3;
//         x
//     };
//     assert!(v == 3);
// }

fn main(){
    let s = sum(1,3);
    assert_eq!(s,4);
    println!("Success");
}

fn sum(x: i32,y:i32) -> i32 {
    x + y // If ; added it means function does not return any value unit  
}