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

主要任务是实现PageTable和InactivePageTable 以及内存的初始化

### Lab 3 虚拟内存管理

主要设计MemorySet相关内容的实现