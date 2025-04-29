#![no_std]
#![no_main]

// pick a panicking behavior
use panic_rtt_target as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::{entry, pre_init};

// run success when not used this code
// #[pre_init]
// unsafe fn set_vtor(){
//     const NVIC_VTOR_MASK: u32 = 0xFFFFFF80;
//     let pac = cortex_m::peripheral::Peripherals::take().unwrap();
//     let scb = pac.SCB;
//     scb.vtor.write(NVIC_VTOR_MASK & 0x08010000);
// }

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let gpioc = p.GPIOB.split();
    let mut led = gpioc.pb13.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            led.set_high();
        }
        for _ in 0..10_000 {
            led.set_low();
        }
    }
}
