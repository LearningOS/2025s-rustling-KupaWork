// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));//format返回String
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));//replace返回String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
    // &str 是借用（引用）：它必须指向某个已存在的、不可变的 UTF-8 字节序列。
    //format replace to_lowercase这些方法产生新数据：新字符串无法复用原字符串的内存（长度、内容均可能变化），必须分配独立内存，因此只能返回 String。

}
