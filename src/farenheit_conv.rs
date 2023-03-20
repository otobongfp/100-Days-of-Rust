use std::io;

pub fn farenheit_conv() {
    //Purpose of App
    println!("FARENHEIT TO CELSIUS CONVERTER");

    //Declare varible to collect F and C
    println!("Input the temperature value in Farenheit: ");
    let mut _f = String::new();

    //IO to read the inputs
    io::stdin().read_line(&mut _f).expect("Something went wrong");

    //Convert the string
    let _f : f32 = _f.trim().parse().expect("Are you sure you used a numerical value");

    //Manipulate the maths for conversion: C = 5/9(F-32)
    let _c : f32 = 5.0/9.0 * _f-32.0;
    println!("{_f} Fahrenheit = {_c} Censius");

}