  fn main() { 
      let qtoshiba:f64 = 2.0; 
      let atoshiba:f64 = 450_000.0;
      let qmac:f64 = 1.0;
      let amac = 1_500_000.0; 
      let qhp = 3.0; 
      let ahp = 750_000.0;
      let qdell = 3.0; 
      let adell = 2_850_000.0; 
      let qacer = 1.0;
      let aacer = 250_000.0; 

      //sum  
      let s = (qtoshiba * atoshiba) + (qmac * amac) + (qhp * ahp) + (qdell * adell) + (qacer * aacer); 
      println!("Sum is {}", s); 
      let a = s / 5.0; 
      println!("Average is {}", a ); 

      }  
