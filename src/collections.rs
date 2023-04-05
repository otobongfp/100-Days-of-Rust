pub fn collections(){

    //Sorting for the largest number
    let number_list = vec![23,45,21,12,65,78,34,98,55];
    let result = largest(&number_list);
    println!("The largest number here is {}",result);


    let number_list = vec![7,34,12,65,23,42,56,86,2];
    let result = largest(&number_list);
    println!("The largest number here is {}",result);
}

fn largest(list: &[i32]) -> &i32{
    let mut largest = &list[0];

    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}