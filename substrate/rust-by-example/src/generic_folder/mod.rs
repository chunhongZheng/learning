use crate::generic_folder::implement::implement_fn_test;

//泛型（generic）是关于泛化类型和函数功能，以扩大其适用范围的话题。
// 泛型极大地 减少了代码的重复，但它自身的语法很要求细心。也就是说，采用泛型意味着仔细地指定 泛型类型具体化时，什么样的具体类型是合法的。泛型最简单和常用的用法是用于类型参数。
mod first;
mod function;
mod implement;
mod constraint;


pub fn generic_fn_test(){
    implement_fn_test();
}