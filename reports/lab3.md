# lab3

首先完成sys_get_time(),sys_mmap(),sys_munmap()函数的迁移。与之前相比，进程管理数据结构进行了调整：
- 任务控制块TaskControlBlock：表示进程的核心数据结构
- 处理器管理结构Processor：用于进程调度，维护进程的处理器状态。通过Processor结构找到当前进程的MemorySet（current_task()），然后执行相应的mmap、munmap操作。

进程管理设计到很多前面章节涉及到的内容：
- [TrapContext](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter2/4trap-handling.html)：Trap发生时需要保存的物理资源内容
    ```rust

    // os/src/trap/context.rs

    #[repr(C)]
    pub struct TrapContext {
        pub x: [usize; 32],
        pub sstatus: Sstatus,
        pub sepc: usize,
    }
    ```
- [函数调用上下文](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter1/5support-func-call.html)：我们将由于函数调用，在控制流转移前后需要保持不变的寄存器集合称之为 函数调用上下文 (Function Call Context) 。由于每个 CPU 只有一套寄存器，我们若想在子函数调用前后保持函数调用上下文不变，就需要物理内存的帮助。这一过程由函数的调用者（caller）和被调用者（callee）合作完成。

spawn的实现仿照INITPROC的代码，但是需要注意：
- 设置子进程的父进程为调用spawn的进程
- 设置调用spawn的进程为子进程的父进程

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
    无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
    无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。