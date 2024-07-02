pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn pluralize(s: &String) -> String {
    let mut ss = String::from(s);
    ss.push_str("s");
    ss
}

pub fn pluralize_idiomatic_plus(s: &str) -> String {
    // create owned data from borrowed (usually implements a clone)
    s.to_owned() + "s"
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn pluralize_adds_s_at_the_end() {
        let s = String::from("book");
        let result = pluralize(&s);
        assert_eq!(result, String::from("books"))
    }

    #[test]
    fn pluralize_idiomatic_plus_adds_s_at_the_end() {
        let s = String::from("book");
        let result = pluralize_idiomatic_plus(&s);
        assert_eq!(result, String::from("books"));
    }

}
