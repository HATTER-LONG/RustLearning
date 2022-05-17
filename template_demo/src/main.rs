// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             // 由于泛型不清楚具体类型，需要指定比较函数才可以进行比较，应用到 trait
//             largest = item;
//         }
//     }
//     largest
// }

fn main() {
    test_template_struct();
    test_template_impl();
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn test_template_struct() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct PointI<T> {
    x: T,
    y: T,
}

impl<T> PointI<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl PointI<i32> {
    // 偏特化
    fn xi32(&self) -> &i32 {
        &self.x
    }
}

fn test_template_impl() {
    let integer = PointI { x: 5, y: 10 };
    println!("p.x = {}", integer.x());
}
