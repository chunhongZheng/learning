//和函数类似，impl 块要想实现泛型，也需要很仔细。


use std::fmt::{Debug, Display};

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


//约束的另一个作用是泛型的实例可以访问作为约束的 trait 的方法。

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}
#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }
// 泛型 `T` 必须实现 `Debug` 。只要满足这点，无论什么类型
// 都可以让下面函数正常工作。
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` 必须实现 `HasArea`。任意符合该约束的泛型的实例
// 都可访问 `HasArea` 的 `area` 函数
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn constraints_fn_test_trait() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ 试一试：取消上述语句的注释。
    // | 报错：未实现 `Debug` 或 `HasArea`。
}


// 空约束
//约束的工作机制会产生这样的效果：即使一个 trait 不包含任何功能，你仍然可以用它 作为约束。标准库中的 Eq 和 Ord 就是这样的 trait。

struct Cardinal;
struct BlueJay;
struct Turkey;
trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 这些函数只对实现了相应的 trait 的类型有效。
// 事实上这些 trait 内部是空的，但这没有关系。

fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }


pub fn constraints_fn_null_test_trait() {
    let c=Cardinal{};
    red(&c);
    let b=BlueJay{};
    blue(&b);
   // red(&b);

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // 由于约束，`red()` 不能作用于 blue_jay （蓝松鸟），反过来也一样。
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ 试一试：去掉此行注释。

}