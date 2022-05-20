//From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。
// 在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换功能。
use std::convert::From;
#[derive(Debug)]
struct Number {
    value: i32,
    v: u8
}

impl From<i32> for Number{
    fn from(item: i32) -> Self {
        Number { value: item,v:3u8 }
    }
}
//Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
//
// 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。不过考虑到我们免费获得了 Into，这点代价不值一提。
pub fn from_test_fn1() {
    let num = Number::from(30);
    //My number is Number { value: 30 }
    println!("My number is {:?}", num);

    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
//类似于 From 和 Into，TryFrom 和 TryInto 是 类型转换的通用 trait。
//不同于 From/Into 的是，TryFrom 和 TryInto trait 用于易出错的转换，也正因如此，其返回值是 Result 型。
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber{
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn from_test_fn2(){
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(8), Err(()));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

pub fn from_test_fn3(){
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}

