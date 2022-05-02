fn main() {
    another_function(5, 6); // argument
}

// 参数需要指明类型
// parameter
fn another_function(x: i32, y: i32) {
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);

    function();
}

//函数声明语句
fn function() {
    let y = 5 + 6; //y = 绑定语句 , 5 + 6 不包括分号是表达式

    //let x = (let z = 7); err: 语句不能被赋值

    let x = 5;
    let y = {
        // {} 其中包含 x + 3 是块中的最后一个表达式，相当于返回值
        let x = 1;
        x + 3 // 如果加上分号使其变为语句 x+3; 则返回的知识一个空的 tuple ()
    };

    println!("The value of y is : {}", y)
}

fn five() -> i32 {
    5 // 返回最后一个表达式 5
}

fn plus_five(x: i32) -> i32 {
    x + 5 // 返回表达式 x+5 的值，加上分号就变成语句了
}

