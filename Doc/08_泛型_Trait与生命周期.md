# 泛型与 Trait

- [泛型与 Trait](#泛型与-trait)
  - [重复代码提取](#重复代码提取)
  - [泛型](#泛型)
  - [Trait](#trait)
    - [定义 Trait](#定义-trait)
    - [trait 作为参数](#trait-作为参数)
    - [使用 Trait 作为返回类型](#使用-trait-作为返回类型)
    - [使用示例](#使用示例)

## 重复代码提取

1. 重复代码危害：

   - 容易出错。
   - 需求变更时需要在多处进行更改。

2. 消除重复：提取函数。

   - 识别重复代码。
   - 提取重复代码到函数体中，并在函数签名汇总指定函数到输入和返回值。
   - 将重复的代码使用函数调用进行替代。

   ```rust
   fn largest(list: &[i32]) -> i32 {
       let mut largest = list[0];
       for &item in list {
           if item > largest {
               largest = item;
           }
       }
       largest
   }

   fn main() {
       let number_list = vec![34, 50, 25, 100, 65];
       let result = largest(&number_list);
       println!("The largest number is {}", result);
   }
   ```

## 泛型

1. 泛型：提高代码`复用`能力：
   - 处理重复代码的问题。
2. 泛型是具体类型或其它属性的抽象代替：

   - 你编写的代码不是最终的代码，而是一种模版，里面有一些`占位符`。
   - 编译器在编译时将占位符替换为具体的类型。
   - `fn largest<T>(list:&[T])->T{...}`

3. 函数定义中的泛型：

   - 泛型函数包括：参数类型与返回类型。

   ```rust
   fn largest<T>(list: &[T]) -> T {
       let mut largest = list[0];
       for &item in list {
           if item > largest {
               // 由于泛型不清楚具体类型，需要指定比较函数才可以进行比较，应用到 trait
               largest = item;
           }
       }
       largest
   }

   fn main() {
       let number_list = vec![34, 50, 25, 100, 65];
       let result = largest(&number_list);
       println!("The largest number is {}", result);

       let char_list = vec!['y', 'm', 'a', 'q'];
       let result = largest(&char_list);
       println!("The largest char is {}", result);
   }
   ```

4. Struct 定义中的泛型：

   - 可以使用多个泛型类型参数：
     - 参数太多往往意味着需要重构为更多个更小的单元。

   ```rust
   struct Point<T, U> { // 定义 T、U 支持不同类型传入
       x: T,
       y: U,
   }

   fn test_template_struct() {
       let integer = Point { x: 5, y: 10 };
       let float = Point { x: 1.0, y: 4.0 };

       let diff = Point { x: 5, y: 2.0 };
   }
   ```

5. Enum 定义中的泛型：

   - 可以让枚举的编译持有泛型数据类型：
     - 例如 `Option<T>, Result<T, E>`。

   ```rust
   enum Option<T> {
       Some(T),
       None,
   }

   enum Result<T, E> {
       Ok(T),
       Err(E),
   }
   ```

6. 方法定义中的泛型：

   - 为 struct 或 enum 实现方法的时候，可在定义中使用泛型。

   ```rust
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
   ```

   - struct 里的泛型类型参数可以和方法的泛型类型参数不同。

   ```rust
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
       //p3.x = 5, p3.y = c
   }
   ```

7. 泛型代码的性能：
   - 使用泛型的代码和使用具体类型的代码运行速度是一样的。
   - 单态化（monomorphization）：在编译时将泛型替换成具体类型的过程。

## Trait

1. Trait 告诉 Rust 编译器，某种类型具有哪些并且可以与其他类型共享的功能。
   - Trait：抽象的定义共享行为。
   - Trait 与其他语言的接口（interface）类似，但有些区别。
2. Trait bounds（约束）：泛型类型参数指定为实现了特定行为的类型。

### 定义 Trait

1. Trait 的定义：把方法签名放在一起，来定义实现某种目的所必须的一组行为。

   - 关键字：trait。
   - 只有方法签名，没有具体实现。
   - trait 可以有多个方法：每个方法签名占一行，以 `;` 结尾。
   - 实现该 trait 的类型必须提供具体的方法实现。

   ```rust
   pub trait Summary {
       fn summarize(&self) -> String;
       fn summarize1(&self) -> String;
   }
   ```

2. 在类型上实现 trait：

   - 与为类型实现方法类似。
   - 不同之处：
     - `impl Xxxx for Tweet {....}`。
     - 在 impl 的块里，需要对 Trait 里的方法签名进行具体的实现。

   ```rust
   // trait_demo/lib.rs
   pub trait Summary {
       fn summarize(&self) -> String;
   }

   pub struct NewArticle {
       pub headline: String,
       pub location: String,
       pub author: String,
       pub content: String,
   }

   impl Summary for NewArticle {
       fn summarize(&self) -> String {
           format!("{}, by {} ({})", self.headline, self.author, self.location)
       }
   }

   pub struct Tweet {
       pub username: String,
       pub content: String,
       pub reply: bool,
       pub retweet: bool,
   }

   impl Summary for Tweet {
       fn summarize(&self) -> String {
           format!("{}: {}", self.username, self.content)
       }
   }

   // trait_demo/main.rs
   use trait_demo::Summary;
   use trait_demo::Tweet;
   fn main() {
       let tweet = Tweet {
           username: String::from("horse_ebooks"),
           content: String::from("of course, as you probably alread know, people"),
           reply: false,
           retweet: false,
       };

       println!("one new tweet: {}", tweet.summarize())
   }
   ```

3. 在某个类型上允许实现某个 trait 是有约束的：

   - 这个类型或这个 trait 是在本地 crate 里定义的。
   - 无法为外部类型来实现外部的 trait。
     - 这个限制是程序属性的一部分（也就是一致性）。
     - 更具体说属于孤儿原则：即父类型不存在。
     - 此规则确保其他人的代码不能破外当前的代码，反之亦然。
     - 如果没有这个规则，两个 crate 可以为同一类型实现同一 trait，Rust 就不知道应该使用哪个实现了。

4. 默认实现：默认实现的方法可以调用 trait 中其他的方法，即使这些方法没有默认实现。
   - 注意：无法从方法的重写实现里面调用默认的实现。

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String { // 默认实现的 trait，并且可以调用其他的 trait
        format!("(Read more from {} ...)", self.summarize_author())
    }
}
...
impl Summary for NewArticle {
    // fn summarize(&self) -> String { // 使用默认的 trait
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
```

### trait 作为参数

1. 使用 `impl Trait` 语法：适用于简单情况。

   ```rust
   pub fn notify(item: impl Summary) {
       println!("Breaking news! {}", item.summarize());
   }
   ```

2. `Trait bound` 语法：可用于复杂情况。

   - `impl Trait` 语法是 `Trait bound` 的语法糖。

   ```rust
   pub fn notify_bound<T: Summary>(item: T) {
       println!("Breaking news! {}", item.summarize());
   }
   ```

3. 使用 `+` 指定多个 `Trait bound`。

   ```rust
   pub fn notify_mix(item: impl Summary + std::fmt::Display) {
       println!("Breaking news! {}", item.summarize());
   }

   pub fn notify_bound_mix<T: Summary + std::fmt::Display>(item: T) {
       println!("Breaking news! {}", item.summarize());
   }
   ```

4. `Trait bound` 使用 where 子句。

   - 在方法签名的后边使用 where 子句。

   ```rust
   pub fn notify_bound_mulit_mix<T: Summary + std::fmt::Display, U: Clone + core::fmt::Debug>(
       a: T,
       _b: U,
   ) -> String {
       format!("Breaking news! {}", a.summarize())
   }

   pub fn notify_bound_mulit_mix_where<T, U>(a: T, _b: U) -> String
   where
       T: Summary + std::fmt::Display,
       U: Clone + core::fmt::Debug,
   {
       format!("Breaking news! {}", a.summarize())
   }
   ```

### 使用 Trait 作为返回类型

1. `impl Trait` 语法。

   - 注意：impl Trait 只能返回确定的同一种类型，返回可能不同类型的代码会报错。

   ```rust
   pub fn notify_return_trait(s: &str) -> impl Summary {
       NewArticle {
           headline: String::from("headline str"),
           content: String::from("content text"),
           author: String::from("author info"),
           location: String::from("location info"),
       }
       // 接口中只能返回一种 Trait，类似继承的派生类不会强转为父类
       // if(xxx)
       // {
       //     return tweetinfo will error
       // }
   }
   ```

### 使用示例

```rust
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

//or

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn check() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![String::from("Hello"), String::from("World")];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

1. 使用 Trait Bound 有条件的实现方法：

   - 在使用泛型类型参数的 impl 块上使用 Trait bound，我们可以有条件的为实现了特定 Trait 的类型来实现方法。

   ```rust
   struct Pair<T> {
       x: T,
       y: T,
   }

   impl<T> Pair<T> {
       fn new(x: T, y: T) -> Self {
           Self { x, y }
       }
   }

   impl<T: std::fmt::Display + PartialOrd> Pair<T> {
       fn cmp_display(&self) {
           if self.x >= self.y {
               println!("The largest member is x = {}", self.x);
           } else {
               println!("The largest member is x = {}", self.y);
           }
       }
   }
   ```

   - 也可以为实现了其他 Trait 的任意类型有条件的实现某个 Trait。
   - 为满足 Trait Bound 的所有类型上实现 Trait 叫做覆盖实现（blanket implementations）。
