//Rust 不提供原生类型之间的隐式类型转换（coercion），但可以使用 as 关键字进行显 式类型转换（casting）。

//整型之间的转换大体遵循 C 语言的惯例，除了 C 会产生未定义行为的情形。在 Rust 中所 有整型转换都是定义良好的。

// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

pub fn type_example_test_fn(){
    let decimal = 65.4321_f32;
    // 错误！不提供隐式转换
   // let integer: u8 = decimal;
    // 改正 ^ 注释掉这一行
    // 可以显式转换
    let integer = decimal as u8;

    let character = integer as char;   // ASCII码
    //  Casting: 65.4321 -> 65 -> A
    println!("Casting: {} -> {} -> {}", decimal, integer, character);


    // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
    // 直到值位于新类型 T 的范围内。


    // 1000 已经在 u16 的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8 is: {}", 1000 as u8);
    // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
    // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
    // 译注：MSB 就是二进制的最高位，LSB 就是二进制的最低位，按日常书写习惯就是
    // 最左边一位和最右边一位。
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对正数，这就和取模一样。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
    // 如果 MSB 是 1，则该值为负” 是一样的。
    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 重复之前的例子
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}


//对数值字面量，只要把类型作为后缀加上去，就完成了类型说明。比如指定字面量 42 的 类型是 i32，只需要写 42i32。
//
// 无后缀的数值字面量，其类型取决于怎样使用它们。如果没有限制，编译器会对整数使用 i32，对浮点数使用 f64。

pub fn type_example_test_fn2(){
    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    // 无后缀的字面量，其类型取决于如何使用它们。
    let i = 1;
    let f = 1.0;
    // `size_of_val` 返回一个变量所占的字节数  一个字节8位
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));  //  1个字节
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));  // 4个字节
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));  // 4个字节
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));  // 4个字节
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));  // 8个字节
}

pub fn type_example_test_fn3(){
    // 因为有类型说明，编译器知道 `elem` 的类型是 u8。
    let elem = 5u8;
    // 创建一个空向量（vector，即不定长的，可以增长的数组）。
    let mut vec = Vec::new();
    // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）
    // 在向量中插入 `elem`。
    vec.push(elem);
    vec.push(8);
    // 啊哈！现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）。
    // 试一试 ^ 注释掉 `vec.push(elem)` 这一行。

    println!("{:?}", vec);   //[5, 8]

    println!("{:#?}", vec);
    // [
    //     5,
    //     8,
    // ]
}

//可以用 type 语句给已有的类型取个新的名字。类型的名字必须遵循驼峰命名法（像是 CamelCase 这样），否则编译器将给出警告。原生类型是例外，比如： usize、f32，等等。
// `NanoSecond` 是 `u64` 的新名字。
type NanoSecond = u64;
type Inch = u64;
// 通过这个属性屏蔽警告。
#[allow(non_camel_case_types)]
type u64_t = u64;
pub fn type_example_test_fn4(){
    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);


}