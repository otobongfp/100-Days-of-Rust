

pub fn gen() {
    let num_lists = vec![2,4,34,6,78,23,56,97,93,64,65];

    let mut largest = &num_lists[0];

    for num_list in &num_lists{
        if num_list > largest {
            largest = &num_list;
        }
    }

    println!("The largest number is {}",largest);

}