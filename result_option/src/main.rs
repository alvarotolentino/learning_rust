use std::collections::HashMap;
use std::env::args;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(3, "Hello");
    hm.insert(5, "world");

    // using match
    // let r  = match hm.get(&3){
    //     Some(v) => v,
    //     _=> "NOTHING",
    // };

    // using unwrap
    let r = hm.get(&4).unwrap_or(&"NoString");

    println!("{}", r);

    // using match sentence
    // match "3t".parse::<f32>() {
    //     Ok(v) => println!("OK - {}", v),
    //     Err(e) => println!("Error - {}", e)
    // }

    match get_arg(3) {
        Ok(v) => println!("OK - {}", v),
        Err(e) => println!("Error - {}", e)
    }
}

fn get_arg(n:usize) -> Result<String, String> {
    for (i,a) in args().enumerate() {
        if i == n {
            return Ok(a)
        }
    }
    Err("Not enough arguments".to_string())

}