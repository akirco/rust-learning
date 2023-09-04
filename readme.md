## rust install for windows

```shell
scoop install rustup-msvc
```

## new rust project

```shell
# create project
cargo new <projectName>
```

## rustup - rust toolchain installer

```shell
rustup update
rustup doc
```

## rustc - rust complier

```shell
rustc main.rs
# more: rustc -h
```

## cargo - rust package manager

```shell
# create a new package
cargo new [packageName]

# create a new package in a dir
cargo init

# cargo version
cargo --version

# build code --debug
cargo build

# build code --release
cargo build --release

# compiler + excute
cargo run

# more: cargo -h
```

## cargo.toml - config file

```toml
[package] # statements are configuring a package.
name = "hello_world" # compile name
version = "0.1.0"   # compile version
edition = "2021"   # edition of Rust to use ,edition key in Appendix E.https://doc.rust-lang.org/book/appendix-05-editions.html

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]


# list any of your project’s dependencies. In Rust, packages of code are referred to as crates.
```

## cargo concept

> Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. There’s a place for everything, and everything is in its place.

## building and running a cargo project

```shell
cargo build

#   Compiling hello_world v0.1.0 (E:\Notes\rust-learning\hello_world)
#   Finished dev [unoptimized + debuginfo] target(s) in 1.08s


# cargo first build will auto create Cargo.lock file.
```

> This file(Cargo.lock) keeps track of the exact versions of dependencies in your project. You won’t ever need to change this file manually; Cargo manages its contents for you.

## cargo command

```shell
cargo new # create a project
cargo build # build a project ,[target/debug]
cargo run # build and run a project in one step
cargo check # build a project without producing a binary to check for errors

cargo build --release # Building for Release ,[target/release] ,to compile it with optimizations


# extras
cargo search [crateName] # search crate
cargo add [crateName] # add crate
cargo update # update crate
```

## Cargo as Convention

> 对于简单的项目来说，Cargo 与直接使用 rustc 相比提供的价值并不是很大，但是随着程序变得更加复杂，它将证明其价值所在。一旦程序变得越来越复杂，需要使用多个文件或依赖项时，让 Cargo 协调构建会更加容易。

## rust guess game

```rust
// use 导入rust没有预导入(prelude)的库
// :: 语法表示后者与前者的关联关系
/**
 * crate: std 是 Rust 标准库的顶级模块，包含许多 Rust 的标准功能和类型定义。
 * mod: io 是 std 模块中的子模块，包含与输入和输出相关的功能，如文件和标准输入输出流等。
 * fn: stdin 是 io 模块中的一个函数，用于从标准输入流中读取用户输入。
 *
 * crate , mod, fn怎么理解？怎么创建？
 */

// use std::io; // 引入 Rust 标准库中的 io 模块
// use std::io::{xxxx,xxx} //多方法或多子模块导入使用{}

use std::io::stdin; // 具体引入

fn main() { // fn 定义一个无参的main方法
    println!("guess a number!"); //标准输出，with a new line
    println!("please input your guess:");
    let mut guess = String::new(); //mut 定义一个可变的字符串变量 ，否则就不能对其进行修改，只能读取
    stdin().read_line(&mut guess).expect("failed ro read line"); // &mut怎么理解？
    // read_line返回一个Result 类型
    println!("You guessed: {guess}");
}
```

### generating a secret number

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    //* ? ..=
    let secret_number = rand::thread_rng().gen_range(1..=10);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("type err or ...:{e}");
                continue;
            }
        };
        // Rust allows us to shadow the previous value of guess with a new one.
        // 可以使用链式调用判断，相对麻烦
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("TOO SMALL"),
            Ordering::Greater => println!("TOO BIG"),
            Ordering::Equal => {
                println!("YOU WIN");
                // quit loop
                break;
            }
        }
    }
}
```

#### questions

1. [`crate` , `mod`, `fn`怎么理解？怎么创建？](docs/1.md)

2. [`let mut guess = String::new();`中`String`是否可以理解为构造函数?](docs/2.md)

3. [`io::stdin().read_line(&mut guess)`中`&must`怎么理解?](docs/3.md)

4. [`read_line`返回值](docs/4.md)

5. [`println!("You guessed: {guess}");`占位符](docs/5.md)

6. [what is `..= `](docs/6.md)

7. [`enum `类型是否可以用除 `match `外匹配](docs/7.md)

8. [`loop`与 for, while 的区别](docs/8.md)

## rust basic

### variables and mutability - 变量 与 可变性

```rust
fn main() {
    let x = 5; //不可变的变量
    println!("the value of x is: {x}");
    x = 6; //不能给变量x第二次分配值
    println!("the value of x is: {x}");
}
```

```rust
fn main() {
    let mut x = 5;
    println!("the value of x is: {x}"); //5
    x = 6;
    println!("the value of x is: {x}"); //6
}
```

### constants - 常量

1. not allow use it with `mut`
2. can be declared in any scope
3. only as a constant expression

```rust
const THREE*HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### shadowing

> declare a new variable with the same name as previous variable

1. shadowing is diff from marking a variable as mut,we got a compile-time error if reassign this variable without use let keyword.

2. let is create a new variable only reuse the same name.

> 在同一个作用域中使用相同名称的变量来覆盖之前的变量。当我们使用 let 关键字声明一个变量时，如果之前已经存在同名的变量，新的变量会覆盖旧的变量，这就是 shadowing。

> 与重新赋值不同，shadowing 允许我们改变变量的类型和值。这样做的好处是可以在不改变变量名称的情况下，对变量进行修改或重新定义。

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }
    println!("the value of x is: {x}");
}
```

```rust
    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("the spaces length is: {spaces}"); //4
    let mut spaces = "    "; //&str type
    spaces = spaces.len(); // error : usize type
    println!("the spaces length is: {spaces}");
```

#### questions

[shadowing 常见的使用场景](./docs/9.md)
