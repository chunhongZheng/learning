//条件编译可能通过两种不同的操作符实现：
//
// cfg 属性：在属性位置中使用 #[cfg(...)]
// cfg! 宏：在布尔表达式中使用 cfg!(...)

// 这个函数仅当目标系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当目标系统 **不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

pub fn cfg_linux_test() {
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}

//有部分条件如 target_os 是由 rustc 隐式地提供的，但是自定义条件必须使用 --cfg 标记来传给 rustc。
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    //conditional_function();
}