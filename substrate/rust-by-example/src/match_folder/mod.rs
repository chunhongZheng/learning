//Rust 通过 match 关键字来提供模式匹配，和 C 语言的 switch 用法类似。第一个匹配分支会被比对，并且所有可能的值都必须被覆盖。



pub fn match_test_fn() {
    let number = 13;
    // 试一试 ^ 将不同的值赋给 `number`

    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11|13 => println!("This is a prime"),
        // 试一试 ^ 将 13 添加到质数列表中
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Ain't special"),   //不能注释其他情况的  区配必须是详尽的，
        // 试一试 ^ 注释掉这个总括性的分支
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // 试一试 ^ 将其中一条分支注释掉
    };

    println!("{} -> {}", boolean, binary);
}

//解构元组
pub fn match_test_triple_fn() {
    let triple = (7, -2, 3,);
    // 试一试 ^ 将不同的值赋给 `triple`

    println!("Tell me about {:?}", triple);
    // match 可以解构一个元组
    match triple {
        // 解构出第二个和第三个元素
        //如triple为(0, -2, 3) 此选项执行
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        //如triple为(5, -2, 3) 此选项执行
        (5, y, z) => println!("First is `5`, `y` is {:?}, and `z` is {:?}", y, z),
        //如triple为(1, -2, 3) 此选项执行
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        // `..` 可用来忽略元组的其余部分
        //如triple为(3, -2, 3) 此选项执行
        _=> println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}
//解构枚举
// 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
#[allow(dead_code)]
enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定。
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}


pub fn match_test_enum_fn() {
    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}

//对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念是不同的，和 C 那样的语言用法不一样。

//   解引用使用 *
//   解构使用 &、ref、和 ref mut


pub fn match_test_dereference_fn() {
    // 获得一个 `i32` 类型的引用。`&` 表示取引用。
    let reference=&4;
    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
        &val => println!("Got a value via destructuring: {:?}", val),
        //&val => println!("Got a value via destructuring: {:?}", &val), 结果也是4
    }

    // 如果不想用 `&`，需要在匹配前解引用。
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
    // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
    let _not_a_reference = 3;

    // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用。
    let ref _is_a_reference = 3;

    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字来创建引用。
    // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
    // 引用。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}

//解构struct
struct Foo {
    x: (u32, u32),
    y: u32
}
pub fn match_test_struct_fn() {


    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // 可以解构结构体并重命名变量，成员顺序并不重要

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 也可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 这将得到一个错误：模式中没有提及 `x` 字段
    // let Foo { y } = foo;
}

//guard语句：卫语句
//可以加上 match 卫语句（guard） 来过滤分支。

pub fn match_test_guard_fn() {
 //   let pair1 = (2, 56.4);
    let pair = (2, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ `if` 条件部分是一个卫语句
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
//绑定
//在 match 中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。match 提供了 @ 符号来绑定变量到名称：
// `age` 函数，返回一个 `u32` 值。
fn age() -> u32 {
    25
}
pub fn match_test_age_fn() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        a@20 ..=30 =>println!("I'm a teen of age {:?}", a),
        // 不符合上面的范围。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }
}




//使用绑定来“解构” enum 变体
fn some_number() -> Option<u32> {
  //  Some(42)
    None
}
pub fn match_test_option_fn(){
    match some_number() {
        // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 匹配任意其他数字。
        Some(n)      => println!("Not interesting... {}", n),
        // 匹配任意其他值（`None` 可变类型）。
        _            => (),
    }
}
// // 将 `optional` 定为 `Option<i32>` 类型
// let optional = Some(7);
//
// match optional {
// Some(i) => {
// println!("This is a really long string and `{:?}`", i);
// // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
// // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
// },
// _ => {},
// // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
// };

// if let

pub fn match_test_ifLet_fn(){
    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。切换到失败情形。
        println!("Didn't match a number. Let's go with a letter!");
    };
    // 提供另一种失败情况下的条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}