# ownership

> https://zh.practice.rs/ownership/intro.html

### Mutable Reference

> 同一作用域，特定数据只能有一个可变引用
> 大括号可以帮我们解决一些编译不通过的问题，通过手动限制变量的作用域

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    println!("r1 = {}", r1);

} // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

let r2 = &mut s;
println!("r2 = {}", r2);
```