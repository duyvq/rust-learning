// Đề bài : Implement trait Iterator (của thư viện Rust) cho kiểu dữ liệu Struct sau

//https://doc.rust-lang.org/std/iter/trait.Iterator.html

// struct Fibonacci {
//     a: u32,
//     b: u32,
// }
// Như mọi người đã biết Dãy Fibonacci có quy luật như sau
// Dãy Fibonacci là dãy số bắt đầu bằng 0 với 1. Mọi số tiếp theo
// đều là tổng của 2 số trước đó
// Ví dụ: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, ...


// Trong trường hợp bài toán này 
// Khởi tạo ban đầu sẽ là 
// struct Fibonacci {
//     a = 1,
//     b = 0,
// }

// Một số kiến thức để làm dc bài này: Trait, Generic Type, Associated type,
// Gợi ý có sườn như sau:

#[derive(Debug)]
struct Fibonacci {
    a: u32,
    b: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

// Trả về số fibonaci tiếp theo dựa trên kiểu dữ liệu struct Fibonacci
    fn next(&mut self) -> Option<u32> {
        if self.a <= self.b {            
            self.a = self.a + self.b;            
            Some(self.b)
        } else if self.a > self.b {
            let x = self.a;
            self.a = self.b;
            self.b = x;   
            self.a = self.a + self.b;            
            Some(self.b)   
        } else {
            None
        }
    }
}

// Khởi tạo ban đầu cho Fibonaci: 0, 1
fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: 1, b: 0 }
}

fn main() {
//     Vì struct Fibonacci có implement trait Iterator của Rust nên 
// có thể dùng câu lệnh for dc
// Câu lệnh for bản chất sẽ chuyển qua trait Iterator nên instance của
// struct Fibonacci có thể duyệt được, 
// Mỗi lần duyệt sẽ tự động chạy function signature next() trên
// Nên cần implement hàm next() cho struct Fiboncci.
    for number in fibonacci_numbers() {
        println!("{}", number);
    }
}