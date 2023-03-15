// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1
//     println!("{}",s1);

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                         // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

//--------------Prt2
// fn main(){
//     let s1 = String::from("Hello");

//     println!("{}",s1.len());

//     let s2 = calculate_length(&s1);

//     println!("The Length of {} is {}", s1,s2);

// }

// fn calculate_length(s : &String) -> usize{
//     s.len()
// }


//----------------Prt3
// fn main() {
//     let s = String::from("hello ");

//     change(&s);

//     let r = String::from("Five");
    
//     let r1 = &r;
//     let r2 = &r;
//     println!("{} and {}",r1,r2);
//     println!("{r}");
// }

// fn change(some_string: &String){
//     println!("Some String {}", some_string);
// }


//---------------Dangling References
// fn main() {
//     let reference_to_nothing = dangle();
//     println!("{}",reference_to_nothing);
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }


//--------------String Slices
// fn main() {
//     let s = String::from("Hello World");

//     let words = first_words(&s);

//     println!("{}",words);

// }

// fn first_words(s : &String) -> &str {
//     let bytes = s.as_bytes();

//     for(i, &items) in bytes.iter().enumerate(){
//         if items == b' '{
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

//---------------Other Slices
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}