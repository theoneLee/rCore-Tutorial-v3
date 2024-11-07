
#![no_std]
#![no_main]
mod lang_items;

//不使用 Rust 标准库 std 转而使用核心库 core
// fn main() {
//     // println!("Hello, world!");
// }

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
//通过 include_str! 宏将同目录下的汇编代码 entry.asm 转化为字符串并通过 global_asm! 宏嵌入到代码中

// entry.asm 的 .bss.stack 段最终会被汇集到 .bss 段（linker.ld脚本处理）。但bss一般需要放置初始化为0的数据。
// 然而栈并不需要在使用前被初始化为零，因为在函数调用的时候我们会插入栈帧覆盖已有的数据。我们尝试将其放置到全局数据 .data 段中但最后未能成功，
// 因此才决定将其放置到 .bss 段中。全局符号 sbss 和 ebss 分别指向 .bss 段除 .bss.stack 以外的起始和终止地址，我们在使用这部分数据之前需要将它们初始化为零

#[no_mangle]
pub fn rust_main()->!{
    clear_bss();
    loop{}
}




/*
函数 clear_bss 中，我们会尝试从其他地方找到全局符号 sbss 和 ebss ，
它们由链接脚本 linker.ld 给出，并分别指出需要被清零的 .bss 段的起始和终止地址。接下来我们只需遍历该地址区间并逐字节进行清零即可。
 */
fn clear_bss(){
    extern "C"{ //调用它的时候要遵从目标平台的 C 语言调用规范
        fn sbss();
        fn ebss();
    }
    //这里只是引用位置标志并将其转成 usize 获取它的地址。由此可以知道 .bss 段两端的地址。
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe{ //将 .bss 段内的一个地址转化为一个 裸指针 (Raw Pointer)，并将它指向的值修改为 0
            (a as *mut u8).write_volatile(0)
        }
    });
}