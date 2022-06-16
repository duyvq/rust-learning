// fn main(){

// }

//Exercise 1
// Mục đích: giải quyết vấn đề ownership and borrowing không dùng clone()
// fn main() {
    
//     let x = change_value(10,20);                    // Solution 1: add "&mut" to front of second argument 20
// }



// fn change_value(input:u32, mut output: u32) -> u32{ // Solution 2: remove "reference" of argument output
//     if input ==1 {                                  // and remove "dereference" of output inside function
//         output =3;
//     }
//     else {
//         output = 4;
//     }

//     output
// }


//Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố 
// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     while count < 10 {
//         num += 2;
//         if vector_is_prime(num, &primes) {              // Solution 1: step 2, borrow primes instead of taking ownership
//             count += 1;
//             primes.push(num);
//         }
        
//     }
//     println!("{:?}", primes);
// }

// fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {    // Solution 1: step 1, argument "p" is changed to borrow type
//     let mut counter = 0;                                // Revise logic: step 1, add counter
//     for i in p {
//             if num > *i && num % i != 0 {               // Solution 1: step 3, dereference "i" here
            
//             counter += 1;                               // Revise logic: step 2, don't return immediately after if but increase counter
//         }                                               // to check iterately over all the element of vector
//                                                         // to ensure number isn't divisible by other number than itself

//         if counter == p.len() {        
//         return true;                                    // Revise logic: step 3, if counter equals length of vector, num is prime number
//         }
//     }

//     false
// }



//Exercise 3
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;

//     let mut max = 0;
    
//     //for n in &mut values {
//     for n in v.into_iter() {                                      // Solution 1: reborrow "v" or Solution 2: add into_iter()
//         max = std::cmp::max(max, *n);
//     }

//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");
//     //for n in &mut values {
//     for n in v {
//         *n = 100 * (*n) / max;
//     }
//     println!("values: {:#?}", values);
// }


//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn main(){
    let mut a = vec![1,2,3,4,5];                               // Solution: Change a to mutable
    let  i = 0;
    let c = 0;
    loop {
        let (a, c) = test(&mut a);                             // Solution: mutable reference a instead of taking ownership
        println!("{}",c);
        if c >= a.iter().map(|i| (*i) as i32).sum()  {break;}  // Logic revise: check if c equals sum of a's element to quit the loop
    }
}

pub fn test(a: &mut Vec<u8>) -> (Vec<u8>, i32) {               // Solution: mutable reference a instead of taking ownership
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