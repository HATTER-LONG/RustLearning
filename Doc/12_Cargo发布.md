# Cargo 与 crates.io

- [Cargo 与 crates.io](#cargo-与-cratesio)
  - [前言](#前言)
  - [通过 release profile 来自定义构建](#通过-release-profile-来自定义构建)
    - [自定义 profile](#自定义-profile)
  - [发布库到 crates.io](#发布库到-cratesio)
    - [文档注释](#文档注释)
    - [常用章节](#常用章节)
    - [文档注释作为测试](#文档注释作为测试)
    - [为包含注释的项添加文档注释](#为包含注释的项添加文档注释)
  - [pub use](#pub-use)
  - [发布 crate](#发布-crate)
  - [Cargo 工作空间](#cargo-工作空间)
    - [创建工作空间](#创建工作空间)
    - [在工作空间中依赖外部 crate](#在工作空间中依赖外部-crate)

## 前言

本章主要内容：

1. 通用 release profile 来自定义构建。
2. 尝试在 crates.io 上发布库。
3. 通过 workspaces 组织大工程。
4. 从 crates.io 安装库。
5. 使用自定义命令扩展 cargo。

## 通过 release profile 来自定义构建

- release profile 即发布配置文件：
  - 其是预定义大。
  - 可自定义：可使用不同的配置，对代码编译拥有更多控制。
- 每个 profile 的配置都是相互独立的。
- Cargo 主要的两个 profile：
  - dev profile：适用于开发，即执行 cargo build 时使用。
  - release profile：适用于发布，执行 cargo build --release 时使用。

### 自定义 profile

- 针对每个 profile，Cargo 都提供来默认的配置。
- 如果想要自定义 xxx profile 的配置：
  - 可以在 Cargo.toml 里添加 [profile.xxx] 区域，在里面覆盖默认配置子集。
  - 对于每个配置的默认值和完整选项，详见 [官方文档](https://doc.rust-lang.org/cargo/reference/profiles.html)。

```toml
[profile.dev]
opt-level = 1 # 决定编译优化等级，0 - 3

[profile.release]
opt-level = 3
```

## 发布库到 crates.io

- 可以通过发布包来共享你的代码。
- crate 的 [注册表](https://crates.io/)：
  - 他会分发已注册的包的源代码。
  - 主要托管开源的代码。

### 文档注释

- 文档注释：用于生成文档。
  - 生成 HTML 文档。
  - 显示公共 API 的文档注释，例如如何使用 API。
  - 使用 `///` 来表示。

````rust
/// Adds one to the number given.
///
/// # example
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32{
    x + 1
}
````

- cargo doc：
  - 它会运行 rustdoc 工具，默认会安装。
  - 它会把生成的 HTML 文档放在 target/doc 目录下。
- cargo doc -open：
  - 构建当前 crate 文档（也包含 crate 依赖项的文档）。
  - 在浏览器打开文档。

### 常用章节

- `# Example`：例子，上边的源码已经展示了。
- 其他常用章节：
  - `# Panics`：函数可能发生 panic 的场景。
  - `# Errors`：如果函数返回 Result，描述可能的错误种类，以及可导致错误的条件。
  - `# Safety`：如果函数处于 unsafe 调用，经应该解释函数 unsafe 的原因，以及调用者确保的使用前提。

### 文档注释作为测试

- 示例代码块的附加值：
  - 运行 cargo test：将把文档注释中的示例代码作为测试来运行。

### 为包含注释的项添加文档注释

- 符号：`//!` 为包裹当前注释的外层条目添加文档注释。
  - 这类注释通常描述 crate 和模块：
    - crate root（按惯例 src/lib.rs）。
    - 一个模块内，将 crate 或模块作为一个整理进行记录。

````rust
//! # My Crate
//!
//! `my_crate` is collection of utilities to make proforming certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # example
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32{
    x + 1
}
````

## pub use

- 使用 pub use 导出方便使用的公共 API。
- 问题：crate 的程序结构在开发时对于开发者很合理，但对于它的使用者不够方便。
  - 开发者会把程序结构分为很多层，使用者想要找到这种深层结构中的某个类型很费劲。
  - 例如：`my_crate::some_module::another_module::UsefulType`，用户想要使用就比较麻烦。
  - 方便的形式 `my_crate::UsefulType`。
- 解决方法：
  - 不需要重新组织内部代码结构。
  - 使用 pub use：可以重新导出，创建一个与内部私有结构不同的对外公共结构。

```rust
//lib.rs

//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}

//main.rs
// use art::kinds::PrimaryColor;
// use art::utils::mix;
use art::mix;
use art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

- 重新导出后的效果如下：

![](https://fastly.jsdelivr.net/gh/HATTER-LONG/Resource@main/images/RustLearning/20220604112537.png)

## 发布 crate

- 暂略。

// TODO 待完善。

## Cargo 工作空间

- cargo 工作空间：帮忙管理多个相互关联且需要协同开发的 crate。
- cargo 工作空间是一套共享同一个 Cargo.lock 和输出文件夹的包。

### 创建工作空间

- 有多种方式来组件工作空间：例如，1 个二进制 crate，2 个库 crate：

  - 二进制 crate：main 函数，依赖于其他两个 crate。
  - 其中一个 crate 提供 `add_one` 函数。
  - 另外一个 crate 提供 `add_two` 函数。

- 添加 workspace.members：

```toml
[workspace]

members = ["adder", "add_one"]
```

- 创建 adder 子模块，文件目录结构如下，产物是在 workspace 根：

```text
❯ tree -I target
.
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── adder
    ├── Cargo.toml
    └── src
        └── main.rs

4 directories, 6 files
```

- adder 需要依赖 add-one：
  - 在 adder 的 Cargo.toml 依赖中添加。

```toml
[dependencies]

add-one = { path = "../add-one" }
```

### 在工作空间中依赖外部 crate

- 工作空间只有一个 Cargo.lock 文件，在工作空间的顶层目录。
  - 保证工作空间内所有 crate 使用的依赖的版本都相同。
  - 工作空间内所有的版本都是相兼容的。
