fn main() {
    

    let number_1:i32=109;
    let mut number_1_string:String=number_1.to_string();
    number_1_string.push_str("Take inputs in the terminal");
    println!("Hey {number_1_string}");
   
   
    
   //panic 
   let input_1:&str="5";
   let input_number :i32=input_1.parse().expect("was expecting an integer" );
   println!("Hey {input_number}");
   
   
   let array = [5,10,15,20];
   for count in array{
       if count <10 {
           println!("less than 10");
       }
       if count >20 {
           println!("greater than 20");
       }
   }
   
   
   
   
   
   
   









}
