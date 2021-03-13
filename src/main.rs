use std::io;

const BINARY: u32 = 2;


fn binary_to_decimal() {
    println!("Please input a binary number to convert to decimal.");

    let mut user_bin = String::new();

    io::stdin()
        .read_line(&mut user_bin)
        .expect("Failed to read line");
    
    if user_bin.ends_with('\n') {
        user_bin.pop();
        if user_bin.ends_with('\r') {
            user_bin.pop();
        }
    }

    let mut ans: u32 = 0;
    let mut factor: u32 = 0;

    let bin_reversed: String = user_bin.chars().rev().collect::<String>();
    
    for place in bin_reversed.chars() {
        match place {
            '1' => {ans += BINARY.pow(factor)}
            '0' => (),
            _ => panic!("Invalid character in input string")
        }

        factor += 1
    }

    println!("The number {} is {} in decimal.", user_bin, ans)
}


fn decimal_to_binary() {
    println!("Please input a decimal number to convert to binary");

    let mut user_dec = String::new();

    io::stdin()
        .read_line(&mut user_dec)
        .expect("Failed to read line");

    if user_dec.ends_with('\n') {
        user_dec.pop();
        if user_dec.ends_with('\r') {
            user_dec.pop();
        }
    }

    let mut num: u32 = user_dec.parse().expect("Not a valid number");

    let mut rem: u32;
    let mut dec: Vec<u32> = Vec::new();

    while num > 0 {
        rem = num % BINARY;
        num = num / BINARY;

        dec.push(rem);
    }

    let mut output = String::new();

    for number in dec.iter().rev() {
        output.push_str(&number.to_string())
    }

    println!("The number {} is {} in binary", user_dec, output);

}


fn main() {
    println!("Welcome to my bin/dec converter.
    Are we converting to binary or to decimal?
    (b for binary, d for decimal)");

    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

    let resp: char = response.chars().next().unwrap(); 

    match resp {
        'b' | 'B' => decimal_to_binary(),
        'd' | 'D' => binary_to_decimal(),
        _ => panic!("Response doesn't indicate binary or decimal")
    }
}
