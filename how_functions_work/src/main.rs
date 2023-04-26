fn main() {
    println!("Hello, world!");

    another_function();

    another_function_params(5);

    print_labeled_measurement(5, 'h');

    // 创建一个表达式
    // let y = {
    //     let x = 3;
    //     x + 1;
    // };

    let y = five();
    println!("The value of y is: {y}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_params(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 具有返回值的函数
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
   return  x + 1;
}
