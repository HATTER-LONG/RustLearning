// 不可以直接写返回闭包，无法推断其具体大小 Sized trait
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

// 放入智能指针中
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn main() {}
