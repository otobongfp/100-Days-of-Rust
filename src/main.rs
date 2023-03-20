use crate::farenheit_conv::farenheit_conv;
use crate::fibonacci::fibonacci;

pub mod farenheit_conv;
pub mod fibonacci;


fn main() {
    println!("The Goal of this repo is to push myself to consistently write Rust daily for 100days");
    farenheit_conv();
    fibonacci();
}