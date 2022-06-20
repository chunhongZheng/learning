//crate（中文有 “包，包装箱” 之意）是 Rust 的编译单元。
// 当调用 rustc some_file.rs 时，some_file.rs 被当作 crate 文件。
//如果 some_file.rs 里面含有 mod 声明，那么模块文件的内容将在编译之前被插入 crate 文件的相应声明处。
// 换句话说，模 块不会单独被编译，只有 crate 才会被编译。
//crate 可以编译成二进制可执行文件（binary）或库文件（library）。
// 默认情况 下，rustc 将从 crate 产生二进制可执行文件。这种行为可以通过 rustc 的选项 --crate-type 重载。
// 这个 crate 是一个库文件
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "rary"]
mod rary;
mod test;


pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
