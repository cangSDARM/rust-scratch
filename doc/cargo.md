# Cargo
[return](../README.md)

cargo是rust的包(crates)管理工具

使用`cargo init --bin 项目名`或者`cargo new 项目名`可以创建一个默认的项目<br>
  *官方解释是，init从已有文件开始，new可以创建文件，实际上没看出区别*

1. `Cargo.toml`和`package.json`效果相同
2. `cargo run`和`npm run`效果相同
3. `cargo build`和`npm build`相同
4. `cargo build --release`会构建release包

## 安装依赖
1. 打开[crates.io](https://crates.io)
2. 搜索想要的依赖
3. 在`Cargo.toml`中的dependences下添加`依赖="依赖版本号"`，等同于在`package.json`中添加`依赖:"^依赖版本号"`
4. 手动或自动地运行`cargo build`

## crate
- crate 是一个二进制或库项目
- 若有`src/main.rs`，则是一个二进制crate；若有`src/lib.rs`，则是一个库crate
- crate 根(crate root)是一个用来描述如何构建 crate 的文件(main或lib)
- 带有 Cargo.toml 文件的包用以描述如何构建一个或多个 crate
- 包可以带有多个二进制 crate，需将其文件置于 src/bin 目录，每个文件将是一个单独的二进制 crate
- 一个包中**至多**可以有一个*库*项目