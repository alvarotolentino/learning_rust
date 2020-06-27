fn main() {
    let mut s = String::from("Hello 世界");
    s.push_str("!");

    println!("bytes: {}", s.len());

    for c in s.chars() {
        println!("{}", c);
    }

    // number of characters
    for (i, c) in s.chars().enumerate() {
        println!("{} = {}", i, c);
    }

    println!("number of l = {}", count_l(&s));

    // byte position
    for (i, c) in s.char_indices() {
        println!("{} = {}", i, c);
    }

    for c in s.bytes() {
        println!("{}", c);
    }
}

fn count_l(s: &str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res += 1;
        }
    }
    res
}
