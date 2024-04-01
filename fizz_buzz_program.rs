fn fizz_buzz() {

  let mut number = 1;
  let mut fizz_buzz_count = 0;

    while number <= 301 { 
        if number % 15 == 0 {
            println!("FizzBuzz");
            fizz_buzz_count += 1;
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", number);
        }
        number += 1; 
    }
    println!("FizzBuzz occured {} times",fizz_buzz_count);
}
fn main() {
    fizz_buzz();
}
