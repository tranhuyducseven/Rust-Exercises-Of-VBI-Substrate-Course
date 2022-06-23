#[derive(Debug)]
pub struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let capacity = self.a + self.b;

        self.b = self.a;
        self.a = capacity;

        Some(self.b)
    }
}

pub fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}
fn main() {
    for number in fibonacci_numbers() {
        println!("{}", number);
        if number > 10000 {
            break;
        }
    }
}
