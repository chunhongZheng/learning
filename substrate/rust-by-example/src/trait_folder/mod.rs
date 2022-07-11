use crate::trait_folder::dropTrait::drop_trait_test;
use crate::trait_folder::iteratorTrait::iterator_fn_test;
use crate::trait_folder::operatorOverload::operator_overload_fn_test;
use crate::trait_folder::trait_example::trait_example_fn_test;

pub(crate) mod derive;  //定义为pub，外部文件才可以访问到
pub(crate) mod return_trait;
pub(crate) mod trait_example;
mod dynTrait;
mod operatorOverload;
mod dropTrait;
mod iteratorTrait;
//trait 是对未知类型 Self 定义的方法集。该类型也可以访问同一个 trait 中定义的 其他方法。
//
// 对任何数据类型都可以实现 trait。在下面例子中，我们定义了包含一系列方法 的 Animal。
// 然后针对 Sheep 数据类型实现 Animal trait，因而 Sheep 的实例可以使用 Animal 中的所有方法。


pub fn trait_fn_test(){
 //   trait_example_fn_test();
    //operator_overload_fn_test();
   // drop_trait_test();
    iterator_fn_test();
}