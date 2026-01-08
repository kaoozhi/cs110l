fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    println!("{}", ref3.to_uppercase());
    s = String::from("goodbye");
    println!("{}", s.to_uppercase());
}

fn drip_drop() -> String {
    let s = String::from("hello world!");
    return s;
}

fn vector() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: &String = &v[0];
    println!("{}", s2);
}
