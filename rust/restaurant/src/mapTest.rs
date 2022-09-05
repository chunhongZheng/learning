use std::collections::HashMap;

pub fn mapte(){
    let mut scores = HashMap::new();
    //scores.insert("Blue", 10);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    }

//vec! 宏，这个宏会根据我们提供的值来创建一个新的 vector




fn own_test() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}



pub fn own1_test(){
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);    //所有权被移动到map了
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
  //  println!("The length of '{}' is {}.", field_name, field_value);
}


pub fn own2_test(){
    let field_name2 = String::from("Favorite color 引用");
    let field_value2 = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name2, &field_value2);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    println!("The length of '{}' is {}.", field_name2, field_value2);
}