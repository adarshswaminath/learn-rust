# Learn Rust Programming

## Variables
- **Assigned** using ```let``` keyword
- **Print** to standard output ```print!()``` or ```println!()```
- **Scope** of the variable defined by the block of code in which it is declared
- **Function** is named block of code that is **reusable**
- **Shadowing** allows a variable to be **re-declared** in the **same scope** with the **same name**
- use ```mut``` to make a variable as mutable
- Uninitialized variable cannot be used
- Avoid warning in unused variable use ```_``` staring of the variable ```let _x=1;```

### Shadowing
Declare a new variable with the same name as a previous variable
### Destructuring
```rust
let (mut x,y) = (1,2);
x+=2;
assert_eq!(x,3);
```
Indroduced In Rust 1.59: You can use tuple,slice, and struct patterns 