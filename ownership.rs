// Ownership, Borrowing and Regerences

// Ownership
// --------
// C, C++ -> Memory Managment Control Issue
// Garbage Collector solved this issue, 
// but created a new issue -> Slow Performance:
// [stopping/Resumig the program]


// Ownership
// [stopping/Resuming the program]
// Ownershop introduced by Rust to solve memory safey
// issues and high performanceat the same time.
// What is Ownership?
// Every value has a single owner [every variable has one value, and it is its sole owner].

// Ownership Rules
//_________________
// 1- Each value in Rust has a variable that's its owner.
// 2- There can be only one owner at a time.
// 3- When the owner goes out of scope, the value will be dropped.

// Example: Each value in Rust has a variable that's its owner
fn main(){
    let s1 = String::from("RUST");
    // rule 2
    //let s2 = s1;
    // println!("{}",s1);
    //println!("{}",s2);
    // len len = calculate_lenght(&s1);
    // println!("Lenght of '{}' is {}.", s1, len);

    let len = calculate_lenght(&s1);
    println!("Lenght of '{}' is {}.", s1, len);
}

fn printLost(s1: &string){
    println!("{}", &s1);
}

fn calculate_lenght(s: &String) -> usize{
    s.len()
}

