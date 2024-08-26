# mingrep

来源于《Rust语言圣经》一书中的 入门实战部分的示例。

## 示例

> cargo run -- searchstring example-filename.txt

```rs
// in main.rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

通过 `use` 引入标准库中的 `env` 包，使用 `env::args` 方法读取并分析传入的命令行参数，最终通过`collecti` 方法输出一个集合类型。

- `std::env::args()`：只能读取分析Unicode字符；
- `std::env::args_os()`：可以读取分析非Unicode字符；

## clone的得与失

clone：直接完整的复制目标数据，无需被所有权、借用等问题所困惑，但是它也有缺点，就是性能的损耗。

判断是否使用clone

- 是否严肃的项目；
- 要看所在的代码路径是否是热点路径（hot path），例如执行次数较多的显然就是热点路径，热点路径就值得去使用性能更好的实现方式；


