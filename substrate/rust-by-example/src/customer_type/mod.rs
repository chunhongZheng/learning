mod struct_example;
mod enum_example;
mod constant_example;

use crate::customer_type::constant_example::constant_example_fn;
use crate::customer_type::enum_example::{enum_fn_test, enum_fn_test1, linked_list_fn_test};
use crate::customer_type::struct_example::struct_example_fn;


pub unsafe fn customer_type_test_fn(){
  //  struct_example_fn();
  //  enum_fn_test();
 //   enum_fn_test1();
  //  linked_list_fn_test();
    constant_example_fn();
}