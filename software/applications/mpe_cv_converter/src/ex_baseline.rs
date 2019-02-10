//! examples/baseline.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::{debug, hprintln};
use stm32f7::stm32f7x6::Interrupt;
use rtfm::app;

// NOTE: does NOT properly work on QEMU
#[app(device = stm32f7::stm32f7x6)]
const APP: () = {
    #[init(spawn = [foo])]
    fn init() {
        hprintln!("init(baseline = {:?})", start).unwrap();

        // `foo` inherits the baseline of `init`: `Instant(0)`
        spawn.foo().unwrap();
    }

    #[task(schedule = [foo])]
    fn foo() {
        static mut ONCE: bool = true;

        hprintln!("foo(baseline = {:?})", scheduled).unwrap();

        if *ONCE {
            *ONCE = false;

            rtfm::pend(Interrupt::USART1);
        } else {
            debug::exit(debug::EXIT_SUCCESS);
        }
    }

    #[interrupt(spawn = [foo])]
    fn USART1() {
        hprintln!("UART0(baseline = {:?})", start).unwrap();

        // `foo` inherits the baseline of `UART0`: its `start` time
        spawn.foo().unwrap();
    }

    extern "C" {
        fn USART2();
    }
};