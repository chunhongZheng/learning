//在使用泛型时，类型参数常常必须使用 trait 作为约束（bound）来明确规定 类型应实现哪些功能。
// 例如下面的例子用到了 Display trait 来打印，所以它用 Display 来约束 T，也就是说 T 必须实现 Display。

use std::fmt::{Debug, Display};

// 定义一个函数 `printer`，接受一个类型为泛型 `T` 的参数，
// 其中 `T` 必须实现 `Display` trait。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}



//  多重约束
//  多重约束（multiple bounds）可以用 + 连接。和平常一样，类型之间使用 , 隔开。

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", u);
}

pub fn multiple_bounds_test() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // 试一试 ^ 将此行注释去掉。

    compare_types(&array, &vec);
}

//where 分句
// 约束也可以使用 where 分句来表达，它放在 { 的前面，而不需写在类型第一次出现 之前。另外 where 从句可以用于任意类型的限定，而不局限于类型参数本身。

// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
//
// // 使用 `where` 从句来表达约束
// impl <A, D> MyTrait<A, D> for YourType where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF {}
//


trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
// 或者改用另一种间接的方法。
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
    // 否则我们会给出错误的约束。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}