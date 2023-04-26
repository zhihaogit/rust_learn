fn main() {
    // println!("Hello, world!");
    let mut x = 5; // mut声明一个可变的变量
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // 定义一个与之前变量同名的新变量。Rustacean 们称之为第一个变量被第二个 隐藏（Shadowing） 了
    let x = x + 1;

    {
        let x = x * 2;
        print!("The value of x in the inner scope is: {x} ");
    }

    // 使用 const来声明一个常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    print!("The valve of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
}
