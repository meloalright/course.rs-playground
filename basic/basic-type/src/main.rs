use num::complex::Complex;

fn fn_2_2_1() {
    let guess = "42".parse::<i32>().expect("Not a number!");
    println!("Hello, world! {}", guess);
    let c = 98_222;
    println!("98_222! {}", c);
    assert_eq!(98_222, 98222);
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [42.0, 42f32, 42.0_f32];

    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 != xyz.2);

    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 'a'..'f' {
        println!("{}", i);
    }
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);

    println!("13.14_f32.round() {}", 13.14_f32.round());

    let w_0: f64 = 3.1;
    let w_1: f64 = 23.0;
    println!("w_0 + w_1 = {}", w_0 + w_1);
}

fn fn_2_2_2() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));

    let t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("这是段毫无意义的代码");
    }
    println!(
        "布尔值 false 占用了{}字节的内存大小",
        std::mem::size_of_val(&f)
    );

    let u = ();
    println!(
        "单元类型 () 占用了{}字节的内存大小",
        std::mem::size_of_val(&u)
    );
}

fn fn_2_2_3() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = {
        let x = 18;
    };
    println!("The value of z is: {:?}", z);
}

fn fn_2_2_4() {
    let x = plus_or_minus(6);

    println!("The value of x is: {}", x);

    // dead_end();
    forever();
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
}

fn forever() -> ! {
    loop {
        //...
    }
}

fn main() {
    // fn_2_2_1();
    // fn_2_2_2();
    // fn_2_2_3();
    fn_2_2_4();
}
