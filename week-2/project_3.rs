fn main() { 
	    let s:f64 = 210_000.0;
	    let r:f64 = 5.0; 
	    let t:f64 = 3.0; 

	    //compound interest 
	    let a = s * (1.0- (r / 100.0)).powf(t); 
	    println!("The new price is {}", a)


}