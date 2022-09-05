

//T 必须是实现了PartialOrd，Copy这两个trait的类型才可以
//针对slice进行所有权转移
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


//不对slice进行所有权转移

//   <T: PartialOrd>   约束，表示T类型需要实现PartialOrd trait类型
pub fn largest2<T: PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0]; //此处为&T 类型，只进行引用，并没有所有权转移
    for &item in list{
        if &item>largest{
            largest=&item;
        }
    }
    &largest

}
