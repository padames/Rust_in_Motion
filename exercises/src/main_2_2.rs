fn main() {
    let s = String::from("book");
    // Add code here that calls the pluralize function
    let ss = pluralize(&s);
    println!(
        "I have one {}, you have two {}",
        s,
        ss // you add something here
    );
}

//Add approprieate parameters, return values, and implementation to this function
fn pluralize(s: &String) ->String
{
    let mut ss = String::from(s);
    ss.push_str("s");
    ss
}