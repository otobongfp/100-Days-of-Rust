use crate::farenheit_conv::conv;
use crate::fibonacci::fibonacci;
use crate::countdown::counter;

pub mod farenheit_conv;
pub mod fibonacci;
pub mod countdown;


fn main() {
    println!("The Goal of this repo is to push myself to consistently write Rust daily for 100days");
    //conv();
    //fibonacci();
    counter();

}