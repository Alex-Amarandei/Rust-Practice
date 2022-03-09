fn main() {
    let s = String::new();
    println!("{}", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    // the method also works on a literal directly:
    let s = "final contents".to_string();
    println!("{}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let s1 = String::from("foo");
    let mut s2 = String::from("bar");
    let s3 = s1 + &s2;
    println!("{}", s3);

    s2.push_str("ouch");
    println!("{}", s3);

    let s = format!("{}-{}-{}", s2, s2, s3);
    println!("{}", s);
}
