use std::fs::File;
use std::io::ErrorKind;

fn ds(){
    println!("hello caspar1");
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    println!("hello caspar1");

    let f = File::open("hello.txt");
    let f=match f {
        Ok(file) =>file,
        Err(error)=> match error.kind(){
            //文件不存在，创建新文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                //创建新文件 结果枚举
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_e22rror => {
                panic!("Problem opening the file: {:?}", other_e22rror)
            }

        }
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}