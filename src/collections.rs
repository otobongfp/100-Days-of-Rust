pub fn collections(){
    //Vec

    let v = vec![2,3,4,5,6,7];

    let num :Option<&i32>  = v.get(4);
    match num{
        Some(num) => println!("Ouput the 4th value {num}"),
        None => println!("Actually nothing was provided"),
    }
}