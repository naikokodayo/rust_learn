// 数据类型 - 复合类型
use std::io;

fn main() {
    // Tuple 元组 小括号，值用 逗号 隔开
    // tuple 每个位置都对应一个类型、且类型不必相同
    let tup: (i32, f64, u8) = (100, 1.1, 1);

    // 点标记法获取值，后面是元素的索引号
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // 可以使用 解构获取 tuple 获取元素值
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    // 数组 
    // 数组里每个类型的值必须相同， 数组的类型是固定的
    // vector 和 数组类似 比数组更灵活， 由基础库提供

    // [&str; 12]
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let first = months[0];
    println!("{}", first);
    // let a = [3; 5]; 写法相同于 => let a = [3, 3, 3, 3, 3];
    // let a = [3; 5];


    // 如果访问超过了数组的索引的最大，则会引起程序 panic
    // [i32; 5]
    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number");
    
    let element = a[index];

    println!("The value of element index {} is: ", element);
}