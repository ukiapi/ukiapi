fn main() {
    let mut s = String::from("Custom details of internal server error that is very long");
    s.clear();
    s.push_str("Internal Server Error");
    println!("{}", s);
}
