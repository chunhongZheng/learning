// 隐藏未使用代码警告的属性。
#![allow(dead_code)]
enum Status {
    Rich,
    Poor,
}
enum Work {
    Civilian,
    Soldier,
}

pub fn use_fn_test(){
    // 明确地 `use` 各个名称使他们直接可用而不需要手动加上作用域。
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    use Work::*;

    // 等价于 `Status::Poor`。
    let status = Poor;
    // 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里少了作用域，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到这里没有作用域。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}