fn main(){
    let _ager: () = ();
    let ager: (i32,i32) = (21,12);
    assert_eq!(_ager,iam_unit_fn());
    println!("Passed")
}
fn iam_unit_fn(){
    println!("Iam Trapped");
}