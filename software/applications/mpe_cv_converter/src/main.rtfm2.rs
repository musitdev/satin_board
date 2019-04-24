//start on satin board
//./openocd-0.10.0/bin/openocd -f interface/stlink-v2.cfg -f target/stm32f7x.cfg
//./openocd-0.10.0/bin/openocd -f interface/stlink-v2-1.cfg -f target/stm32f7x.cfg
//../../../programs/gcc-arm-none-eabi-6-2017-q2-update/bin/arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/test1
// configure stty -F /dev/ttyS0 speed 9600 cs8 -cstopb -parenb &&
//cat /dev/ttyS0
//od -x < /dev/ttyS0 hexdump

#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception, interrupt)]
extern crate cortex_m_rt as rt;
#[macro_use(block)]
extern crate nb;
extern crate panic_semihosting;
//extern crate panic_halt;
extern crate satinapi;
extern crate stm32f7;

use rtfm::app;
use stm32f7xx_hal as hal;
//use core::fmt::Write;
//use cortex_m_rt::entry;
use crate::hal::{
    dac::{Dac, DacWord, DacWriter},
    delay::Delay,
    device,
    prelude::*,
    serial::{Error, Event, Rx, Serial, Tx},
    timer::{Event as TimerEvent, Timer},
};
use arraydeque::{ArrayDeque, Wrapping};
use cortex_m_semihosting::{debug, hprintln}; //, hio

const CONST_TIMER_FREQ: u32 = 1;

#[app(device = stm32f7::stm32f7x6)]
const APP: () = {
    static mut ON: bool = false;
    static mut RX: Rx<stm32f7::stm32f7x6::USART2> = ();
    static mut LED: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>> = ();
    static mut MESSAGE_BUF: ArrayDeque<[satinapi::midi::MidiEvent; 4], Wrapping> = ();
    static mut RX_BUF: satinapi::midi::MidiBuffer = ();

    #[init] //(schedule = [blink])
    fn init() {
        hprintln!("deb").unwrap();

        //light led.

        let device: stm32f7::stm32f7x6::Peripherals = device;
        let gpiob = device.GPIOB.split();
        let mut led: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>> =
            gpiob.pb7.into_push_pull_output();
        led.set_high();

        let mut rcc = device.RCC.constrain();
        //let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
        let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

        //serial init
        let gpiod = device.GPIOD.split();
        let tx = gpiod.pd5.into_alternate_af7();
        let rx = gpiod.pd6.into_alternate_af7();
        //        let mut serial = Serial::usart2(device.USART2, (tx, rx), 115_200.bps(), clocks, false);
        let mut serial = Serial::usart2(device.USART2, (tx, rx), 31_250.bps(), clocks, true);

        //        serial.listen(Event::Txe);
        serial.listen(Event::Rxne);

        let (tx, rx) = serial.split();

        LED = led;
        RX = rx;
        RX_BUF = satinapi::midi::MidiBuffer::new();
        MESSAGE_BUF = ArrayDeque::new();
    }

    #[idle]
    fn idle() -> ! {
        //        resources.LED.set_high();
        // error: `SHARED` can't be accessed from this context
        // SHARED += 1;
        hprintln!("idle").unwrap();

        //debug::exit(debug::EXIT_SUCCESS);
        loop {}
    }

    #[interrupt(resources=[RX, ON, LED,RX_BUF, MESSAGE_BUF], spawn = [manage_midi_input])]
    fn USART2() {
        //hprintln!("USART2.").unwrap();
        // Read each character from serial as it comes in
        loop {
            //            hprintln!("RL.").unwrap();
            match resources.RX.read() {
                Ok(c) => {
                    // hprintln!("c:{}", c).unwrap();
                    if let Some(message) = resources.RX_BUF.push_byte(c) {
                        let event = satinapi::midi::MidiEvent {
                            message,
                            timestamp: 0,
                        };
                        // TODO: handle buffer being full
                        if resources.MESSAGE_BUF.push_front(event).is_some() {
                            hprintln!("input midi buf queue full lost byte.").unwrap();
                        } else {
                            spawn.manage_midi_input().unwrap();
                        }
                    }
                    *resources.ON = !*resources.ON;
                    break;
                }
                Err(e) => {
                    match e {
                        nb::Error::WouldBlock => {
                            //hprintln!("wblck.").unwrap();
                            continue;
                        }
                        // currently no way to easily clear the overrun flag, if you hit this
                        // it'll be stuck here
                        nb::Error::Other(hal::serial::Error::Overrun) => {
                            hprintln!("input serial read overrun error.").unwrap();
                        }
                        nb::Error::Other(hal::serial::Error::Framing) => {
                            hprintln!("input serial read Framing error.").unwrap();
                        }
                        nb::Error::Other(hal::serial::Error::Noise) => {
                            hprintln!("input serial read Noise error.").unwrap();
                        }
                        nb::Error::Other(hal::serial::Error::Parity) => {
                            hprintln!("input serial read Parity error.").unwrap();
                        }
                        _ => {
                            hprintln!("input serial read other error.").unwrap();
                        }
                    }
                    break;
                }
            }
        }

        if *resources.ON {
            resources.LED.set_high();
        } else {
            resources.LED.set_low();
        }
    }

    #[task(resources=[MESSAGE_BUF], capacity = 4)]
    fn manage_midi_input() {
        //hprintln!("manage_usart_input").unwrap();
        while let Some(event) = resources.MESSAGE_BUF.pop_front() {
            //hprintln!("send event with time:{}", event.timestamp).unwrap();
            //for test set dac1 level depend on the note.
            //note
            /*            buf.iter().for_each(|b| {
                write_byte(*b, &mut *resources.TX);
            }); */
        }
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART1();
    }
};
