
use std::io;
fn factorial(int:i128)
{
    println!("Enter a number to generate factorial: ");
    let mut int = String ::new();
 io::stdin().read_line(& mut int).expect("Failed to read line");
      let int :i128 = int.trim().parse().unwrap();
      let mut _factorial=1;
      if int<0
      {
          println!("Error! cannot generate factorial because entered number is less than zero");
      }
      else
      {
          for mut _x in 1..int
          {
              _x=_x+1;
              _factorial =_x*_factorial;
             
          }
           println!("Factorial of {}: {}",int,_factorial);
      }




}
