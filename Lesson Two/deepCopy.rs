fn main(){
    let s1 = String::from("Hello World");
    // let s2 = s1.clone();
    // println!("{} {} ", s1, s2);
    let s2 = takes_ownershop(s1);
    println!("{}", s2);

}

fn takes_ownershop(s:String) -> String {
    println!("{}",s);
    s
}