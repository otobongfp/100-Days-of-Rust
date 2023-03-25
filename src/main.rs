use crate::farenheit_conv::conv;
use crate::fibonacci::fibonacci;
use crate::countdown::counter;
use crate::generic_types::gen;
use crate::collections::collections;

pub mod farenheit_conv;
pub mod fibonacci;
pub mod countdown;
pub mod generic_types;
pub mod collections;


fn main() {
    print!("Excertps of codes you can play with");
    println!("The different functions are commented out you can uncomment them to praactice with");
    //conv();
    //fibonacci();
    //counter();
    collections();
    //gen();

}