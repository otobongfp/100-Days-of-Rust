use std::io;

pub fn fibonacci() {
   println!("To end the program, type `exit` ");
   loop {
       println!("Type a positive integer");
       let mut int = String::new();
        io::stdin()
       .read_line(&mut int)
       .expect("");
       if int.trim() == "exit"{
           break;
       }
       let int: u32= match int.trim()
       .parse() {
           Ok(int) => int,
           Err(_) => continue,
       };
       println!("Fibonacci ({}) => {}", int, fib(int));

   }

}

fn fib(_n : u32) -> u32{
    if _n <= 0{
        return 0;
    }else if _n == 1{
        return 1;
    }fib(_n-2) + fib(_n-1)
}