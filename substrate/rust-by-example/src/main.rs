mod trait_folder;
mod hello_world;
mod primitives;
mod customer_type;
mod var_binding;
mod types_example;
mod conversion;
mod expression_folder;
mod loop_folder;
mod for_folder;
mod match_folder;
mod function_folder;
mod module_folder;
mod crate_folder;
mod cfg;
mod generic_folder;
mod scope_folder;

use crate::cfg::cfg_linux_test;
use crate::conversion::{from_test_fn1, from_test_fn2, from_test_fn3};
use crate::customer_type::customer_type_test_fn;
use crate::expression_folder::express_test_fn;
use crate::for_folder::{for_test_fn1, for_test_fn3, for_test_fn4};
use crate::function_folder::{function_test, function_test_main};
use crate::generic_folder::generic_fn_test;
use crate::hello_world::format_print::format_print_fn;
use crate::loop_folder::{loop_test_fn1, loop_test_fn2, loop_test_fn3};
use crate::match_folder::{match_test_age_fn, match_test_dereference_fn, match_test_fn, match_test_ifLet_fn, match_test_option_fn, match_test_struct_fn, match_test_triple_fn};
use crate::module_folder::{module_fn_main_test, module_super_test_fn, module_use_test_fn};
use crate::primitives::primitives_fn_test;
use crate::trait_folder::trait_fn_test;
use crate::types_example::{type_example_test_fn, type_example_test_fn2, type_example_test_fn3, type_example_test_fn4};
use crate::var_binding::var_binding_fn;
//此处引进traitFoler目录下的module
// use crate::trait_folder::return_trait::test_return_trait;

fn main() {

    /// traitExample main testing  start
    // Type annotation is necessary in this case.
    // let mut dolly: Sheep = Animal::new("Dolly");
    // //
    // dolly.talk();
    // dolly.shear();
    // dolly.talk();
    // // traitExample main testing  end

    // testDerive();
    //  test_return_trait();
   // format_print_fn();
    //  test_return_trait();
   // primitives_fn_test();
   // customer_type_test_fn(); //var_binding_fn();
   // type_example_test_fn();
  //  type_example_test_fn2();
  //  type_example_test_fn4();
  //  from_test_fn1();
  //  from_test_fn2();
   // from_test_fn3();
  //  express_test_fn();
  //  loop_test_fn3();
  //  for_test_fn4();
  //  match_test_fn();
  //  match_test_triple_fn();
  //  match_test_dereference_fn();
 //   match_test_age_fn();
 //   match_test_option_fn();
 //   match_test_ifLet_fn();
  //  function_test_main();
  //  function_test();
 //   module_fn_main_test();
 //   module_use_test_fn();
 //   module_super_test_fn();
 //     cfg_linux_test();
 //     generic_fn_test();
      trait_fn_test();
}

