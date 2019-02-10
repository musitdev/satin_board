//! examples/interrupt.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_semihosting::{debug, hprintln};
use stm32f7::stm32f7x6::Interrupt;
use rtfm::app;

#[app(device = stm32f7::stm32f7x6)]
const APP: () = {
    #[init]
    fn init() {
        // Pends the UART0 interrupt but its handler won't run until *after*
        // `init` returns because interrupts are disabled
        rtfm::pend(Interrupt::USART1);

        hprintln!("init").unwrap();
    }

    #[idle]
    fn idle() -> ! {
        // interrupts are enabled again; the `UART0` handler runs at this point

        hprintln!("idle").unwrap();

        rtfm::pend(Interrupt::USART1);

        debug::exit(debug::EXIT_SUCCESS);

        loop {}
    }

    #[interrupt]
    fn USART1() {
        static mut TIMES: u32 = 0;

        // Safe access to local `static mut` variable
        *TIMES += 1;

        hprintln!(
            "UART0 called {} time{}",
            *TIMES,
            if *TIMES > 1 { "s" } else { "" }
        )
        .unwrap();
    }
};