## 处理并发编程的各种方法

### OS Thread

**优点:**

- 简单、易用
- 任务之间切换相当快
- 天生并行

**缺点:**

- 每个线程都有一个相当大的栈。任务很多时，容易耗尽内存（web-server）
- 涉及许多系统调用。任务比较多时，这么多的系统调用就会很昂贵。
- OS 有许多事情要处理。不能如你期望的那样及时切换回你的线程。
- 某些系统不支持多线程。

### Green threads

绿色线程是**一种由运行环境或虚拟机调度，而不是由本地底层操作系统调度的线程**。 绿色线程并不依赖底层的系统功能，模拟实现了多线程的运行，这种线程的管理调配发生在用户空间而不是内核空间，所以它们可以在没有原生线程支持的环境中工作。

**优点:**

- 使用简单。就像使用 OS thread。
  
- 上下文切换相当快。（只需要切换关键寄存器即可）
  
- 每个栈开始只占用很小的内存，因此可以运行成百上千的绿色线程。
  
- 很容易实现抢占调度，因为调度的控制器交给了运行时实现。
  

**缺点:**

- 栈可能需要增长
  
- 每次切换任务都需要保存CPU状态。
  
- 不是零成本抽象
  
- 如果要支持多平台，正确实现很复杂
  

### 基于 Callback 的方式

**优点:**

- 容易实现
  
- 没有上下文切换
  
- 内存开销相对较低
  

**缺点:**

- 每个任务必须保存稍后需要的状态，内存使用量随着回调链的数量线新增长。
  
- 阅读代码逻辑困难，众所周知的回调地狱问题。
  
- 编写方式与常规代码不同。
  
- 由于 Rust 所有权问题，不同任务之间难以共享状态。
  

### Promises

`promises`、`futures`和其他延迟计算的名称经常 interchangeably 。

**Js 的领土Promises 和 Rust 的 future有区别**

- JS 中的 Promises 是 early evaluated，一旦它被创建，他就开始运行一个任务；
  
- RUST 中的 Future 是的lazy evaluated，除非 Poll一次，否则什么都不会发生；
  

## Rust 中的 Futures

Rust Async 使用 Poll 机制，整个过程可划分为三个阶段：

| 阶段  | 管理者 | 操作  |
| --- | --- | --- |
| Poll | executor | Poll futures to make progress, 直到不能继续 make progress |
| Wait | reactor | reactor 注册等待一个事件发生，并确保当该事件准备好时唤醒相应的Future |
| Wake | executor | 事件发生,相应的Future被唤醒。 executor 调度Future再次被轮询，并向前走一步，直到它完成或达到一个阻塞点，不能再向前走, 如此往复,直到最终完成 |

-  **leaf futures** ：代表访问一个资源（如socket）
  
- **Non-leaf-futures**： 用户使用 async 标记的代码块

**Rust 标准库干了哪些活？**

1. 定义 Future 接口，标识一个未来完成的操作。
  
2. 使用 async、await 生成一个可以挂起、恢复的任务。
  
3. Waker 接口，可以唤醒挂起的任务。

## Rust 中的 Waker

**Example `&[i32]` :**

- The first 8 bytes is the actual pointer to the first element in the array (or part of an array the slice refers to)
- The second 8 bytes is the length of the slice.

**Example `&dyn SomeTrait`:**

This is the type of fat pointer we'll concern ourselves about going forward. `&dyn SomeTrait` is a reference to a trait, or what Rust calls a *trait object*.

The layout for a pointer to a *trait object* looks like this:

- The first 8 bytes points to the `data` for the trait object
- The second 8 bytes points to the `vtable` for the trait object

Waker 就是使用了运行是动态分发的机制，具体就是使用`vtable`, 这个vtable允许我们使用动态方式调用我们真实的Waker实现
比如
```rust
// A reference to a trait object is a fat pointer: (data_ptr, vtable_ptr)
trait Test {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

// This will represent our home brewn fat pointer to a trait object
#[repr(C)]
struct FatPointer<'a> {
    /// A reference is a pointer to an instantiated `Data` instance
    data: &'a mut Data,
    /// Since we need to pass in literal values like length and alignment it's
    /// easiest for us to convert pointers to usize-integers instead of the other way around.
    vtable: *const usize,
}

// This is the data in our trait object. It's just two numbers we want to operate on.
struct Data {
    a: i32,
    b: i32,
}

// ====== function definitions ======
fn add(s: &Data) -> i32 {
    s.a + s.b
}
fn sub(s: &Data) -> i32 {
    s.a - s.b
}
fn mul(s: &Data) -> i32 {
    s.a * s.b
}

fn main() {
    let mut data = Data {a: 3, b: 2};
    // vtable is like special purpose array of pointer-length types with a fixed
    // format where the three first values has a special meaning like the
    // length of the array is encoded in the array itself as the second value.
    let vtable = vec![
        0,            // pointer to `Drop` (which we're not implementing here)
        6,            // lenght of vtable
        8,            // alignment

        // we need to make sure we add these in the same order as defined in the Trait.
        add as usize, // function pointer - try changing the order of `add`
        sub as usize, // function pointer - and `sub` to see what happens
        mul as usize, // function pointer
    ];

    let fat_pointer = FatPointer { data: &mut data, vtable: vtable.as_ptr()};
    let test = unsafe { std::mem::transmute::<FatPointer, &dyn Test>(fat_pointer) };

    // And voalá, it's now a trait object we can call methods on
    println!("Add: 3 + 2 = {}", test.add());
    println!("Sub: 3 - 2 = {}", test.sub());
    println!("Mul: 3 * 2 = {}", test.mul());
}
```
