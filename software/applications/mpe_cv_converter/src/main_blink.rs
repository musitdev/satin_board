//start on satin board
//./openocd-0.10.0/bin/openocd -f interface/stlink-v2.cfg -f target/stm32f7x.cfg
//./openocd-0.10.0/bin/openocd -f interface/stlink-v2-1.cfg -f target/stm32f7x.cfg
//../../../programs/gcc-arm-none-eabi-6-2017-q2-update/bin/arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/test1
// configure stty -F /dev/ttyS0 speed 9600 cs8 -cstopb -parenb &&
//cat /dev/ttyS0
//od -x < /dev/ttyS0 hexdump

// removed beacause of DWT bug and RTFM scheduler. DWT conf need unsafe #![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]

// panic-handler crate
extern crate panic_semihosting;
//extern crate panic_halt;
extern crate embedded_hal as ehal;
extern crate satinapi;
extern crate stm32f7;

mod satinboard;

//mod dac;

//use crate::satinboard::CONST_TIMER_FREQ;
use arraydeque::ArrayDeque;
use arraydeque::Wrapping;
use cortex_m::peripheral::DWT;
use heapless::consts::U64;
use heapless::String;
use nb::block;
use rtfm::{app, Instant};
use satinapi::mpe::MPEControl;
use stm32f7xx_hal as hal;
use stm32f7xx_hal::serial::Rx;
use stm32f7xx_hal::serial::Tx;
//use stm32f7xx_hal::timer::Timer;
//use core::fmt::Write;
//use cortex_m_rt::entry;

use crate::ehal::spi::{Mode, Phase, Polarity};
use crate::hal::{
    dac::{Dac, DacWord, DacWriter},
    gpio::*,
    prelude::*,
    serial::{Event, Serial},
    spi,
    spi::{NoMiso, Spi},
    //    timer::{Event as TimerEvent, Timer},
};

use crate::hal::prelude::*;

use cortex_m_semihosting::hprintln; //, hio

#[app(device = stm32f7::stm32f7x6)]
const APP: () = {
    static mut ON: bool = false;
    //    static mut LED: hal::gpio::gpioh::PH3<hal::gpio::Output<hal::gpio::PushPull>> = ();
    static mut LED: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>> = ();

    #[init(schedule = [tick])]
    fn init() {
        let device: stm32f7::stm32f7x6::Peripherals = device;

        let gpioh = device.GPIOH.split();
        let mut led_satin: hal::gpio::gpioh::PH3<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioh.ph3.into_push_pull_output();

        let gpiob = device.GPIOB.split();
        let mut led_disco: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>> =
            gpiob.pb7.into_push_pull_output();

        let mut rcc = device.RCC.constrain();
        //let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
        let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

        //        unsafe { (*DWT::ptr()).lar.write(0xC5ACCE55) };

        schedule.tick(Instant::now() + 8_000_000.cycles()).unwrap();
        LED = led_disco;
    }

    /*    #[idle (resources = [LED])]
        fn idle() -> ! {

    //        resources.LED.set_high();
            // error: `SHARED` can't be accessed from this context
            // SHARED += 1;
            hprintln!("idle").unwrap();

            debug::exit(debug::EXIT_SUCCESS);
            loop {}
        } */

    // `SHARED` can be access from this context
    #[task(resources = [ON, LED], schedule = [tick])]
    fn tick() {
        if *resources.ON {
            resources.LED.set_high();
        } else {
            resources.LED.set_low();
        }

        *resources.ON = !*resources.ON;
        hprintln!("on{}", *resources.ON).unwrap();
        //     resources.EVENT_TIMER.start(CONST_TIMER_FREQ.hz());
        schedule.tick(Instant::now() + 8_000_000.cycles()).unwrap();
    }

    // Interrupt handlers used to dispatch software tasks. One interrupt per task.
    extern "C" {
        fn USART1();
    }
};
