# Cargo

[返回](../README.md)

cargo 是 rust 的包(crates)管理工具

使用`cargo init --bin 项目名`或者`cargo new 项目名`可以创建一个默认的项目<br>
_官方解释是，init 从已有文件开始，new 可以创建文件，实际上没看出区别_

1. `Cargo.toml`和`package.json`效果相同
2. `cargo install xx`和`pip install xx`效果相同
3. `cargo run`和`npm run`效果相同
4. `cargo build`和`npm build`相同
5. `cargo build --release`会构建 release 包
6. `cargo doc`会生成文档注释（`/// 注释`或`//! crate/mod注释`）的 HTML，`--open`直接打开看

## 安装依赖

1. 打开[crates.io](https://crates.io)
2. 搜索想要的依赖
3. 在`Cargo.toml`中的 dependences 下添加`依赖="依赖版本号"`，等同于在`package.json`中添加`依赖:"^依赖版本号"`
4. 手动或自动地运行`cargo build`

## crate

- crate 是一个二进制或库项目
- 若有`src/main.rs`，则是一个二进制 crate；若有`src/lib.rs`，则是一个库 crate
- crate 根(crate root)是一个用来描述如何构建 crate 的文件(main 或 lib)
- 带有 Cargo.toml 文件的包用以描述如何构建一个或多个 crate
- 包可以带有多个二进制 crate，需将其文件置于 src/bin 目录，每个文件将是一个单独的二进制 crate
- 一个包中**至多**可以有一个*库*项目

## 工作空间

根空间的 Cargo.toml

```toml
[workspace]
members = [
  "xx",
  "yy",
]
```

然后在工作空间下运行`cargo new xx`

之后每个项目的 dependences

```toml
[dependences]
yy = { path="../yy" }
```

单独运行/测试某个 crate (及其依赖)，`cargo run -p yy`

> 如果在 `Cargo.toml` 和 `xx/Cargo.toml` 中都增加 rand crate，则 Cargo 会将其都解析为同一版本并记录到唯一的 `Cargo.lock` 中，目的是为了让每个 crate 的依赖互相兼容
