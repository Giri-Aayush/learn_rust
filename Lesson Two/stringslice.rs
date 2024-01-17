fn main (){
    let s1 = String::from("hello");
    
    // two ways to convert a String to &str (string slice)
    let s2 : &str = s1.as_str();
    let s3 : &str = &s1;

    // two ways to convert a &str (string slice) to String
    let s4 = s2.to_string();
    let s5 = String::from(s3);
    println!("Success");
}