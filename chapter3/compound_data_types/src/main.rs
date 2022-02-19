fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, z) = tup;

    println!("x: {}, y: {}, z: {}", tup.0, y, z);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("8th month: {}", months[7]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[3]: {}", a[3]);

    let a = [3; 5]; // a will be [3, 3, 3, 3, 3]
    println!("a[2]: {}", a[2]);
}
