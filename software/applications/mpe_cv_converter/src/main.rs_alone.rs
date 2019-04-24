#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
#[macro_use(block)]
extern crate nb;
extern crate panic_semihosting;
//extern crate panic_halt;
extern crate arraydeque;
extern crate satinapi;
extern crate stm32f7;

//mod dac;

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
use arraydeque::ArrayDeque;
use cortex_m_semihosting::{debug, hprintln}; //, hio

const CONST_TIMER_FREQ: u32 = 1;

#[entry]
fn main() -> ! {
    let device: stm32f7::stm32f7x6::Peripherals = device::Peripherals::take().unwrap();
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
    let serial = Serial::usart2(device.USART2, (tx, rx), 31_250.bps(), clocks, true);

    let (tx, mut rx) = serial.split();

    let mut RX_BUF: satinapi::midi::MidiBuffer = satinapi::midi::MidiBuffer::new();
    let mut deque: ArrayDeque<[_; 2]> = ArrayDeque::new();
    let mut counter = 0;
    let mut blink = false;
    hprintln!("deb").unwrap();
    loop {
        //blink on error
        counter += 1;

        if let Some(mess) = deque.pop_front() {
            if counter % 2 == 0 {
                if blink {
                    led.set_high();
                } else {
                    led.set_low();
                }
                blink = !blink;
            }
        }

        //main loop
        loop {
            //            hprintln!("RL.").unwrap();
            match rx.read() {
                Ok(c) => {
                    //hprintln!("c:{}", c).unwrap();
                    if let Some(message) = RX_BUF.push_byte(c) {
                        let event = satinapi::midi::MidiEvent {
                            message,
                            timestamp: 0,
                        };

                        deque.push_back(event);

                        /* for i in 0..100 {
                            counter += 1;
                        } */
                        // TODO: handle buffer being full
                        /*                       if MESSAGE_BUF.enqueue(event).is_err() {
                            hprintln!("input midi buf queue full lost byte.").unwrap();
                        } else {
                            // spawn.manage_midi_input().unwrap();
                        } */
                    }
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
    }
}
