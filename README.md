# 项目6-协程异步操作系统

## 第一周
| 序号  | 任务  | 状态  | 结果  |
| --- | --- | --- | --- |
| 1   | 看教学视频 | 已完成 |  [《操作系统专题训练课》笔记](https://github.com/lighkLife/new-blog/blob/main/%E3%80%8A%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F%E4%B8%93%E9%A2%98%E8%AE%AD%E7%BB%83%E8%AF%BE%E3%80%8B%E7%AC%94%E8%AE%B0.md)   |
| 2   | 学习携程实现 | 已完成 | [200行Rust代码解释Futures](https://github.com/lighkLife/new-blog/blob/main/200%E8%A1%8CRust%E4%BB%A3%E7%A0%81%E8%A7%A3%E9%87%8AFutures.md) |
| 3   | 翻译 Embassy Doc | 已完成 | [Embassy 执行器](https://github.com/lighkLife/new-blog/blob/main/Embassy%E6%89%A7%E8%A1%8C%E5%99%A8.md) |

## 第二周

| 序号  | 任务  | 状态  | 结果  |
| --- | --- | --- | --- |
| 1   | 学习 Async Rust vs RTOS showndown | 已完成 |  [Async Rust vs RTOS 一决雌雄](https://github.com/lighkLife/new-blog/blob/main/Async%20Rust%20vs%20RTOS%20%E4%B8%80%E5%86%B3%E9%9B%8C%E9%9B%84.md)|
| 2   | embassy 中文在线文档翻译 | 已完成 | https://github.com/lighkLife/embassy-cn |
| 3   | embassy 项目代码标注 | 进行中 | https://github.com/lighkLife/embassy-cn/issues/8 |
| 4   | 异步驱动实现 | 进行中 | https://github.com/lighkLife/new-blog/issues/1 |

| 序号  | 任务  | 状态  | 结果  |
| --- | --- | --- | --- |
| 1   | 异步驱动实现 | 完成 | https://github.com/lighkLife/rCore-async/tree/ch9 |
| 2   | embassy 项目代码标注 | 进行中 | https://github.com/lighkLife/embassy |

遇到的问题：
embassy 运行时应该在哪里启动？ 每个线程一个运行时？ 
embassy 的任务没有返回值，需要借助 channel 通信机制或重写 embassy 运行时。
![image](https://github.com/lighkLife/new-blog/assets/7992705/9224de8b-1bf3-40e8-986c-3738879b810f)
