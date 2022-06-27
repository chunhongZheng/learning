//在使用泛型时，类型参数常常必须使用 trait 作为约束（bound）来明确规定 类型应实现哪些功能。
// 例如下面的例子用到了 Display trait 来打印，所以它用 Display 来约束 T，也就是说 T 必须实现 Display。

use std::fmt::Display;

// 定义一个函数 `printer`，接受一个类型为泛型 `T` 的参数，
// 其中 `T` 必须实现 `Display` trait。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}