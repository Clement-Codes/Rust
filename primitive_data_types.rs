// int, float, bool, char
// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: Unsigned integer
fn main(){
    // int
    // i8, i16, i32, i64, i128: Signed integers
    // u8, u16, u32, u64, u128: Unsigned integer
    let x:i32 = -42;
    let y:u64 = 100;
    println!("Signed Intger: {}", x); 
    println!("Unsigned Intger: {}", y); 

    // Floats [Floating point types]
    // f32 f62
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    let is_snowing: bool = true;
    println!("Is it snowing ? {}", is_snowing);

    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);


    // Compound Data Types
    // arrays, tuples, slices and strings(slice string)

    let number:[i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", number);
    // let mix = [1,2,"apple", true];
    // println!("Mix Array: {:?}", mix);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);
    
    // Tuples
    //let human = ("Alice", 30, false);
    let human:(String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);


    // Slices: [1,2,3,4,5]
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices:&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Animal Slice: {:?}", animal_slices);

    //Strings Vs String Slices (&str)
    //Strings [growable, mutable, owned string type]
    // heap slow
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String:: from("Hello, World");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);

}

// fn print(){
//     println!("Slice Value: {}", slice);
// }