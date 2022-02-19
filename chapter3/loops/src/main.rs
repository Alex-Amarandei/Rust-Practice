fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    break_return();

    for_example();
}

fn break_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// while is the same as usual and it is written just as an if

fn for_example() {
    let a = [1, 2, 3];

    for val in a {
        println!("{}", val);
    }

    for val in 1..3 {
        println!("{}", val);
    }

    for val in (1..3).rev() {
        println!("{}", val);
    }
}
