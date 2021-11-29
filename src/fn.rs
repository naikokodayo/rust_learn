// 函数如何工作
// rust 不关心函数定义于何处，只要定义了就可以
fn main() { // argument
    another_function();
    let f = five();
    println!("{}", f);
    let p = plus_one(1);
    println!("{}", p);
}

// 包含语句和函数体的表达式
fn another_function() { // parameter
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
println!("Another Function {} {}", x, y);
}


// 具有返回值的函数
// -> i32 则是声明表达式 返回的类型
fn five() -> i32 {
    5
}


fn plus_one(x: i32) -> i32 {
    // 这里如果在 x + 1 后 添加一个分号，则会 把表达式变成一个语句，此时将会看到一个错误
    x + 1
}