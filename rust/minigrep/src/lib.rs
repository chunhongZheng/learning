
use std::error::Error;
use std::fs;
//解析参数配置
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}


pub struct Config {
    pub query: String,
    pub filename: String,
}

//创建config实例
fn parse_config2(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}


impl Config {
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let filename = args[2].clone();
    //
    //     Config { query, filename }
    // }
    //返回Result<T,E>实例
    pub fn new2(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
    //unwrap_or_else，它定义于标准库的 Result<T, E> 上。使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理
    //当 Result 是 Ok 时，这个方法的行为类似于 unwrap：它返回 Ok 内部封装的值。然而，当其值是 Err 时，该方法会调用一个 闭包（closure），
    // 也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

   // println!("With text:\n{}", contents);
}
// 返回Result<(), Box<dyn Error>> 类型数据，成功返回()，失败返回动态error类型
pub fn run2(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}




pub  fn testTryFrom(){

}