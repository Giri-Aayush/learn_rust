use std::mem::size_of_val;

fn main(){
    let x = 'a';
    println!("{} bytes for char",size_of_val(&x));

    let w1: u32 = 1;
    println!("{} bytes for u32",size_of_val(&w1));

    let w: u8 = 1;
    println!("{} bytes for u8",size_of_val(&w));

    let y = 1;
    println!("{} bytes for i32",size_of_val(&y));

    let z = 1.0;
    println!("{} bytes for f64",size_of_val(&z));

    let a:f32 = 1.0;
    println!("{} bytes for f32",size_of_val(&a));

    let b = true;
    println!("{} bytes for bool",size_of_val(&b));
}
/*
4 bytes for char
4 bytes for u32
1 bytes for u8
4 bytes for i32
8 bytes for f64
4 bytes for f32
1 bytes for bool
*/