
// extern crate rary; // 在 Rust 2015 版或更早版本需要这个导入语句
// extern crate rary;
use crate::crate_folder::rary;

fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}