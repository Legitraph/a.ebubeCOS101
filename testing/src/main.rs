use std::io;

fn main() { 
    println!("Enter your gpa"); 
    let mut gpa_string = String::new(); 
    io::stdin().read_line(&mut gpa_string).expect("Failed to read line"); 
    let gpa:f32 = gpa_string.trim().parse().expect("Please input your gpa");

    let eligibility = check_student_eligibility(gpa); 
    println!("GPA:{:.1}, eligibility:{}, gpa, eligibility"); 

    fn check_student_eligibility(gpa:f32) -> String 
     {if gpa < 2.5} { 
        "Not eligible to join student council".to_string()
             else "Eligible to join student council".to_string()
    }
}
