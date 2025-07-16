#![no_std]
#![no_main]


use log::info;
// pick a panicking behavior
use panic_rtt_target as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                           // use panic_abort as _; // requires nightly
                           // use panic_itm as _; // logs messages over ITM; requires ITM support
                           // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

mod loger;
use cortex_m_rt::entry;

const APP_ADDRESS: u32 = 0x08010000; // Address of the application to jump to

#[entry]
fn main() -> ! {
    loger::log_init();
    info!("starting up!");
    cortex_m::interrupt::disable();
    let cp = cortex_m::Peripherals::take().unwrap();
    let scb = cp.SCB;

    unsafe {
        // 如果不设置vtor寄存器, 在APP中使用中断会导致单片机重启
        // 当初测试APP没有使用中断, 所以跳过去后可以正常运行
        // 而且我以为 'bootload' 函数里面已经设置了vtor寄存器, 所以我没有在意
        // 直到最近尝试使用 rust 写一个完整功能, 跳转复杂APP的 bootloader时, 遇到了重复重启的问题
        // 我一直以为 bootload 函数中的 vr 参数就是设置 vtor 寄存器参数, 直到我翻到最底层才发现 vr 参数是传给 jump app 函数的
        // 所以我手动设置了 vtor 才解决了问题
        // 不过在开发全功能 bootloader 过程中我发现了许多问题:
        // 1. 使能过后的设备不能集中失能
        // 2. 不能集中清理中断标志位
        // 可能是我认知有限没有找到更好的方法, 最终我采用的是: 固件下载完成后复位跳转APP 的逻辑 
        scb.vtor.write(APP_ADDRESS & 0xFFFFFF80);
        cortex_m::asm::bootload(APP_ADDRESS as *const u32);
    }
}
