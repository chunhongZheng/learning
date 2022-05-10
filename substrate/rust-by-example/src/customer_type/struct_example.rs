// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
// 隐藏未使用代码警告的属性。
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

//增加一个计算长方形面积的函数 rect_area（尝试使用嵌套的解构方式）。
impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        } = self;    //解构,把对应的值赋值给参数x1,x2,y1,y2

        ((x1 - x2) * (y1 - y2)).abs()   //绝对值
    }
}
//增加一个函数 square，接受的参数是一个 Point 和一个 f32，并返回一个 Rectangle（长方形），其左下角的点等于 Point 参数，长和宽都等于 f32 参数。
//长方形 对角 两个点的坐标
fn square(point: &Point, length: f32) -> Rectangle {
    Rectangle {
        p1: Point {
            x: point.x,
            y: point.y + length,
        },
        p2: Point {
            x: point.x + length,
            y: point.y,
        },
    }
}

pub fn struct_example_fn(){
    // 实例化结构体 `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };
    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用 `let` 绑定来解构 point
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };
    // 实例化一个单元结构体
    let _nil = Nil;
    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    //根据起始坐标和长度 生成新的长方形
    println!("square: {:?}", square(&Point { x: 0.0, y: 0.0 }, 1.0));


    //定义待计算长方形结构体
    let rec=Rectangle{
        p1:Point{
          x:1.0,y:2.0
        },
        p2:Point{
            x:2.0,
            y:3.0
        }
    };
    //调用计算长方形的面积函数square
    println!("rec area: {}", rec.rect_area());

}