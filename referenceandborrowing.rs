// References and Borrowing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understanding Regerences
// References: Enable you to borrow values without taking ownership.
// Immutable Reference.
// Mutabl Reference.
// Create Reference by add "&"

fn main(){
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x;
    *_r += 1;
    *_r -= 3;
    println!("Value of _x : {}", _x);
    //println!("Value of _r : {}", _r);

    // one mutable refferences is allowed
    // many immutable refference is allowed
    
}