//Problem:

// fn main() {

//     let x = change_value(10,20);
// }

// fn change_value(input:u32, output: &mut u32) -> u32{
//     if input ==1 {
//         *output =3;
//     }
//     else {
//         *output = 4;
//     }

//     *output
// }

//Solution:

fn main() {
    let x = change_value(1, 20);
    println!("{}", x);
}

fn change_value(input: u32, mut output: u32) -> u32 {
    println!("Output_0: {}", output);
    if input == 1 {
        output = 3;
    } else {
        output = 4;
    }
    println!("Output_1: {}", output);
    output
}
