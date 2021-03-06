mod literal_operator;
mod tuple;
mod slice;

use crate::primitives::literal_operator::literal_operator_fn;
use crate::primitives::slice::slice_test_fn;
use crate::primitives::tuple::tuple_fn;

pub fn primitives_fn(){

    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation

    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.  mut关键字，可修改
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
}

pub fn primitives_fn_test(){
   // literal_operator_fn();
   // tuple_fn();
    slice_test_fn();
}