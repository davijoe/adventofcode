fn main() {
    hello();
    tell_height(175);
    human_id("Dan", 31, 175.5);
}

fn hello() {
    println!("Hello, world! 🦀 ")
}

fn tell_height(height: u32) {
    println!("My height is {}", height)
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {} and I am {} years old and my height is {} cm",
        name, age, height
    );
}

// Expressions and statements
// Expression: Anything that returns a value
// statements: Anything that does not return a value

//Expression
//5
//true & false
//add(3,4)
//if condition {value1} else {value2}
//({code})
