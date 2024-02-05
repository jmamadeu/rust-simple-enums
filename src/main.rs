#[derive(Debug)]
enum IPAddrKind {
    V4(String),
    V6(String),
}


fn main() {
    let four = IPAddrKind::V4(String::from("::1"));
    
    println!("{:#?}", four);
}

