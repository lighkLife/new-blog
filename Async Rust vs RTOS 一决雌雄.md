原文：https://tweedegolf.nl/en/blog/65/async-rust-vs-rtos-showdown

## 概述
这篇文章主要将 Embassy/Rust 和 FreeRTOS/C 性能和资源占用对比，硬件环境为 STM32F446 微控制器，主频180Mhz。最后还追加了 RTIC 的对比数据；

## Rust异步

Rust 中的的异步函数只是一个函数语法糖，返回结果为 Future 结构体。

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

这个异步函数会被编译器转换为一个支持轮询（Poll）的状态机对象。这个状态机可以让代码跳转到之前停止的地方，继续执行。 并且它可以追踪到等待点保留的所有变量。

Rust 的 Futures 是惰性的，它只在被轮询（Poll）时执行。为了完成一个任务，执行器可以一直调用 `Poll`函数，直到它停止，并返回一个 `Pending` 或`Ready(OutPut)`状态。

但是啥也不管，一直调用`Poll`函数效率很低，可能每次得到结果都是`Pending`（没有拿到等待的资源）。于是就需要提到 Waker 唤醒机制了。

等待资源就绪时，Rust 使用 Waker 来通知执行器可以调用 Poll 了。这个 Waker 可以被 Futures 自己调用（感觉这样就等于一直轮询），也可以由 Futures 等待的那个进程/线程调用（也就是资源就绪时，对方线程来调用我们的Waker进行通知）。总而言之，执行器只有但Waker被触发时，才调用 `Poll`函数。

## Embbassy

Embbassy 是 Rust 实现的现代嵌入式框架，实现了啊RUST 异步执行器、硬件抽象层（HAL）、网络、蓝牙、Lora、USB、Bootloader 和 DFU（Device Firmware Upgrade Mode）。

Embbassy 使用了Rust这种异步机制，但额外增加一些限制：

- 任务必须被静态分配。不依赖分配器，所有任务必须在编译期间确定。
  
- 需要`nightly`版本的编译器。需要使用预览特性`type_alias_impl_trait`。
  

## RTOS

RTOS 是实时操作系统。实时操作系统将所有不同的事情都放在独立的线程中。切换线程时，需要保存、恢复处理器上下文。这种设计适合实现抢占式线程模型。

## 测试程序

- 每 200ms 点亮 LED 100ms；
  

- 追踪用户按钮
  
- 向串口输出消息
  

## Embbassy/Rust 与 RTOS/C 测试结果

| Test | C   | Rust | Difference | Difference % |
| --- | --- | --- | --- | --- |
| 中断时间 (avg) | 2.962us | 1.450us | -1.512us | -51.0% |
| 中断时间 (stddev) | 124.8ns | 4.96ns | -119.84ns | -96.0% |
| 线程时间(avg) | 16.19us | 11.64us | -4.55us | -28.1% |
| 线程时间 (stddev) | 248.2ns | 103.0ns | -145.2ns | -56.2% |
| 中断延迟(avg) | 4.973us | 3.738us | -1.235us | -24.8% |
| 中断延迟(stddev) | 158.0ns | 45.3ns | -112.7ns | -71.3% |
| 程序大小 | 20676b | 14272b | -6404b | -31.0% |
| 静态内存大小 | 5480b | 872b | -4608b | -84.1% |

无论从均值，还是方差来看，Embassy/Rust 在各个方面都胜出。

## 附加 RTIC 测试结果

rtic 是实时中断驱动并发框架（Real-Time Interrupt-driven Concurrency），但目前仅限于 `ARM Cortex-M`微处理器。

| Test | RTIC | Embassy | Difference | Difference % |
| --- | --- | --- | --- | --- |
| 中断时间 (avg) | 650.8ns | 1450ns | 799ns | 122.8% |
| 中断时间 (stddev) | 10.34ns | 4.96ns | -5.38ns | -52.0% |
| 线程时间 (avg) | 7.807us | 11.64us | -3.83us | 49.1% |
| 线程时间 (stddev) | 279.9ns | 103.0ns | -176.9ns | -63.2% |
| 中断延迟 (avg) | 1.184us | 3.738us | 2.554us | 215.7% |
| 中断延迟 (stddev) | 77.75ns | 45.3ns | -32.45ns | -41.7% |
| 程序大小 | 8888b | 14272b | 5384b | 60.0% |
| 静态内存大小 | 392b | 872b | 480b | 122.4% |

可以看到 RTIC 的均值表现更佳，但方差不及 Embassy，也就是说Embassy表现更加稳定。

## 总结

Embbassy 使用 RUST 的异步语法，自己实现了异步运行时，可以很方便的在嵌入式中使用异步操作。在多任务并发执行、任务间共享数据、中断响应的场景下，Embbassy 性能要比RTOS/C 更优秀，占用资源也更少。

个人推测 RTOS的任务切换需要涉及系统调用、处理器上下文保存与恢复等操作，这些额外操作占用了处理器时间，而Embbassy的任务切换只需要切换处理器PC等关键寄存器，配合 waker机制，避免了不必要的轮询，节约了处理器时间。

RTIC 表项优于 Embbassy， 但是RTIC仅限于 `ARM Cortex-M`微处理器，而 Embbassy 没有平台依赖性，适用范围更广。
