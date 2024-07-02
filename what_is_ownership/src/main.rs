fn main() {
    let s = String::from("Book");

    // Add code here that calls the pluralize function
    let ss = pluralize(&s);
    println!(
        "I have one {}, you have two {}",
        s,
        ss,
    );
}

// Add appropiate paramrters, return values, and implementation to this function 
fn pluralize( s: &String) -> String {
    let mut ss = String::from(&s[..]);
    ss.push_str("s");
    ss
}