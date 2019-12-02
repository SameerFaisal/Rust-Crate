
use std::io;
fn factorial(_int:i32)
{
    println!("Enter a number to generate factorial: ");
     let mut _int = String ::new();
 io::stdin().read_line(& mut _int).expect("Failed to read line");
      let _int :i128 = _int.trim().parse().unwrap();
      let mut _factorial=1;
      if _int<0
      {
          println!("Error! cannot generate factorial because entered number is less than zero");
      }
      else
      {
          for mut _x in 1.._int
          {
              _x=_x+1;
              _factorial =_x*_factorial;
             
          }
           println!("Factorial of {}: {}",_int,_factorial);
      }




}
