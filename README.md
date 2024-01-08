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

## Numbers - Integer Types
- **Signed integers:** Can represent both **positive** and **negative** integers

- **Unsigned integer:** Always **positive** intege

        Length      Signed      Unsiged

        8-bit         i8       u8
        16-bit        i16      u16
        32-bit        i32      u32
        64-bit        i64      u64
        128-bit       i128     i128
        arch          isize    usize

        arch -> Architecture
**default types**
integers: **i32**
Floats: **f64**

- Smallest possible 8-bit integer (unsigned): **0**
- Largest possible 8-bit inetger (unsigned): **255**
- Smallest possible 16-bit integer (unsigned): **0**
- Largest possible 16-bit inetger (unsigned): **65535**

### usize & isize
- Architecture dependent
- On 32-bit architecture: **32-bit**
- On 64-bit architecture: **64-bit**
- Pointer sized integer type,matches size of a **word** in given platform

### what is a word ?
- Processor does NOT read 1 byte at a time from memory, reads 1 word at a time
- In a **32-bit** processor it can access 4 bytes(32 bits) at a time
- In a **64-bit** processor it can access 8 bytes (64 bits) at a time

## Floating Point
- f32 - size of 32bits
- f64 - size of 64 bits
- Representation according to IEEE-754 specification

## Boolean Logic
- Boolean logic deals with values are either "true" or "false" (or 1 and 0)
- Three basic operations: AND,OR,NOT 
## Bitwise Operations
- Operations that manipulate individual bits that make up a binary number
- Treating each bit of a binary number as a seperate unit and perform logical operations on them
- AND,OR,XOR bitwise shifting

## Char,Bool,Unit

## Char
Single character of size 4 bytes
## Bool
Boolean value of true or false of size 1 byte
## uint
Empty tuple of size 0 bytes,used to return "nothing" in expression or function
 
## Statement:
- Instruction that perform some action but do not produce a value
- Function definition are statements as well as code that ends with ";" (usually)
## Expression:
- Evaluate to resultant value

## Functions:
- Block of reusable code that performa a specific tasks
- Can take arguments,Process those input and returns a result
## Diverging function: 
- Never return to the caller
- E.g. panic,looping forever,quitting the program

panic macro will immeditely exit and return error
```rust
panic!()
```
macros used to not return function to the called
```rust
todo!();
unimplemented!();
```
## Ownership
- Rust's ownership system is unique and set it apart from other programming languages
- Set of rules that govern memory management
- Rules are enforces at compile time
- If any of the rules are violated the program won't compile 

## Three Rules of Ownership
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. when the owner goes out of scope the value will be dropped

**Owner**: The owner of a value is the variable or data structure that holds it and is respsonsible for allocationg and freeing the memory used to store that data

## Scope
- Range within a program for which an item is valid
- **Global Scope**:
- Accessible through the entire program 
- **Local Scope**:
- Accessible only within particular function or block of code
- Not accessible outside of that function block

```rust
{                       // s is not valid here,its not yet declared
        let s = "hello" //s is valid from this point forward
        // do stuff with s
} 
// this scope is now over,and s is no longer valid
```
- When **s** comes into scope,is is valid
- It remains valid until it goes out of scope.
- **General rule**: Scope ends wher block of code ends (culty brackets)

**1:35:05**