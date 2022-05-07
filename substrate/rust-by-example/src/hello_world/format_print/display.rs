///fmt::Debug hardly looks compact and clean, so it is often advantageous to customize the output appearance.
/// This is done by manually implementing fmt::Display, which uses the {} print marker.
/// Implementing it looks like this:

// Import (via `use`) the `fmt` module to make it available.
use std::fmt;
use std::fmt::Formatter;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

//fmt::Display 可能比 fmt::Debug 更干净，但这会给 std 库带来问题。
// 模棱两可的类型应该如何显示？ 例如，如果 std 库为所有 Vec<T> 实现了单一样式，它应该是什么样式？ 会是这两者中的任何一个吗？

//不，因为没有适用于所有类型的理想样式，而且 std 库也不假定规定一个。 fmt::Display 没有为 Vec<T> 或任何其他通用容器实现。
// fmt::Debug 然后必须用于这些一般情况。
//这不是问题，因为对于任何非通用的新容器类型，都可以实现 fmt::Display。

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}
// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn display_fn() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    //因此，fmt::Display 已实现，但 fmt::Binary 尚未实现，因此无法使用。 std::fmt 有许多这样的特性，
    // 每个特性都需要自己的实现。 这在 std::fmt 中有更详细的说明。
}
//为每个元素必须按顺序处理的结构实现 fmt::Display 是很棘手的。
// 问题是每次都写！ 生成一个 fmt::Result。 正确处理此问题需要处理所有结果。 Rust 提供 ? 运营商正是为了这个目的。

// Try `write!` to see if it errors. If it errors, return
// the error. Otherwise continue.
// write!(f, "{}", value)?;

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

//实现数组打印
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
         //   write!(f, "{0}:{1}", count,v)?;   count是坐标  v是值
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

pub fn display_list(){
    let v = List(vec![1, 2, 3]);
    println!("数组打印{}", v);
}
