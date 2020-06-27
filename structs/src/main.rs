#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    height: i32,
    admin: bool,
}

impl User {
    fn simple_string(&self) -> String {
        format!(
            "{} - {} - {}cm - admin:{}",
            self.name, self.age, self.height, self.admin
        )
    }

    fn grow(&mut self, h: i32) {
        self.height += h
    }

    fn die(self) {
        println!("Dead {}", self.simple_string());
    }
}

fn main() {
    let mut u = User {
        name: "James".to_string(),
        age: 38,
        height: 90,
        admin: true,
    };

    println!("User is {}", u.simple_string());
    u.grow(20);
    println!("User is {}", u.simple_string());
    u.die();
}
