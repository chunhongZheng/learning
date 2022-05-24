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


