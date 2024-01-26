fn main(){

   const v:Vec<i32> = vec![20, 40, 60, 80]; 
   // vector v owns the object in heap 

   const v2:Vec<i32> = v; 
   const v2_return:Vec<i32> = display(v2); 
   print!("In main {:?}",v);  


fn display(v:Vec<i32>)->Vec<i32> { 
   // returningt the same vector 
   println!("inside display {:?}",v); 
   return v; 
} 

} 
