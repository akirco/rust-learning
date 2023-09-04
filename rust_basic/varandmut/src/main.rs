// variables and mutability - 变量 与 可变性

// fn main() {
//     let x = 5; //不可变的变量
//     println!("the value of x is: {x}");
//     x = 6; // 不能给变量x第二次分配值
//     println!("the value of x is: {x}");
// }

// fn main() {
//     let mut x = 5;
//     println!("the value of x is: {x}");
//     x = 6;
//     println!("the value of x is: {x}");
// }

// constants - 常量
// 1. not allow use it with `mut`
// 2. can be declared in any scope
// 3. only as a constant expression

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// shadowing
// intro: declare a new variable with the same name as previous variable
// 1. shadowing is diff from marking a variable as mut,we got a compile-time
// error if reassign this variable without use let keyword.
// 2. let is create a new variable only reuse the same name.

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("the value of x in the inner scope is: {x}");
//     }
//     println!("the value of x is: {x}");
//
//     // let spaces = "    ";
//     // let spaces = spaces.len();
//     // println!("the spaces length is: {spaces}"); //4
//     let mut spaces = "    ";
//     spaces = spaces.len();
//     println!("the spaces length is: {spaces}");
// }
