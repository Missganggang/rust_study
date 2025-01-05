fn main() {
    println!("Hello, world!");

    // 这个世界上大部分事情，你要能像坐牢一样做一年，基本都能有一个很大的突破

    // rust的词法结构
    // 默认不可变：在 Rust 中，变量默认是不可变的。这意味着一旦变量被赋值后，就不能再改变它的值，除非显式声明为可变的。
    // 可变性：使用 mut 关键字可以声明一个可变变量。
    let s1 = 100;
    let s2 ="hello world";
    println!("s1 = {}, s2 = {}", s1, s2);

    // 字符串
    let s3 = String::from("hello world");
    // 字符串切片
    let s4 ="hello";
    println!("s3 = {}, s4 = {}", s3, s4);



}
