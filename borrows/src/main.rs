fn main() {
    let mut list = vec![1, 2, 3];

    let first_element = list.first();

    println!("The first element is {:?}", first_element);
 
    *list.first_mut().expect("list was empty") += 1;

    println!("The first element after adding one is {:?}", list.first() );
}
