# compound_type

> https://course.rs/basic/compound-type/intro.html

### Tuple Struct

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Point(0, 127, 255);
    check_color(v);
}

fn check_color(p: Point) {
    let Point(x, y, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}
```

### 链表

```rust
// 填空，让代码运行
use crate::List::*;

enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

// 为枚举实现一些方法
impl List {
    // 创建空的链表
    fn new() -> List {
        // 因为没有节点，所以直接返回 Nil 节点
        // 枚举成员 Nil 的类型是 List
        Nil
    }

    // 在老的链表前面新增一个节点，并返回新的链表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        match *self {
            // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 空链表的长度为 0
            Nil => 0
        }
    }

    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // 递归生成字符串
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 创建一个新的链表(也是空的)
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(15);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}

// >> 链表的长度是: 4
// >> 15, 3, 2, 1, Nil
```

### Option<T>

```rust
// 修复代码中的错误
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
    let _name0 = names.get(0).unwrap();

    // 但是下标索引就存在越界的风险了
    let _name1 = match names.get(1) {
        None => {
            println!("None!")
        },
        Some(i) => {
            println!("i = {}", i)
        }
    };
}

// >> i = Sunface
```