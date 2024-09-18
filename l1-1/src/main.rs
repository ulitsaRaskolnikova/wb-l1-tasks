struct Person {pub name : String}

trait Action {
    fn say(&self) {
        println!("I am saying.");
    }
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let person = Person{ name : String::from("Nick") };
    person.say();
}