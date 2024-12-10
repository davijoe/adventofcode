fn main() {
    hello();
    tell_height(175);
    human_id("Dan", 31, 175.5);
    let _x = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Result is: {}", _x);
    let y: i32 = add(25, 17);
    println!("Value of y is: {}", y);

    //calling bmi function
    let weight_kg = 72.5;
    let height = 1.82;
    let bmi = calculate_bmi(weight_kg, height);
    println!("Your BMI is: {:.2}", bmi);
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

// Functions returning values (no semicolon after b)
fn add(a: i32, b: i32) -> i32 {
    a + b
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

// BMI example
fn calculate_bmi(weight_kg: f64, height: f64) -> f64 {
    weight_kg / (height * height)
}
