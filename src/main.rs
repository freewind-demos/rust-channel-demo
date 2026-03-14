fn main() {
    println!("=== Rust Channel 消息传递演示 ===\n");

    use std::thread;
    use std::sync::mpsc;

    // 1. 创建 channel
    let (tx, rx) = mpsc::channel();

    // 发送消息
    thread::spawn(move || {
        let msg = "Hello from thread!";
        tx.send(msg).unwrap();
        println!("发送了: {}", msg);
    });

    // 接收消息
    let received = rx.recv().unwrap();
    println!("收到: {}", received);

    // 2. 多个生产者
    println!("\n--- 多个生产者 ---");
    let (tx2, rx2) = mpsc::channel();

    let tx3 = tx2.clone();
    thread::spawn(move || {
        tx3.send("from thread 1").unwrap();
    });

    thread::spawn(move || {
        tx2.send("from thread 2").unwrap();
    });

    // 接收所有消息
    for msg in rx2 {
        println!("收到: {}", msg);
    }

    // 3. 带有所有权的消息
    println!("\n--- 所有权转移 ---");
    let (tx4, rx4) = mpsc::channel();

    thread::spawn(move || {
        let data = vec![1, 2, 3];
        tx4.send(data).unwrap();
        // data 在这里被移动，不能再使用
    });

    let received_data = rx4.recv().unwrap();
    println!("收到数据: {:?}", received_data);

    // 4. 迭代器接收
    println!("\n--- 迭代器接收 ---");
    let (tx5, rx5) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx5.send(i).unwrap();
        }
    });

    // try_iter() 非阻塞接收
    // for msg in rx5.iter() {
    //     println!("迭代: {}", msg);
    // }

    // 5. 选择分支（使用 select）
    println!("\n--- select (通过 loop 模拟) ---");
    let (tx6, rx6) = mpsc::channel();
    let (tx7, rx7) = mpsc::channel();

    thread::spawn(move || {
        tx6.send("channel 1").unwrap();
    });

    thread::spawn(move || {
        tx7.send("channel 2").unwrap();
    });

    // 手动选择
    // 注意：标准库没有 select，需要使用 crossbeam 或第三方库
    println!("等待消息...");

    // 6. 通道类型
    println!("\n--- 通道类型 ---");

    // sync_channel 有缓冲区上限
    let (tx8, rx8) = mpsc::sync_channel(2);
    tx8.send(1).unwrap();
    tx8.send(2).unwrap();
    // tx8.send(3).unwrap(); // 会阻塞，因为缓冲区满了

    println!("sync_channel 容量: 2");

    // 7. 错误处理
    println!("\n--- 错误处理 ---");
    let (tx9, rx9) = mpsc::channel::<i32>();

    // 发送前关闭发送端
    drop(tx9);

    // recv() 返回 Result
    match rx9.recv() {
        Ok(msg) => println!("收到: {}", msg),
        Err(e) => println!("通道已关闭: {:?}", e),
    }

    println!("\n=== 总结 ===");
    println!("mpsc: 多生产者单消费者");
    println!("send() 发送消息，所有权转移");
    println!("recv() 阻塞等待，try_recv() 非阻塞");
    println!("通道关闭后 recv 返回 Err");
}
