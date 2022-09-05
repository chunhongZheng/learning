
//Option 和 unwrap
//上个例子展示了如何主动地引入程序失败（program failure）。当公主收到蛇这件不合适 的礼物时，我们就让程序 panic。
// 但是，如果公主期待收到礼物，却没收到呢？这同样 是一件糟糕的事情，所以我们要想办法来解决这个问题！
//在标准库（std）中有个叫做 Option<T>（option 中文意思是 “选项”）的枚举 类型，用于有 “不存在” 的可能性的情况。它表现为以下两个 “option”（选项）中 的一个：
//
// Some(T)：找到一个属于 T 类型的元素
// None：找不到相应元素
//这些选项可以通过 match 显式地处理，或使用 unwrap 隐式地处理。隐式处理要么 返回 Some 内部的元素，要么就 panic。

// 平民（commoner）们见多识广，收到什么礼物都能应对。
// 所有礼物都显式地使用 `match` 来处理。
fn give_commoner(gift: Option<&str>) {
    // 指出每种情况下的做法。
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// 养在深闺人未识的公主见到蛇就会 `panic`（恐慌）。
// 这里所有的礼物都使用 `unwrap` 隐式地处理。
fn give_princess(gift: Option<&str>) {
    // `unwrap` 在接收到 `None` 时将返回 `panic`。
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}


pub fn test_optionAndUnWrap() {
    let food  = Some("chicken");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}




//使用 ? 解开 Option
//你可以使用 match 语句来解开 Option，但使用 ? 运算符通常会更容易。
// 如果 x 是 Option，那么若 x 是 Some ，对x?表达式求值将返回底层值，否则无论函数是否正在执行都将终止且返回 None。


fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // 如果 `current_age` 是 `None`，这将返回 `None`。
    // 如果 `current_age` 是 `Some`，内部的 `u8` 将赋值给 `next_age`。
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}


struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}


impl Person {

    // 获取此人的工作电话号码的区号（如果存在的话）。
    fn work_phone_area_code(&self) -> Option<u8> {
        // 没有`？`运算符的话，这将需要很多的嵌套的 `match` 语句。
        // 这将需要更多代码——尝试自己编写一下，看看哪个更容易。
        self.job?.phone_number?.area_code
    }
}


pub fn test_multi_option() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
