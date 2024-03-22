fn main() {
    println!("{}", likes(&[]));
    println!("{}", likes(&["Peter"]));
    println!("{}", likes(&["Peter", "Jacob"]));
    println!("{}", likes(&["Peter", "Jacob", "bruh"]));
    println!("{}", likes(&["Peter", "Jacob", "bruh", "yay"]));
}
