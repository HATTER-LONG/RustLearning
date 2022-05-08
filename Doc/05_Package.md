# Package、Crate and Module

- [Package、Crate and Module](#packagecrate-and-module)
  - [Rust 的代码组织](#rust-的代码组织)
  - [Package 和 Crate](#package-和-crate)
    - [Cargo 的惯例](#cargo-的惯例)
  - [定义 module 来控制作用域与私有性](#定义-module-来控制作用域与私有性)
  - [路径](#路径)
  - [私有边界 privacy boundary](#私有边界-privacy-boundary)
    - [pub 关键字](#pub-关键字)
    - [super 关键字](#super-关键字)
    - [pub struct](#pub-struct)
    - [pub enum](#pub-enum)
  - [use 关键字](#use-关键字)
    - [use 的习惯用法](#use-的习惯用法)
    - [as 关键字](#as-关键字)
    - [使用 pub use 重新导出名称](#使用-pub-use-重新导出名称)
  - [package](#package)
    - [使用嵌套路径清理大量的 use 语句](#使用嵌套路径清理大量的-use-语句)
    - [通配符 \*](#通配符-)
  - [如何将模块拆分为不同的文件](#如何将模块拆分为不同的文件)

## Rust 的代码组织

1. 代码组织主要包括：

   - 哪些细节可以暴露，哪些细节是私有的。
   - 作用域内哪些名称有效。

2. 这些组织被称为模块系统：
   - Package（包）：Cargo 的特性，使用者构建、测试、共享 crate 的基础。
   - Crate（单元包）：一个模块树，它可产生一个 library 或可执行文件。
   - Module（模块）、use：让使用者控制代码的组织、作用域、私有路径。
   - Path（路径）：为 struct、function 或 module 等项命名的方式。

## Package 和 Crate

1. Crate 包括 binary 和 library 两种类型。
2. Crate Root 则表示为源代码文件，Rust 编译器是从这里开始，组成你的 Crate 的 Module。
3. 一个 Package ：
   - 包括一个 Cargo.toml，它描述了如何构建这些 Crates。
   - 只能包含 0-1 个 library crate。
   - 可以包含任意数量的 binary crate。
   - 但必须至少包含一个 crate（library 或 binary）。

### Cargo 的惯例

1. src/main.rs：

   - 是 binary crate 的 crate root。
   - crate 名与 package 名相同。

2. src/lib.rs:

   - package 包含一个 library crate。
   - library crate 的 crate root。
   - crate 名与 package 名相同。

3. Cargo 把 crate root 文件交给 rustc 来构建 library 或 binary。

4. 一个 Package 可以同时包含 src/main.rs 和 src/lib.rs。

   - 一个 binary crate，一个 library crate。
   - 这两个名称都与 package 名相同。

5. 一个 Package 可以有多个 binary crate。
   - 文件放在 src/bin。
   - 每个文件都是单独的 binary crate。

## 定义 module 来控制作用域与私有性

1. Module 在一个 crate 内，将代码进行分组。增加代码可读性，易于服用。控制项目（item）的私有性，public、private。

2. 建立 module 需要使用 mod 关键字，其可以嵌套。

   ```rust
   //file: module_demo

   mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
            fn seat_at_table() {}
        }
        mod serving {
            fn take_order() {}
            fn serve_order() {}
            fn take_payment() {}
        }
   }
   ```

## 路径

1. 为了在 Rust 的模块中找到某个条目，需要使用路径。
2. 路径又两种形式：
   - 绝对路径：从 crate root 开始，使用 crate 名或字面值 crate。
   - 相对路径：从当前模块开始，使用 slef，super 或当前模块的标识符。
3. 路径至少由一个标识符组成，标识符之间使用 ::。

## 私有边界 privacy boundary

1. 模块不仅可以组织代码，还可以定义私有边界。
2. 如果想要把函数或 struct 等设置为私有，可以将它放到某个模块中。
3. Rust 中所有的条目（函数，方法，struct，enum，模块，常量）默认是私有的。
4. 父级模块无法访问子模块中的私有条目。
5. 子模块里可以使用所有祖先模块中的条目。

### pub 关键字

1. 使用 pub 关键字来将某些条目标记为公共的。
   - `front_of_house` 是同级别的，不标记 pub 也可以直接调用。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
```

### super 关键字

1. super：用来访问父级模块路径中的内容，类似文件系统中的 `..` 。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {
            super::hosting::add_to_waitlist();
        }
    }
}
```

### pub struct

1. pub 放在 struct 前：
   - struct 是公共的。
   - struct 中默认的字段都是私有的。
   - 在 struct 中的字段需要单独设置 pub 来变成公共的。

```rust
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // error : meal.seasonal_fruit = String::from("banana");
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
```

### pub enum

1. 与 struct 类似将 pub 放在 enum 前即可声明为公共：
   - enum 是公共的。
   - enum 的变体也都是公共的。

```rust
mod demo {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
```

## use 关键字

1. 可以使用 use 关键字将路径导入到作用域内：
   - 仍遵循私有性规则。

    ```rust
    mod use_demo {
        pub mod hosting {
            pub fn add_waitlist() {}
        }
    }

    use crate::use_demo::hosting;

    pub fn eat_somgthing() {
        hosting::add_waitlist();
    }
    ```

2. 也可以用 use 来引用相对路径。

### use 的习惯用法

1. 函数：将函数的父级模块引入作用域（指定到父级），避免模糊函数是本地定义还是外部引用进入。
2. struct、enum 等其他：指定完整路径（指定到本身）。

### as 关键字

1. as 关键字可以为引入的路径指定本地的别名。

```rust
use crate::use_demo::hosting as demoHosting;

pub fn eat_somgthing() {
    demoHosting::add_waitlist();
}
```

### 使用 pub use 重新导出名称

1. 使用 use 将路径（名称）导入到作用域后，该名称在此作用域内是私有的。
   - 使用 pub 可以将 use 的暴露出来。
   - 将条目引入到当前作用域，也可以将该条目导出被外部代码引入它们的作用域。

```rust
pub use crate::use_demo::hosting as demoHosting;

pub fn eat_somgthing() {
    demoHosting::add_waitlist();
}
```

## package

1. Cargo.toml 添加依赖的包（package）。
2. use 将特定条目引入到作用域。
3. 标准库（std）也被当作外部包：
   - 不需要修改 Cargo.toml 来包含 std。
   - 需要使用 use 将 std 中的特定条目引入当前作用域。

### 使用嵌套路径清理大量的 use 语句

1. 如果使用同一包或模块下的多个条目，可以使用嵌套路径在同一行内将上述条目进行引入：
   - 路径相同的部分 ::{路径差异部分}。

```rust
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;

use std::io::{self, Write};
```

### 通配符 \*

1. 可以使用 `*` 将路径中所有的公共条目都引用到作用域中。
   - 注意：谨慎使用。
   - 应用场景：
     - 测试：将所有被测试代码引入到 tests 模块中。
     - 优势被用于预导入模块。

```rust
use std::collections::*;
```

## 如何将模块拆分为不同的文件

1. Rust 可以将模块内容移动到其他文件，在模块定义时，如果模块名后边是 `;`，而不是代码块：
   - Rust 会从与模块同名的文件中加载内容。
   - 模块树的结构不会发生变化。
2. 随着模块逐渐变大，该技术可以把模块的内容移动到其他文件中。
