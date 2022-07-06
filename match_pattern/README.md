# Match Pattern

[https://course.rs/basic/match-pattern/intro.html](https://course.rs/basic/match-pattern/intro.html)

## Match or If Let

```rust
// 有时会遇到只有一个模式的值需要被处理，其它值直接忽略的场景，如果用 match 来处理就要写成下面这样：
let v = Some(3u8);
match v {
    Some(3) => println!("three"),
    _ => (),
}

// 我们只想要对 Some(3) 模式进行匹配, 不想处理任何其他 Some<u8> 值或 None 值。但是为了满足 match 表达式（穷尽性）的要求，写代码时必须在处理完这唯一的成员后加上 _ => ()，这样会增加不少无用的代码。

// 杀鸡焉用牛刀，可以用 if let 的方式来实现：
if let Some(3) = v {
    println!("three");
}

// 这两种匹配对于新手来说，可能有些难以抉择，但是只要记住一点就好：当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
```

```rust
// 无论是 match 还是 if let，他们都可以在模式匹配时覆盖掉老的值，绑定新的值:


fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   if let Some(age) = age {
       println!("匹配出来的age是{}",age);
   }

   println!("在匹配后，age是{:?}",age);
}
// cargo run 运行后输出如下：


// 在匹配前，age是Some(30)
// 匹配出来的age是30
// 在匹配后，age是Some(30)
// 可以看出在 if let 中，= 右边 Some(i32) 类型的 age 被左边 i32 类型的新 age 覆盖了，该覆盖一直持续到 if let 语句块的结束。因此第三个 println! 输出的 age 依然是 Some(i32) 类型。

// 对于 match 类型也是如此:


fn main() {
   let age = Some(30);
   println!("在匹配前，age是{:?}",age);
   match age {
       Some(age) =>  println!("匹配出来的age是{}",age),
       _ => ()
   }
   println!("在匹配后，age是{:?}",age);
}
// 需要注意的是，match 中的变量覆盖其实不是那么的容易看出，因此要小心！
```

## Matches

```rust
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }
} 
```

