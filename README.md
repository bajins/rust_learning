# rust_learning
rust 学习


## Rust更换国内(清华)镜像源


**创建以下路径的文件**

```bash
# Linux
~/.cargo/config
# Windows
%USERPROFILE%/.cargo/config
```


**在文件中添加以下配置**

```conf
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```