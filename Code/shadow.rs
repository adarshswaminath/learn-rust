fn main(){
    let msg = "hi Iam Rust";
    {
        let msg = "Iam Learning";
        assert_eq!(msg,"Iam Learning");
    }
    assert_eq!(msg,"hi Iam Rust");
    
    let msg = "Rust is awesome";
    assert_eq!(msg,"Rust is awesome");
    println!("The current msg is {}",msg)


}