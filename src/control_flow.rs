// 控制流
fn main() {
  // if...else...
  let number = 7;
  // if 代码中的条件必须是 bool 
  if number < 5 {
      println!("condition was true");
  } else {
      println!("condition was false");
  }

  // 在 let 语句中使用 if
  let condition = true;
  let number = if condition {
      5
  } else {
      6
      // "six" 会报错，变量中只有一个类型
      // 编译器必须跟踪每一个变量的多种假设类型，那么它就会变得更加复杂，对代码的保证就会减少
  };
  println!("the value of number {}", number);

  // loop  猜数游戏
  let mut count = 0;
  // 'counting_up（loop label）循环标签
  // break 退出循环
  'counting_up: loop {
      println!("count = {}", count);

      let mut remaining = 10;
      loop {
          println!("remaining = {}", remaining);
          if remaining == 9 {
              break;
          } 
          
          if count == 2 {
              break 'counting_up;
          }

          remaining -= 1;
      }

      count += 1;
  }

  println!("End count = {}", count);
  
  let mut counter = 0;
  
  let result = loop {
      counter += 1;

      if counter == 10 {
          // 如果将返回值加入你用来停止循环的 break 表达式，它将会被停止的循环返回;
          // 这里会生效 返回的数值结果为 20
          break counter * 2;
      }
  };

  println!("The result is {}", result);

  // while 条件为真，执行循环。条件不在为真，调用 break 停止循环
  let mut number = 3;
  while number != 0 {
      println!("{}!", number);

      number = number - 1;
  }

  println!("LIFTOFF!!!");

  // 使用 while 结构来遍历集合中的元素
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  // before 这样的代码安全性不高，如果 a length < 4 代码将会 panic
  while index < 5 {
      println!("the value is: {}", a[index]);

      index = index + 1;
  }

  // after  rev用来反转range
  for number in (1..4).rev() {
      println!("{}!", number);
  }
  println!("LIFTOFF!!!");

  // 使用 for 循环遍历集合中的元素
  for element in a.iter() {
      println!("the value is: {}", element);
  }
}