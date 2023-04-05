use crate::farenheit_conv::conv;
use crate::fibonacci::fibonacci;
use crate::countdown::counter1;
use crate::generic_types::gen;
use crate::collections::collections;

pub mod farenheit_conv;
pub mod fibonacci;
pub mod countdown;
pub mod generic_types;
pub mod collections;


fn main() {

    println!("The different functions are commented out you can uncomment them to run the code");
    //conv();
    //fibonacci();
    //counter1();
    collections();
    //gen();
    //counter1();

}