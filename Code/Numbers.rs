fn main(){
    let num: i32 = 10;
    let num1 = num;
    let res = num1 + num;
    println!("{}",res);
    let count: u16 = 38_u8 as u16;
    println!("{}",count);
    println!("Maximum of u16 {}",u16::MAX);
    println!("{}",0o77);
    // decimal,hexadecimal,octant,binary
    // 1_024,0xff,0o77,0b1111_1111
    // reduce floating size
    // assert!(0.1_+0.2 == 0.3); it make err becuase res is 0.30000000000002
    assert!(0.1_f32+0.2_f32 == 0.3_f32);
    // or
    assert!(0.1 as f32+0.2 as f32 == 0.3 as f32);

} 