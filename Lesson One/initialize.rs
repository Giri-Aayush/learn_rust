fn main(){
    let x:i32 = 15;
    assert_eq!(15,x);
    //so it matches the data-type as well as the value
    println!("Success Step-1");
    assert_eq!(5,x,"we have panicked at this step" );
    println!("Success Step-2");
}

/*
two ways to use the assert_eq
--------------------------------------------------------------------
let a = 3;
let b = 1 + 2;
assert_eq!(a, b);
or
assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
--------------------------------------------------------------------
*/