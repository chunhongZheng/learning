
pub fn literal_operator_fn(){
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
 //   println!("1 - 2 = {}", 1u32 - 2);  //无符号整数，负数会溢出
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);   // 与操作 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); //或操作  0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);//异或操作  0110
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!   下划线提高可读性
    println!("One million is written as {}", 1_000_000u32);

}