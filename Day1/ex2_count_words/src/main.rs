// let given_string = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");

use std::collections::BTreeMap;
use std::io;
use std::io::BufRead;

fn main() {
    let mut counts: BTreeMap<String, isize> = BTreeMap::new();
    let stdin = io::stdin();
    for line_result in stdin.lock().lines() {
        match line_result {
            Ok(line) => {
                let lowercase_line = line.to_lowercase();
                let words = lowercase_line
                    .split(|c: char| !(c.is_alphabetic() || c == '\''))
                    .filter(|s| !s.is_empty());
                for word in words {
                    *(counts.entry(word.to_string()).or_insert(0)) += 1;
                }
            }
            Err(e) => {
                panic!("Error parsing stdin: {:?}", e);
            }
        }
    }
    for (key, value) in counts.iter() {
        println!("{} {}", key, value);
    }
}





// fn main(){
//     // Mọi người tự làm thêm phần để gọi các hàm
// }
// fn find_text() {

//     let input = input_from_stdin();


//     let text = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
//     let text = read_file("a.txt");
   
//     let first_char = input.chars().next().unwrap().to_lowercase().next().unwrap();

//     let chars = text.chars().collect::<Vec<char>>();
//     let input_chars = input.chars().collect::<Vec<char>>();

//     let mut index = 0;
//     let mut count = 0;
 
//     loop {

//         if index >= text.len() {
//             break;
//         }

//         if chars[index].to_lowercase().next().unwrap() == first_char { 

//             if index + input.len() > text.len() {
//                 println!("Khong hop le");
//                 break;
//             }

//             let mut check = true;
//             for i in 1..input.len() {
//                 if chars[index + i] != input_chars[i] {
//                     check = false;
//                 }
//             }

//             if check {
//                 index = index + input.len();
//                 count += 1;
//                 continue;
//             }
//         }

//         index += 1;
//     }

//     println!("count = {}", count);

// }

// use std::fs::File;
// fn read_file(path: &str) -> String {
//     let mut file = File::open(path).expect("File not found");
//     let mut data = String::new();
//     file.read_to_string(&mut data)
//                                     .expect("Error while reading file");
//     data
// }

// fn input_from_stdin() -> String {
//     let mut line = String::new();
//     println!("Enter any word: ");
//     io::stdin().read_line(&mut line).expect("Error at input");
//     line.pop();
//     line
//     // line.trim().to_string()
// }

