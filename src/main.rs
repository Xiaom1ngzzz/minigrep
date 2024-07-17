use std::env;

fn main() {
    // 通过 env::args() 方法读取分析传入的命令行参数
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
