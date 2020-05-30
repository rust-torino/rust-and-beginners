use std::env;

fn present(name: &str) -> String {
    format!("I am {}", name)
}

fn hello_world(message: &str, name: &str) -> String {
    let present_message = present(name);
    format!("Hello world! {} {}", message, present_message)
}

fn main() {
    let name = env::args().nth(1).unwrap();
    let yell = "Oh yeah!";
    let message = hello_world(yell, &name);
    println!("{}", message);
}
