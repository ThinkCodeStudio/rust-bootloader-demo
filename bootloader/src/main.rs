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

const APP_ADDRESS: *const u32 = 0x08010000 as *const u32; // Address of the application to jump to

#[entry]
fn main() -> ! {
    loger::log_init();
    info!("starting up!");
    cortex_m::interrupt::disable();

    unsafe {
        cortex_m::asm::bootload(APP_ADDRESS);
    }
}
