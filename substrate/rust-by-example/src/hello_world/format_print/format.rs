///格式化显示
/// format!("{}", foo) -> "3735928559"
// format!("0x{:X}", foo) -> "0xDEADBEEF"
// format!("0o{:o}", foo) -> "0o33653337357"
//这种格式化功能是通过特征实现的，并且每种参数类型都有一个特征。 最常见的格式化特征是 Display，它处理未指定参数类型的情况：例如 {}。


use std::fmt::{self, Formatter, Display, UpperHex};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}
impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)

        //{:.2} 精确到小数点后两位，如真实值为0.2，则显示为 0.20。 {:.3}则显示为0.200
        write!(f, "{}: {:.2}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
//添加color自定义显示方法

//RGB (128, 255, 90) 0x80FF5A
// RGB (0, 3, 254) 0x0003FE
// RGB (0, 0, 0) 0x000000
impl Display for Color{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // write!(f, "RGB(");
        // write!(f,"{},{},{}",self.red,self.green,self.blue);
        // write!(f, ")")
        write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)
    }
}
//UpperHex是为了按照大写的格式输出十六进制，用格式化符号{:X}来表示。{:02X}表示输出的长度为2，如果不足两位，则补0。
impl UpperHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

pub fn format_fn(){
  let c=City{
      name: "广州",
      lat: 0.2,
      lon: 0.5
  };
    ///   \n 换行符
    println!("打印City结构体：\n{}", c);
}
//复杂显示
pub fn format_comflex_display(){
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", *color);
    }
    // 16进制打印显示
    for color in [
        Color {red: 128, green: 255, blue: 90},
        Color {red: 0, green: 3, blue: 254},
        Color {red: 0, green: 0, blue: 0},
    ].iter() {
        //{}调用的是impl Display for Color，0x{:X} 调用的是impl UpperHex for Color
        println!("{} 0x{:X}", *color, color)
      //  println!("{} 0o{:o}", *color, color)  //编译报错，因为未实现8进制的方法

    }

}
//打印结果
// Dublin: 53.35°N 6.260°W
// Oslo: 59.95°N 10.750°E
// Vancouver: 49.25°N 123.100°W
// Color { red: 128, green: 255, blue: 90 }
// Color { red: 0, green: 3, blue: 254 }
// Color { red: 0, green: 0, blue: 0 }