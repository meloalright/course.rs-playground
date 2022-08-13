fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}


fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let p = Point{x: 1, y: 1.1};

    let p = Point { x: 5.2, y: 10 };

    println!("p.x = {}", p.x());
    println!("mixup {:?}", p.mixup(Point{ x: 200, y: 400.3 }).y);

    let p = Point { x: 5.2, y: 10.2 };
    println!("distance_from_origin {}", p.distance_from_origin());


    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);

    let arr: [char; 4] = ['a', 'b', 'c', 'd'];
    display_array(arr);
}