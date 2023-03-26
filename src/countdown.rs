 use std::io;
// use std::time::Duration;


//VERSION 1 OF A COUNTDOWN IN RUST

pub fn counter1(){
        //Collects inputs
        println!("Input a number to countdown");
        let mut value = String::new();
    
        //read input
        io::stdin().read_line(&mut value).expect("something went wrong");
    
        //converts to int
        let mut value:u32 = value.trim().parse().expect("Check wella");
        
        //loop
        loop{
            
            if value > 0{
                println!("{}", value);
            }else{
                break;
            }
            let base = &value - 1;
            value = base;
        };

}



//VERSION TWO OF A COUNTDOWN
// // SOURCE OF THE ANSI CHAR - https://www.w3.org/TR/xml-entity-names/025.html
// const  DIGITS : [[&str; 10]; 7] = [
//     ["┏━┓ "," ╻ "," ┏━┓ "," ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏━┓ "," ┏━┓ "," ┏━┓ "," ┏━┓ "],
//     ["┃ ┃ "," ┃ ","   ┃ ","   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "],
//     ["┃ ┃ "," ┃ ","   ┃ ","   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "],
//     ["┃ ┃ "," ┃ "," ┏━┛ "," ┣━┫ "," ┗━┫ "," ┗━┓ "," ┃━┓ ","   ┃ "," ┣━┫ "," ┗━┫ "],
//     ["┃ ┃ "," ┃ "," ┃   ","   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "],
//     ["┃ ┃ "," ┃ "," ┃   ","   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "],
//     ["┗━┛ "," ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ "],
// ];

// // pub fn counter(){
// //     println!("Input a number for countdown");

// //     //Get Number as String
// //     let mut num = String::new();

// //     //read the number
// //     io::stdin().read_line(&mut num).expect("Was it a number you picked?");

// //     //run a loop
// //     loop{
// //         &num;

// //         //clear screen
// //         print!("\x1b[2J");
// //         //Hides the cursor
// //         print!("\x1b[?25l");

// //         //convert to u32
// //         let mut value :u32 = num.parse().unwrap();
// //         value -= 1;

// //         if value > 1{
// //             num = value.to_string();

// //             for row in &DIGITS{
// //                 for c in num.chars(){
// //                     let col:usize = match c{
// //                         '0'..='9' => c as usize - '0' as usize,
// //                         '_' => 0,
// //                         _ => 0,
// //                     };
// //                     print!("{}", row[col]);
// //                 }
// //             }
// //             std::thread::sleep(Duration::from_millis(999));
// //             //Moves cursor up 7times to ensure we do not print newlines
// //             // print!("\x1b[7A");
// //             continue;
// //         }else{
// //             println!("DONE!!!");
// //             break;
// //         }
    
// //     }


// // }


// pub fn counter(){
//     println!("Input a number for countdown");

//     //Get Number as String
//     let mut num = String::new();

//     //read the number
//     io::stdin().read_line(&mut num).expect("Was it a number you picked?");

//     //run a loop
//     loop{

//         // //clear screen
//         // print!("\x1b[2J");
//         // //Hides the cursor
//         // print!("\x1b[?25l");

//         //convert to u32
//         let mut value :u32 = num.parse().unwrap();
//         value -= 1;

//         for row in &DIGITS{
//             for c in num.chars(){
//                 let col:usize = match c{
//                     '0'..='9' => c as usize - '0' as usize,
//                     '_' => 0,
//                     _ => 0,
//                 };

//                 if value > 1{
//                     print!("{}", row[col]);
//                     continue;
//                 }else{
//                     break;
//                 }
//             }
//         }
//             //std::thread::sleep(Duration::from_millis(999));
//     }
            


// }