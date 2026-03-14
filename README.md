# Rust Channel Demo

## 简介

演示 Rust 消息传递 channel。

## 基本原理

mpsc: 多生产者单消费者。send() 发送，recv() 接收。

## 启动和使用

```bash
cargo run
```

## 教程

### 基本用法

```rust
let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send("hello").unwrap();
});
let msg = rx.recv().unwrap();
```

### 多个生产者

```rust
let tx2 = tx.clone();
thread::spawn(move || { tx.send("1").unwrap(); });
thread::spawn(move || { tx2.send("2").unwrap(); });
```

### 通道关闭

```rust
drop(tx);
match rx.recv() {
    Ok(msg) => println!("{}", msg),
    Err(_) => println!("通道已关闭"),
}
```
