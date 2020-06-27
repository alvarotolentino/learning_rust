fn main() {
    println!("Hello, world!");
    loopto10_loop();
    loopto10_for();
    loopto10_while();
    array_loopto10();
}

fn loopto10_loop() {
    let mut n = 0;
    loop {
        n += 1;
        println!("Hello {}", n);
        if n >= 10 {
            return;
        }
    }
}

fn loopto10_for() {
    for n in 0..10 {
        println!("Hello {}", n);
    }
}

fn loopto10_while() {
    let mut n = 0;
    n = n + 1;
    while n < 10 {
        println!("Hello {}", n);
    }
}

fn array_loopto10() {
    let v = vec![5,7,8,9,11,10];

    'outer: for _ in 0..10 {
        for n in &v {
            if n%2 == 0 {
                break 'outer;
            }
            println!("{}", n);
        }
    }
}