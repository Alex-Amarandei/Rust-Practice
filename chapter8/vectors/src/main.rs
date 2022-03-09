fn main() {
    let _v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    for e in &v {
        // without the & it is a borrow
        println!("{}", e);
    }

    v.push(8);

    println!("{}", v[3]);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(6) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 50;
    }

    for e in &v {
        println!("{}", e);
    }
}
