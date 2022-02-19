use String;

fn main() {
    {
        let mut x = 5;
        let y = x;

        println!("x: {}, y: {}", x, y);

        x = x + 1;

        println!("x: {}, y: {}", x, y);
    }

    {
        let s1 = String::from("5");
        let s2 = s1;

        // println!("s1: {}, s2: {}", s1, s2); this won't work because s1 no longer exists

        println!("s2: {}", s2);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }
}
