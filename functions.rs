// an function / variable should be written in snake-case
fn main(){
    hello_world();
    tell_height(182);
    human_id("joel", 55, 182.2);
    
    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };

    println!("Result is:{}", x);
    
    let y:i32 = add(4, 6);
    println!("Value of y is :{}", y);
    println!("Value from function 'add' is :{}.", add(4, 6));

    let weight = 70.0;
    let height = 1.82;
    let bmi = calculate_bmt(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}



// Hoisiting
fn hello_world(){
    println!("Hello, Rust!");
}


fn tell_height(height: u32){
    println!("My height is {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm.", name, age, height);
}

fn add(a: i32, b:i32)->i32{
    a+b
}

// Expression and Statement
// Expression: Anything that returns a value.
// Expression
// ---
// 5
// true & false
// add(3, 4)
// if condition {value1} else {value2}
// ({code})


//  Statement
// Statement: Anything that does not return a value.
// Almost all statements in Tusst ends with ;
//ley y = let x = 10;
// 1 Variable declaration: let x = 5;
// 2 Function definations: fn foo() {}
// 3 Control flow statements: if condition { /* code */ } else { /* code */ }, while condition { /* code */ }, etc.

//BMI = weight_kg(kg)/height(m^2)

fn calculate_bmt(weight_kg: f64, height_m: f64)->f64{
    weight_kg/(height_m * height_m)
}