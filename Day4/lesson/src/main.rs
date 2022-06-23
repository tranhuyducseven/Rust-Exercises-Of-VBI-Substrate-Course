// use std::io;
// fn main() {
//     let mut input: Vec<&str>;
//     loop {
//         let mut input_text = String::new();
//         println!("Type instruction in the format Add <name> to <department>:");
//         io::stdin()
//             .read_line(&mut input_text)
//             .expect("failed to read from stdin");
//         let trimmed_text: String = input_text.trim().to_string();
//         input = trimmed_text.split(" ").collect();
//         if input[0] == "Add" && input[2] == "to" {
//             break;
//         } else {
//             println!("Invalid format.");
//         }
//     }
//     println!("{:?}", input);
// }
