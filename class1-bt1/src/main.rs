use std::process;

fn main() {
    let org_arr = [1, 2,5,6,8,10, 11];
    let sub_arr = [3,7,9];    

    // Set counter to count up when element of sub_array is in org_array
    let mut counter = 0;

    // Set x & y for range of array
    let mut x = 0;
    let mut y = 0;

    // Find the index of the 1st match in both array
    'main_loop: for i in x..sub_arr.len() {
        for j in y..org_arr.len() {
            if sub_arr[i] == org_arr[j] {
                counter += 1;

                // Narrow down the range to check the remaining of array
                x = i + 1;
                y = j + 1;
                break 'main_loop;
            } else {
                // Exit program if not any match
                println!("Subarray {:?} is NOT a child of Array {:?}.", sub_arr, org_arr);
                process::exit(1)
            }            
        }
    }

    // Check the rest & ensure the order
    while x < sub_arr.len() {
        if sub_arr[x] == org_arr[y] {
            counter += 1;
            x += 1;
            y += 1;
        } else {
            break
        }
    }
    
    // Check and print out result
    if counter == sub_arr.len() {
        println!("Subarray {:?} is a child of Array {:?}.", sub_arr, org_arr);
    } else {
        println!("Subarray {:?} is NOT a child of Array {:?}.", sub_arr, org_arr);
    }
}


