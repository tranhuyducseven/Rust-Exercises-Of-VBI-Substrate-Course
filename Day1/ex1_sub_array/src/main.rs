use std::io;

fn is_sub_array(parent_array: Vec<String>, sub_array: Vec<String>) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < parent_array.len() && j < sub_array.len() {
        if parent_array[i] == sub_array[j] {
            i += 1;
            j += 1;
            if j == sub_array.len() {
                return true;
            };
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    return false;
}
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn input_array(name_array: &str) -> Vec<String> {
    println!("Input the number of elements in {} array:", name_array);
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Need number of array");

    let number_of_array: usize = buffer.trim().parse().unwrap();
    buffer.clear();
    let mut arr = vec![String::new(); number_of_array];
    for x in 0..number_of_array {
        println!("{}_arr[{}]: ", name_array, x);
        io::stdin().read_line(&mut buffer).expect("Need a number");
        arr[x] = buffer.trim().into();
        buffer.clear();
    }
    arr
}

fn main() {
    let parent_array: Vec<String> = input_array("parent");
    let sub_array: Vec<String> = input_array("sub");
    if is_sub_array(parent_array, sub_array) {
        println!("Yes, it is a sub array")
    }
    else{
        println!("No, it isn't")
    }
}
