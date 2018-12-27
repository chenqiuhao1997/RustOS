# Rust OS lab 开发文档

## 关于 Rust OS lab
### 1 环境搭建(基于ubunut 16.04)
#### 1.1 配置curl并支持https
http://blog.aiyoe.com/linux/curl.html 
这里面会遇到libssl.so.1.1: cannot open shared object file: No such file or directory这个错误
通过创建软链接解决：参照https://blog.csdn.net/l_Laity/article/details/79090191

#### 1.2 安装rust
根据https://rustup.rs/ , 利用curl可安装rust

#### 1.3安装cargo相关工具
由于国内网络原因可以考虑配置代理服务器:

```
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

先装cargo xbuildhttps://github.com/rust-osdev/cargo-xbuild 
再装cargo bootimage https://github.com/rust-osdev/bootimage

#### 1.4 安装qemu
参考https://wiki.qemu.org/Hosts/Linux的介绍安装qemu即可

#### 1.5 安装RiscV64 GNU toolchain
https://www.sifive.com/products/tools/ 中的SiFive GNU Embedded Toolchain v20180629

#### 1.6 获取实验代码并安装相关依赖
```
git clone git@github.com:chenqiuhao1997/RustOS.git --recursive
cd RustOS/kernel
rustup override set nightly
make run smp=1 test=lab_test
```
注意这里编译的时候会出现类似qemu-system-x86_64不存在的问题，需要将之前安装过程中对应程序路径加入环境变量

此外,可以参考王润基同学的[RustOS开发文档](https://rucore.gitbook.io/rust-os-docs/kai-fa-huan-jing-pei-zhi)

### 2 测试
#### 2.1 单元测试
针对crate内部的各模块实现了单元测试,该部分测试在有std环境下进行,主要目的是验证各模块的算法功能正确性.

运行测试的方法是进入对应的crate,执行:

```
cargo test
```

#### 2.2 内核功能测试

内核功能测试用于针对内核(kernel)特定功能模块进行测试,由于内核模块没有std环境,而且很多功能需要在内核启动后才能测试,因此针对这部分功能模块的实现了内核功能测试.

该部分测试在内核启动后,进入用户shell之前来完成,采用了Rust的条件编译来完成,运行测试方式:

```
make run test=lab_test
```

#### 2.3 用户例程测试

最后为了测试系统的整体功能,我们引入了用户历程测试,测试方式是利用管道在qemu启动操作系统并进入用户shell之后输入指令运行特定用户程序并比对程序运行结果,来检测系统功能是否正常.

由于用户程序的输出采用的put_char的方式,因此对于多核情况下输出会发生混乱导致此分测试暂时无法实现.

单核情况下通常此部分测试是可以正确进行的,但是进程切换同样可能以比较小的概率导致输出混乱.

此部分测试和2.1以及2.2中的测试都被集成到了test/test.py中,运行测试的方式是:

```
cd test
python test.py
```



## Rust OS lab 实验设计
### 0 准备工作
* 自学Rust语言基础语法
* 了解riscv架构及汇编语言

### 1 Lab 1 系统软件启动过程
#### 1.1 实验目的
本实验的目标是实现一个简单,基本的操作系统,它能够完成各部分硬件的启动并且实现了一些基本的硬件驱动.之后我们在此基础上完成了基本的中断处理,io操作等功能并使得操作系统能正常处理中断并正确的通过串口打印调试信息.

此部分内容是整个RustOS的基本框架.

本实验希望涉及到的知识点包括:
* 操作系统启动的启动过程
* 中断,异常,系统调用处理流程
* IO操作的实现与调试信息的输出框架搭建

#### 1.2 实验内容

**练习1:**

* 阅读操作系统的Makefile文件,了解系统的编译流程以及常用的编译选项.

* 阅读lab1中RustOS的bootloader相关代码.

* 阅读lab1中关于logging的功能实现,了解RustOS对于IO操作的实现方式.

**练习2:**

* 完成kernel/arch/riscv32/boot/trap.asm中对于中断信息的保存和恢复部分代码.

* 完成kernel/arch/riscv32/interrupt.rs中的init()函数.

* 完成kernel/trap.rs里面对于时钟中断的处理(timer函数). 

### Lab 2 物理内存管理

#### 2.1 实验目的

* 理解基于页式的内存地址转换机制
* 理解页表的建立,使用和管理方法
* 理解物理内存的管理方法

#### 2.2 实验内容

**练习1:**

* 阅读crate/riscv中的内容,了解riscv32的页式地址转换机制和RustOS中为了修改页表项和实现的自映射机制

**练习2:**

* 实现基于线段树的bit-allocator

**练习3:**

* 完成kernel/arch/riscv32/memory.rs中的init函数,实现内存管理初始化.

**练习4:**

* 阅读crate/memory中的相关trait以及mock_page_table.rs中关于对于页表的一个假的实现
* 完成kernel/arch/riscv32/paging.rs对于ActivePageTable的实现
* 完成kernel/arch/riscv32/paging.rs中对于InactivePageTable0的实现

#### 2.3 riscv32下的页式地址转换与自映射

不同于x86, riscv32下如果二级页表项的属性是VRW,那么处理器就会认为这是一个指向4M大页的页表项,而不会去继续查找1级页表并最终找到对应的4K页. 因此RustOS中采用了特别地方式实现了针对二级页表的自映射.

页表的特定目录项(RECURSIVE_INDEX)为自映射项, 其属性为VR,之后其下一项同样指向自身,属性为VRW.

在进行自映射时先找到二级页表RECURSIVE_INDEX项再访问RECURSIVE_INDEX + 1即可完成自映射.

### Lab 3 虚拟内存管理

 #### 3.1 实验目的

* 了解虚存的概念与管理方式

#### 3.2 实验内容

**练习1:**

* 完成crate/memory/src/swap/fifo.rs中对于可置换物理页管理的先进先出算法的代码实现.

* 完成crate/memory/src/swap/mod.rs中对于SwapExt的实现,了解页面换入换出算法框架.
* 完成crate/memory/src/cow.rs中对于CowExt的实现,了解CowOnWrite的算法框架.

**练习2:**

* 完成kernel/src/memory.rs 中对于内核地址空间进行管理的SimpleMemoryHandler的实现,了解虚拟内存管理的基本算法框架.

**练习3:**

* 完成kernel/src/arch/riscv32/memory.rs中对于remap_the_kernel函数,了解RustOS中对于用于内存管理的模块memory_set的工作原理.

**challenge:**

* 完成crate/memory/src/swap/enhanced_clock.rs中对于可置换物理页管理的时钟算法的代码实现.

### Lab4 内核线程管理
#### 4.1 实验目的
* 了解内核线程创建/执行的管理过程
* 了解内核线程的切换和基本调度过程

#### 4.2 实验内容
**练习1:**
* 完成kernel/src/memory.rs中对于用户地址空间进行管理的MemoryHandler的极简实现。
 
*应该包括为进程分配资源的联系和switch相关内容的联系，视最终版本而定*


### Lab5 用户进程管理

#### 5.1 实验目的
* 了解第一个用户进程创建过程
* 了解系统调用框架的实现机制
* 了解ucore如何实现系统调用sys_fork/sys_exec/sys_exit/sys_wait来进行进程管理

#### 5.2 实验内容
**练习1:**

* 完成kernel/src/process/context.rs中newuser中的相关内容

**challenge:**

* 在kernel/src/arch/riscv32/memory.rs中完成具有全局页面置换和物理页帧延迟分配功能的SwapMemoryHandler,并在用户进程对应的用户地址空间(用户栈和用户代码数据段)应用上述MemoryHandler,并对比其与NormalMemoryHandler的区别
* 在kernel/src/arch/riscv32/memory.rs中完成具有CopyOnWrite功能的CowMemoryHandler,并在用户进程对应的用户地址空间(用户栈和用户代码数据段)应用上述MemoryHandler,并对比其与NormalMemoryHandler的区别

### Lab6 调度器

#### 6.1 实验目的
* 理解操作系统的调度管理机制
* 熟悉 RustOS 的系统调度器框架，以及缺省的Round-Robin 调度算法
* 基于调度器框架实现一个(Stride Scheduling)调度算法来替换缺省的调度算法

#### 6.2 实验内容
**练习1:**
* 熟悉在crate/process/src/scheduler.rs中的RRScheduler的算法实现

**练习2:**
* 在crate/process/src/scheduler.rs中实现 Stride Scheduling 调度算法

### Lab7 同步互斥

#### 7.1 实验目的
* 理解操作系统的同步互斥的设计实现；
* 理解底层支撑技术：禁用中断、定时器、等待队列；
* 在ucore中理解信号量（semaphore）机制的具体实现；
* 理解管程机制，在ucore内核中增加基于管程（monitor）的条件变量（condition variable）的支持；
* 了解经典进程同步问题，并能使用同步机制解决进程同步问题。

#### 7.2 实验内容
**练习1:**
* 在kernel/src/sync/semaphore.rs中实现基于中断禁止互斥锁和条件变量的信号量

**练习2:**
* 在kernel/src/sync/test.rs中实现基于互斥锁和信号量的管程
* 在kernel/src/sync/test.rs中分别实现基于互斥锁和管程的哲学家就餐问题

### Lab8 文件系统

#### 8.1 实验目的

#### 8.2 实验内容
