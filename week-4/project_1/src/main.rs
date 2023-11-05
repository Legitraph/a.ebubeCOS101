  // Rust program to find the speed of a car 

  use std::io;

  fn main() {
    let mut distance = String::new();
    let mut time = String::new(); 

    println!("Enter distance travelled by the car: "); 
    let mut distance = String::new(); 
    io::stdin().read_line(&mut distance).expect("Not a valid string"); 
    let d:f32 = distance.trim().parse().expect("Not a valid number"); 

     println!("Enter time taken: "); 
     let mut time = String::new(); 
    io::stdin().read_line(&mut time).expect("Not a valid string"); 
    let t:f32 = time.trim().parse().expect("Not a valid number"); 

    let s:f32 = d / t; 
    let mut speed:f32 = d / t; 
    let final_ = speed * 1.609;

    // Convert miles to kilometers (1 mile = 1.609 kilometers)

    println!("speed of a car: {}", final_);  
}



