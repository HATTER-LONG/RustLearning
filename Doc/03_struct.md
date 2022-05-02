# struct

- [struct](#struct)
  - [定义 struct](#定义-struct)
  - [Tuple struct](#tuple-struct)
  - [Unit-Like Struct 没有任何字段](#unit-like-struct-没有任何字段)
  - [struct 数据的所有权](#struct-数据的所有权)
  - [struct 方法](#struct-方法)
    - [关联函数](#关联函数)
    - [多个 impl 块](#多个-impl-块)

1. struct 结构体，自定义的数据类型，为相关联的值命名，打包 => 有意义的组合。

## 定义 struct

1. 使用 struct 关键字，并为其命名。在花括号内，为所有字段（Field）定义名称和类型。

   ```rust
   struct User{
       username: String,
       email: String,
       sign_in_count: u64,
       active: bool,
   }
   ```

2. 实例化 struct：

   - 为每个字段指定具体值。
   - 无需按照声明的顺序进行指定。

   ```rust
   let user1 = User{
       email: String::from("someone@example.com"),
       username: String:from("someusername123"),
       active: true,
       sign_in_count:1,
   }
   ```

3. 访问 struct 里面的某个值：

   - 使用点标记法。
   - 一旦 struct 的实例是可变的，那么实例中的所有字段都是可变的。

   ```rust
   let mut user2 = User {
       email: String::from("someone@example.com"),
       username: String::from("someusername123"),
       active: true,
       sign_in_count: 1,
   };

   user2.email = String::from("aaaa@example.com");
   ```

4. struct 作为函数的返回值。

   ```rust
   let _user_info: User = test("aaa@ex.com".to_string(), "ccl".to_string());

   fn test(email: String, username: String) -> User {
       User {
           username: username,
           email: email,
           sign_in_count: 1,
           active: true,
       }
   }
   ```

   - 同名字段简写：

   ```rust
    fn test(email: String, username: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
   ```

5. struct 更新语法：

   - 当想基于某个 struct 实例来创建一个新实例的时候，可以使用 struct 更新语法。
   - 语法糖 ..user_info，将其余项赋值为 user_info 的。

   ```rust
    let _user_info2: User = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        active: user_info.active,
        sign_in_count: user_info.sign_in_count,
    };

    let _user_info3: User = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user_info
    };
   ```

## Tuple struct

1. 可以定义类似 tuple 的 struct，叫做 tuple struct。
   - Tuple struct 整体有个名称，但里面的元素没有名称。
   - 适用：想给整个 tuple 进行命名，并让它不同于其他 tuple，而且又不需要给每个元素起名。
2. 定义 tuple struct：使用 struct 关键字，后边是名称以及里面的元素类型。

   - 下例中的 black 与 origin 是不同类型是，属于不同 tuple struct 的实例。

   ```rust
   struct Color(i32, i32, i32);
   struct Point(i32, i32, i32);
   let black = Color(0, 0, 0);
   let origin = Point(0, 0, 0);
   ```

## Unit-Like Struct 没有任何字段

1. 可以定义没有任何字段的 struct，叫做 Unit-Like struct。
2. 适用于需要在某个类型上实现某个 trait， 但是在里面又没有想要存储的数据。

## struct 数据的所有权

1. 以前边的 User struct 为例，其中的字段使用 String 而不是 &str，则表示 User struct 拥有全部的字段所有权。
   - 只要 struct 实例是有效的，那么里面的字段数据也是有效的。
2. struct 里也可以存放引用，但这需要使用生命周期。

## struct 方法

1. 方法和函数类似：fn 关键字、名称、参数、返回值。
2. 方法与函数不同：

   - 方法是在 struct （或 enum、trait 对象）的上下文中定义。
   - 第一个参数是 self，表示方法被调用的 struct 实例。

   ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        length: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.length
        }
    }

    fn main() {
        let rect = Rectangle {
            width: 30,
            length: 50,
        };

        println!("{}", rect.area());

        println!("{:#?}", rect);
    }

   ```

3. 在 impl 块里定义方法，方法的第一个参数可以是 &self，也可以获得其所有权或可变借用。和其他参数一样。同样可以获得更良好的代码组织。

4. 方法参数，与普通函数类似任意携带：

   ```rust
   impl Rectangle {
       fn area(&self) -> u32 {
           self.width * self.length
       }

       fn can_hold(&self, other: &Rectangle) -> bool {
           self.width > other.width && self.length > other.length
       }
   }
   ```

### 关联函数

1. 可以在 impl 块里定义不把 self 作为第一个参数的函数名，它们叫关联函数（不是方法），例如 `String::from()`。
2. 关联函数通常用于构造器。

   ```rust
   impl Rectangle {
       fn area(&self) -> u32 {
           self.width * self.length
       }

       fn can_hold(&self, other: &Rectangle) -> bool {
           self.width > other.width && self.length > other.length
       }

       fn square(size: u32) -> Rectangle {
           Rectangle {
               width: size,
               length: size,
           }
       }
   }

   let s = Rectangle::square(20);
   println!("{:#?}", s);
   ```

### 多个 impl 块

1. 每个 struct 允许拥有多个 impl 块。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}
```
