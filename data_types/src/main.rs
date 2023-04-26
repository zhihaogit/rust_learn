fn main() {
    // println!("Hello, world!");
    // let guess: u32 = "42".parse().expect("Not a number!");

    // 标量（scalar）表示一个单独的值
    // 整形、浮点型、布尔类型和字符类型

    let x = 2.0; // 单精度浮点型
    let y: f32 = 3.0; // 双精度浮点型

    // 加
    let sum = 5 + 10;
    // 减
    let difference = 95.5 - 4.3;
    // 乘
    let product = 4 * 30;
    // 除
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    // 取余
    let remainder = 43 % 5;
    println!("{x} {y} {sum} {difference} {product} {quotient} {floored} {remainder}");

    let t = true;
    let f: bool = false;
    println!("{t} {f}");

    // 字符型
    // 单引号声明 char，双引号声明字符串
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    // 复合类型，多个值合成一个类型
    // 长度固定，长度不会增长和缩小
    // 不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组
    // 这种值以及对应的类型都写作 ()，表示空值或空的返回类型
    // 如果表达式不返回任何其他值，则会隐式返回单元值

    // 定义元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 解构元组
    let (x, y, z) = tup;
    println!("{x} {y} {z}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} {six_point_four} {one}");

    // 定义数组
    // 数组长度是固定的
    // 索引超界会报错
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // 5个元素 3
    let first = a[0];
    let second = b[1];
    let third = c[2];
    println!("{first} {second} {third}");
}
