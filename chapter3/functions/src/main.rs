fn main() {
    println!("Hello, world!");

    another_function();

    another_function_2(5, 'v');

    println!("Whaaaat? It works? \n{}", expression_function());
}

fn another_function() {
    println!("Another function.");
}

fn another_function_2(x: u32, c: char) {
    println!("Another function with two arguments: {}, {}", x, c);
}

fn expression_function() -> u32 {
    let y = {
        let x = 3;
        x + 1
    };

    y // without a semicolon because it is an expression not a statement
}
