fn problem1(arr: Vec<i32>) {
    println!("");
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

fn problem2(arr1: Vec<&str>, arr2: Vec<i32>) {
    println!("");
    println!("Problem 2:");
    println!("Write a function that combines two lists by alternatingly taking elements. For example: given the two lists [a, b, c] and [1, 2, 3], the function should return [a, 1, b, 2, c, 3].");

    let mut result_arr: Vec<String> = vec![];
    let mut for_len: usize = 0;
    if arr1.len() > arr2.len() {
        for_len = arr1.len() - arr2.len();
    } else if arr2.len() > arr1.len() {
        for_len = arr2.len() - arr1.len();
    } else if arr2.len() == arr1.len() {
        for_len = arr1.len();
    }

    for i in 0..for_len {
        result_arr.push(arr1[i].to_string());
        result_arr.push(arr2[i].to_string());
    }

    if arr1.len() > arr2.len() {
        for i in for_len..arr1.len() {
            result_arr.push(arr1[i].to_string());
        }
    } else if arr2.len() > arr1.len() {
        for i in for_len..arr2.len() {
            result_arr.push(arr2[i].to_string());
        }
    }

    for i in 0..result_arr.len() {
        print!("{},", result_arr[i]);
    }
    println!("");
}

fn problem3(num: usize) {
    println!("");
    println!("Problem 3:");
    println!("Write a function that computes the list of the first 100 Fibonacci numbers. By definition, the first two numbers in the Fibonacci sequence are 0 and 1, and each subsequent number is the sum of the previous two. As an example, here are the first 10 Fibonnaci numbers: 0, 1, 1, 2, 3, 5, 8, 13, 21, and 34.");

    let mut arr: Vec<i32> = vec![0, 1];
    let mut c_index: usize = arr.len() - 1;
    'fibonacci: loop {
        if arr.len() == num {
            break 'fibonacci;
        }
        arr.push(arr[c_index] + arr[c_index - 1]);
        c_index += 1;
    }

    for i in 0..arr.len() {
        print!("{},", arr[i]);
    }
    println!("");
}

fn main() {
    problem1(vec![0, 1, 2, 3, 4, 5, 6]);
    problem2(vec!["a", "b", "c"], vec![1, 2, 3]);
    problem3(10);
}
