fn main() {
    let s = String::from("book");
    // Add code here that calls the pluralize function
    let ss = pluralize(&s);
    println!(
        "I have one {}, you have two {}",
        s,
        ss // you add something here
    );

    println!(
        "Now done even more idiomatically...
        I have one {}, you have two {}",
        s,
        pluralize_idiomatic_plus(&s)
    )
}

//Add approprieate parameters, return values, and implementation to this function
fn pluralize(s: &String) -> String {
    let mut ss = String::from(s);
    ss.push_str("s");
    ss
}

fn pluralize_idiomatic_plus(s: &str) -> String {
    // create owned data from borrowed (usually implements a clone)
    s.to_owned() + "s"
}
