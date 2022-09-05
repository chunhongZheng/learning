//我们将要看到的最简单的错误处理机制就是 panic。它会打印一个错误消息，开始 回退（unwind）任务，且通常会退出程序。

fn give_princess(gift: &str) {
    // 公主讨厌蛇，所以如果公主表示厌恶的话我们要停止！
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}
pub fn panic_fn_test(){
    give_princess("teddy bear");
    give_princess("snake");
}


//  CMake是一个跨平台的安装（编译）工具，可以用简单的语句来描述所有平台的安装(编译过程)