use String;

fn main() {
    let s1 = String::from("hello world");

    print_references(&s1);

    println!("Still works: {}", s1);

    let s2 = String::from("hello world again");

    print_without_references(s2);

    // println!("Still works: {}", s2); this will fail because s2 was moved into the s
    // inside the function print_without_references
}

fn print_without_references(s: String) {
    println!("{}", s);
}

fn print_references(s: &String) {
    println!("{}", s);
}
