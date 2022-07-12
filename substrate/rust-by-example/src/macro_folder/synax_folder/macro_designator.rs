//宏的参数使用一个美元符号 $ 作为前缀，并使用一个指示符（designator）来 注明类型：


macro_rules! create_function {
    //此宏接受一个'ident'指示符表示的参数，并创建一个名称 '$func_name'的函数.
    //'ident'指示符用于变量名或者函数名
    ($func_name:ident) =>(
        fn $func_name(){
            // 'stringify!'宏把'ident'转换成字符串.
            println!("You called {:?}()",stringify!($func_name))
        }
    );
}
//借助上述宏来创建名为 'foo'和 'bar' 的函数

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // 此宏接受一个 'expr'类型的表达式，并将它作为字符串，连同其结果一起打印出来
        ($expression:expr) => (
        // `stringify!` 把表达式*原样*转换成一个字符串。
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression)
    )
}

pub fn macro_designator_test(){
    foo();
    bar();

    print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式！
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}