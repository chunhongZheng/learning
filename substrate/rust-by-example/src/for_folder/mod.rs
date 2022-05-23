
//for in 结构可以遍历一个 Iterator（迭代器）。
// 创建迭代器的一个最简单的方法是使用区间标记 a..b。这会生成从 a（包含此值） 到 b（不含此值）的，步长为 1 的一系列值。

pub fn for_test_fn1() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

//或者，可以使用a..=b表示两端都包含在内的范围。上面的代码可以写成：
fn for_test_fn2() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

//for in 结构能以几种方式与 Iterator 互动。
// 如果没有特别指定，for 循环会对给出的集合应用 into_iter 函数，把它转换成一个迭代器。
// 这并不是把集合变成迭代器的唯一方法，其他的方法有 iter 和iter_mut 函数。

pub fn for_test_fn3() {
    let mut names = vec!["Bob", "Frank", "Ferris","caspar"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
//iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
pub fn for_test_fn4(){
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
    //结果:  names: ["Hello", "Hello", "There is a rustacean among us!"]
}