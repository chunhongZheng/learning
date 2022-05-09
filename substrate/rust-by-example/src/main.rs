mod trait_folder;
mod hello_world;
mod primitives;
mod customer_type;

use crate::customer_type::customer_type_test_fn;
use crate::hello_world::format_print::format_print_fn;
use crate::primitives::primitives_fn_test;
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
    customer_type_test_fn();
}

