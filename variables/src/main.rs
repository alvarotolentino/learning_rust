fn main() {
    // mutable variable by using mut keyword
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing variable
    let y = 5;
    let y = y + 1;
    let y = y + 1;
    println!("The value of y is: {}", y);

    // shadowing with different types
    let spaces = "    ";
    let spaces = spaces.len();
    println!("How many whitespaces has the spaces variable: {}", spaces);
}
