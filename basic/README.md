# Basic

> https://course.rs/basic/intro.html

### size_of_val

```rust
fn fn_2_2_2() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    let t = true;
    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("这是段毫无意义的代码");
    }
    println!("布尔值 false 占用了{}字节的内存大小",std::mem::size_of_val(&f));

    let u = ();
    println!("单元类型 () 占用了{}字节的内存大小",std::mem::size_of_val(&u));
}


// >> 字符'中'占用了4字节的内存大小
// >> 布尔值 false 占用了1字节的内存大小
// >> 单元类型 () 占用了0字节的内存大小
```

### ''

> Rust 的字符只能用 '' 来表示， "" 是留给字符串的

```rust
fn main() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c); // >> 中
}
```

### Rust 函数的位置可以随便放

> Rust 不关心我们在哪里定义了函数 只要有定义即可

```rust
fn main() {
    let x = plus_five(5);

    println!("The value of x is: {}", x);
}

fn plus_five(x:i32) -> i32 {
    x + 5
}
```

### 发散函数

> 当用 ! 作函数返回类型的时候，表示该函数永不返回，特别的，这种语法往往用做会导致程序崩溃的函数：

```rust
fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn forever() -> ! {
    loop {
        //...
    }
}
```