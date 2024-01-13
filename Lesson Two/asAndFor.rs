fn main() {
    //the way in which we can declare two variables together
    let (mut sum1, mut sum2) = (0,0);
    
    //for loop where the last index is not included
    for i in 1..11 {
        sum1 += i;
    }
    println!("The sum from the loop is {}", sum1);
    println!("==========================================x=x=x=x=x=x=x=x=x=x=x=x=x=x=x==x=x=x=x=x=x=x=x==x=x=x=x=x=x==x=x==x======================================");
    
    //for loop where the last index in included
    for i in 1..=10 {
        sum2 += i;
    }
    println!("The sum from the loop is {}", sum2);
    println!("==========================================x=x=x=x=x=x=x=x=x=x=x=x=x=x=x==x=x=x=x=x=x=x=x==x=x=x=x=x=x==x=x==x======================================");
    
    //for loop where we can see an explicit type conversion of a char to u8
    for c in 'a'..='z' {
        println!("The ascii value of {} is {}", c, c as u8);
    }
    println!("==========================================x=x=x=x=x=x=x=x=x=x=x=x=x=x=x==x=x=x=x=x=x=x=x==x=x=x=x=x=x==x=x==x======================================");
}
