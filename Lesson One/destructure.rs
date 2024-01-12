fn main(){
    let (x,y);
    // this is a way to initialze two variables at once;
    (x,..) = (1,5);
    // this way we want x to only take the value 1 and we dont care about 5
    [..,y] = [11,15];
    // this way we want 15 in y and we dont care about 11
    assert_eq!([x,y],[1,15]);
    // this way we can compare two assert statements together
    println!("Success");
    //we can use both () and [] to destruture and assign values
}