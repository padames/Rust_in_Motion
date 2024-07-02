pub(crate) use functions::{pluralize, pluralize_idiomatic_plus};

fn main() {


    println!("2.2 Borrowing principles:");

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

