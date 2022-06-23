//和函数类似，impl 块要想实现泛型，也需要很仔细。


use std::fmt::Display;

struct S; // 具体类型 `S`
struct GenericVal<T>(T,); // 泛型类型 `GenericVal`
// GenericVal 的 `impl`，此处我们显式地指定了类型参数：
impl GenericVal<f32> {} // 指定 `f32` 类型
// `<T>` 必须在类型之前写出来，以使类型 `T` 代表泛型。
impl <T> GenericVal<T> {}


struct Val {
    val: f64
}

struct GenVal<T>{
    gen_val: T
}

// Val 的 `impl`
impl Val {
    fn value(&self) -> &f64 { &self.val }
}
// GenVal 的 `impl`，指定 `T` 是泛型类型
impl <T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}

pub fn implement_fn_test() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}






//在使用泛型时，类型参数常常必须使用 trait 作为约束（bound）来明确规定 类型应实现哪些功能。
// 例如下面的例子用到了 Display trait 来打印，所以它用 Display 来约束 T，也就是说 T 必须实现 Display。


// 定义一个函数 `printer`，接受一个类型为泛型 `T` 的参数，
// 其中 `T` 必须实现 `Display` trait。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}