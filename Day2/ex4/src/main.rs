//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
// fn main(){
//     let  a = vec![1,2,3,4,5];
//     let  i = 0;
//     let c = 0;
//     loop {
//         let (a, c) = test(a);
//         println!("{}",c);
//         if i >=5 {break;}
//     }
// }

// pub fn test(mut a: Vec<u8>) -> (Vec<u8>, i32) {
//     let mut b:Vec<u8>  = Vec::new();
//     let mut c:u8 = 0;
//     loop {
//         if a.len() == 0 { break; }
//         let d = a.pop().unwrap();
//         c = c+d;
//         b.push(d);
//     }
//     (b, c as i32)
// }

//Solution::
//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn main(){
    let a = vec![1,2,3,4,5];
    let i = 0;
    let c = 0;
    loop {
        let (&mut a:  c) = test(&a);
        println!("{}",c);
        if i >=5 {break;}
    }
}

pub fn test(mut a: &Vec<u8>) -> (&mut Vec<u8>, i32) {
    let mut b:Vec<u8>  = Vec::new();
    let mut c:u8 = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b, c as i32)
}