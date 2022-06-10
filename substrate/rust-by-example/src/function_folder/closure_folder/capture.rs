// 闭包本质上很灵活，能做功能要求的事情，使闭包在没有类型标注的情况下运行。
// 这使得捕获（capture）能够灵活地适应用例，既可移动（move），又可借用（borrow）。闭包可以通过以下方式捕获变量
// 通过引用：&T
// 通过可变引用：&mut T
// 通过值：T
pub fn capture_fn_test() {
    use std::mem;

    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    //
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);
    // 使用借用来调用闭包 `color`。
    print();

    // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
    let _reborrow = &color;
    print();

    // 在最后使用 `print` 之后，移动或重新借用都是允许的。
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    let mut count2=0;
    let mut inc2 = ||{
         count2 += 1;
        println!("`count2`: {}", count2);
    };

    // 使用可变借用调用闭包
    inc();
    inc2();
    // 因为之后调用闭包，所以仍然可变借用 `count`
    // 试图重新借用将导致错误
   //  let _reborrow = &count;
    // ^ 试一试：将此行注释去掉。   cannot borrow `count` as immutable because it is also borrowed as mutable
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();   // 会报错 this value implements `FnOnce`, which causes it to be moved when called
    // ^ 试一试：将此行注释去掉。
}

//虽然 Rust 无需类型说明就能在大多数时候完成变量捕获，但在编写函数时，这种模糊写法是不允许的。
// 当以闭包作为输入参数时，必须指出闭包的完整类型，它是通过使用以下 trait 中的一种来指定的。其受限制程度按以下顺序递减：
// Fn：表示捕获方式为通过引用（&T）的闭包
// FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
// FnOnce：表示捕获方式为通过值（T）的闭包
// 译注：顺序之所以是这样，是因为 &T 只是获取了不可变的引用，&mut T 则可以改变变量，T 则是拿到了变量的所有权而非借用。

//闭包 作为输入参数

// 该函数将闭包作为参数并调用它。
fn apply<F>(f: F) where
// 闭包没有输入值和返回值。
    F: FnOnce() {
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。
    f();
}

pub fn closure_input_param_test() {
    use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
   // let double = |x| 2 * x;

  //  println!("3 doubled: {}", apply_to_3(double));
}



///闭包从周围的作用域中捕获变量是简单明了的。这样会有某些后果吗？确实有。
/// 观察一下使用闭包作为函数参数，这要求闭包是泛型的，闭包定义的方式决定了这是必要的。

// `F` 必须是泛型的。
fn apply2<F>(f: F) where
    F: FnOnce() {
    f();
}
//当闭包被定义，编译器会隐式地创建一个匿名类型的结构体，用以储存闭包捕获的变量，
// 同时为这个未知类型的结构体实现函数功能，通过 Fn、FnMut 或 FnOnce 三种 trait 中的一种。