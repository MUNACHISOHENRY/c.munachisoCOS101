fn main() {
   let t:f64 = 450_000.0; 
   let m:f64 = 1_500_000.0;
   let h:f64 = 750_000.0;
   let d:f64 = 2_850_000.0;
   let a:f64 = 250_000.0;

   //SUM OF EACH AMOUNT GIVEN
   let tq:f64 = 2.0;
   let mq:f64 = 1.0;
   let hq:f64 = 3.0;
   let dq:f64 = 3.0;
   let aq:f64 = 1.0;
   
   //TOTAL SUM OF RECORD
   let total_quantity = t*tq + m*mq + h*hq +d*dq + a*aq;
   println!("The sum is {}", total_quantity );

   //AVERAGE OF THE SALES RECORD
   let sum_of_each_quantity = tq + mq + hq + dq + aq;
   println!("The overall quantity is {}", sum_of_each_quantity );
   let average = total_quantity / sum_of_each_quantity;
   println!(" The average is {}", average );
}


   

  
