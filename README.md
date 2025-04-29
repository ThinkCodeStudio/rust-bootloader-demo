# Rust BootLoader Demo
使用Rust开发单片机Demo  
其实实现非常简单, 得益于 `Cortex-M` 库已经封装了 [bootload](https://docs.rs/cortex-m/latest/cortex_m/asm/fn.bootload.html) 函数, 我们只需要将中断向量表的地址传入就可以了, 如果没有成功检查两个固件是否重叠了  
实验单片机: STM32F429BIT6  
这个bootloader应该和单片机厂商没有关系, 只要是Cortex-M就可以运行, 因为我写的bootloader固件根本没有使用stm32-hal库  