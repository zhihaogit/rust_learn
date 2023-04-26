fn main() {
    // println!("Hello, world!");

    let number = 7;

    // if语句
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if条件只能是 boolean类型
    if number != 0 {
        println!("number was something other than zero");
    }

    // if else 语句
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //  let语句中使用 if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}

fn infinite_loop_fn() {
    // 循环语句
    loop {
        print!("again!");
    }
}

fn loop_counter() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
    }
}
