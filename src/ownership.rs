fn main()
{
    // gives ownership to s1
    let s1 = give_ownership();
    // s2 comes to scope      
    let s2 = String::from("Hello");
    // s2 moved into function which moves value to s3
    let s3 = takes_and_gives_back(s2);
}
fn give_ownership() -> String 
{
    let some_string = String::from("Hello");
    // move reutrn value to function (return)
    some_string
}
fn takes_and_gives_back(a_string: String) -> String
{   
    a_string
}

