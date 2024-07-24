use project::{helpers::hello_world::greeting, Language};

fn main() {
    println!("{}", greeting(Language::English));
    println!("{}", greeting(Language::Spanish));
    println!("{}", greeting(Language::Italian));
}