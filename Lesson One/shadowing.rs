fn main(){
    let x:i32 = 11;
    assert_eq!(11,x);
    println!("passed step one");
    println!("The value of x here is {}",x);
    {
        /*
        when we are shadowing, we can even change the datatype 
        of the variable. but this will be accessible only in this scope
         */
        let x:u32 = 16;
        assert_eq!(x,16);
        println!("passed step two");
        println!("The value of x here is {}",x);
    }
    assert_eq!(x,11);
    println!("passed step three");
    println!("The value of x here is {}",x);
}