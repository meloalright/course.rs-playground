fn fn_2_3_2_a() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);
    println!("After change the s1 is '{}'.", s1);
}

fn fn_2_3_2_b() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("r1 = {}", r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("r2 = {}", r2);
}

// fn fn_2_3_2_c() {
//     let reference_to_nothing = dangle(); // 悬垂引用会被编译前提前报出错误
// }

fn fn_2_3_2_d() {
    let x = 5;
    // 填写空白处
    let p = &x;
 
    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84

    let mut s = String::from("hello, ");

    borrow_object(&s)
}

fn main() {
    // fn_2_3_2_a();
    // fn_2_3_2_b();
    // fn_2_3_2_c();
    fn_2_3_2_d();
}

fn calculate_length(s: &mut String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }


fn borrow_object(s: &String) {}
