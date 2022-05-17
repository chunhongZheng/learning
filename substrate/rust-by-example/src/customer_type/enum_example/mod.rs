//enum 关键字允许创建一个代表数个可能变量的数据的类型
// （原文：The enum keyword allows the creation of a type which may be one of a few different variants.
// 若您对此句有 更好的翻译或理解，希望指出来，谢谢。）。在 struct 中任何合法的变量在 enum 同样是合法的。

mod use_example;

// 创建一个 `enum` （枚举）来划分人的类别。注意命名和类型的信息是如何一起
// 明确规定变量的：
// `Engineer != Scientist` 和 `Height(i32) != Weight(i32)`。每者都不相同且
// 相互独立。
pub enum Person {
    // 一个 `enum` 可能是个 `unit-like`（类单元结构体），
    Engineer,
    Scientist,
    // 或像一个元组结构体，
    Height(i32),
    Weight(i32),
    // 或像一个普通的结构体。
    Info { name: String, height: i32 }
}
// 此函数将一个 `Person` enum 作为参数，无返回值。

pub fn inspect(p: Person) {
    // `enum` 的使用必须覆盖所有情形（无可辩驳的），所以使用 `match`
    // 以分支方式覆盖所有类型。
    match p {
        Person::Engineer    => println!("Is engineer!"),
        Person::Scientist       => println!("Is scientist!"),
        // 从 `enum` 内部解构 `i`
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // 将 `Info` 解构成 `name` 和 `height`。
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

pub fn enum_fn_test(){
    let person   = Person::Height(18);
    let amira    = Person::Weight(10);
    // `to_owned()` 从一个字符串 slice 创建一个具有所有权的 `String`。
    let dave     = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca  = Person::Scientist;
    let rohan    = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
// enum 也可以像 C 语言枚举那样使用。
// 拥有隐式辨别值（implicit discriminator）的 enum（从0开始计数）
enum Number {
    Zero,
    One,
    Two,
    Three,
}
// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn enum_fn_test1() {
    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
