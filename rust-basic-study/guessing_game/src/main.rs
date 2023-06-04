fn main() {
    println!("猜数");
    println!("请输入一个数");

    let mut guess=String::new();
    // read_line 读取一行
    // expect 进行错误处理
    std::io::stdin().read_line(&mut guess)
    println!("你猜的数是:{}",guess)
}
