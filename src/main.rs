fn problem1(arr: Vec<i32>) {
    println!("Problem 1: ");
    println!("Write three functions that compute the sum of the numbers in a given list using a for-loop, a while-loop, and recursion.");

    // For loop
    let mut for_loop_answer: i32 = 0;
    for i in 0..arr.len() {
        for_loop_answer += arr[i];
    }
    println!("For loop answer: {}", for_loop_answer);

    // while loop
    let mut while_loop_answer: i32 = 0;
    let mut loop_over: bool = false;
    let mut c_index: usize = 0;
    while !loop_over {
        if c_index == arr.len() {
            loop_over = true;
        } else {
            while_loop_answer += arr[c_index];
            c_index += 1;
        }
    }
    println!("while loop answer: {}", while_loop_answer);

    // recursion
    let mut recursion_loop_answer: i32 = 0;
    let mut nc_index: usize = 0;
    'recursion: loop {
        if nc_index == arr.len() {
            break 'recursion;
        } else {
            recursion_loop_answer += arr[nc_index];
            nc_index += 1;
        }
    }
    println!("recursion loop answer: {}", recursion_loop_answer);
}

fn main() {
    problem1(vec![0, 1, 2, 3, 4, 5, 6]);
}
