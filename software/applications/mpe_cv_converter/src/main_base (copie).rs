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
    //    delay::Delay,
    //    device,
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
    static mut TIMESTAMP: u64 = 0;
    static mut MESSAGE_BUF: ArrayDeque<[satinapi::midi::MidiEvent; 4], Wrapping> = ();
    static mut MPE_MANAGER: satinapi::mpe::MonoNoteMPEManager = ();
    static mut RX_BUF: satinapi::midi::MidiBuffer = ();
    static mut TX: Tx<stm32f7::stm32f7x6::USART2> = ();
    static mut RX: Rx<stm32f7::stm32f7x6::USART2> = ();
    static mut BOARD: crate::satinboard::SatinBoard = ();

    #[init(schedule = [tick])]
    fn init() -> init::LateResources {
        let device: stm32f7::stm32f7x6::Peripherals = device;

        let (tx, rx, mut satinboard) = crate::satinboard::init_board(device);
        schedule.tick(Instant::now() + 8_000_000.cycles()).unwrap();

        //for test
        satinboard.write_sp2_cv1(5342);
        satinboard.write_sp2_cv2(5342);
        satinboard.write_sp2_cv3(5342);
        satinboard.write_sp2_cv4(5342);

        init::LateResources {
            TX: tx,
            RX: rx,
            RX_BUF: satinapi::midi::MidiBuffer::new(),
            MESSAGE_BUF: ArrayDeque::new(),
            BOARD: satinboard,
            MPE_MANAGER: satinapi::mpe::MonoNoteMPEManager::new(),
        }
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
    #[task(priority = 1, resources = [TIMESTAMP, TX,ON, BOARD], schedule = [tick])]
    fn tick() {
        //        static mut BLINK: bool = false;
        resources.TIMESTAMP.lock(|timestamp| *timestamp += 1);
        //        *resources.TIMESTAMP = *resources.TIMESTAMP + 1;
        resources.TX.lock(|tx| {
            [0x90 as u8, 0x35 as u8, 0x15 as u8].iter().for_each(|b| {
                //write_byte(*b, &mut tx);
                /*            if resources.MIDIOUT_BUF.enqueue(*b).is_err() {
                    hprintln!("test send note queue full lost byte.").unwrap();
                } */
            })
        });

        let mut ison = false;
        resources.ON.lock(|on| ison = *on);
        if ison {
            resources.BOARD.lock(|board| board.led1.set_high());
        } else {
            resources.BOARD.lock(|board| board.led1.set_low());
        }
        hprintln!("on{}", ison).unwrap();
        //     resources.EVENT_TIMER.start(CONST_TIMER_FREQ.hz());
        schedule.tick(Instant::now() + 8_000_000.cycles()).unwrap();
    }

    #[interrupt(priority = 4, resources=[RX,RX_BUF, TX, BOARD, MESSAGE_BUF, TIMESTAMP, ON])]
    fn USART2() {
        match block!(resources.RX.read()) {
            Ok(c) => {
                //hprintln!("c:{}", c).unwrap();
                if let Some(message) = resources.RX_BUF.push_byte(c) {
                    let event = satinapi::midi::MidiEvent {
                        message,
                        timestamp: *resources.TIMESTAMP,
                    };

                    if resources.MESSAGE_BUF.push_front(event).is_some() {

                    } else {
                        //TODO use message to send the event.
                    }
                }
                *resources.ON = !*resources.ON;
            }
            Err(e) => {}
        }
    }

    /*    #[task(priority = 2, resources=[MESSAGE_BUF, TX, BOARD, MPE_MANAGER], spawn = [show_error, show_string], capacity = 4)]
    fn manage_midi_input() {
        //hprintln!("manage_usart_input").unwrap();
        let mut mess_opt = None;
        resources
            .MESSAGE_BUF
            .lock(|buff| mess_opt = buff.pop_front());
        if let Some(midi_event) = mess_opt {
            let mpe_event = resources.MPE_MANAGER.manage_message(midi_event.message);
            //hprintln!("manage_midi_input mpe_event:{:?}", mpe_event).unwrap();
            match mpe_event {
                satinapi::mpe::MPEEvent::NoEvent => {}
                satinapi::mpe::MPEEvent::GlobalNoteChange { control, value } => {
                    hprintln!(
                        "manage_midi_input MPEEvent::GlobalNoteChange control:{:?} value:{:?}",
                        control,
                        value
                    )
                    .unwrap();
                }
                satinapi::mpe::MPEEvent::PerNoteChange { control, value } => {
                    if let Err(err) = match control {
                        MPEControl::XAxis => resources.BOARD.write_sp2_cv1(value.into()),
                        MPEControl::YAxis => resources.BOARD.write_sp2_cv2(value.into()),
                        MPEControl::ZAxis => resources.BOARD.write_sp2_cv3(value.into()),
                    } {
                        //TODO manage error
                        hprintln!("SPI error:{:?}", err).unwrap();
                    }
                }
                satinapi::mpe::MPEEvent::NoteOn { note, velocity } => {
                    //hprintln!("mpe:{:?}", note).unwrap();
                    resources.BOARD.output_note(note, velocity);
                }
                satinapi::mpe::MPEEvent::NoteOff => {
                    resources.BOARD.output_note_off();
                }
                satinapi::mpe::MPEEvent::OtherMPE(midi_event) => {
                    //TODO write to the output.
                    let buf: [u8; 3] = midi_event.into();
                    resources.TX.lock(|tx| {
                        buf.iter().for_each(|b| {
                            //write_byte(*b, &mut tx);
                        })
                    });
                }
            };

            //hprintln!("send event with time:{}", event.timestamp).unwrap();
        }
    } */

    /*    #[task]
    fn show_error(value: nb::Error<stm32f7xx_hal::serial::Error>) {
        hprintln!("serail error:{:?}", value).unwrap();
    }

    #[task]
    fn show_string(value: String<U64>) {
        hprintln!("{:?}", value).unwrap();
    } */

    // Interrupt handlers used to dispatch software tasks. One interrupt per task.
    extern "C" {
        fn USART1();
    //        fn USART3();
    //        fn UART4();
    //        fn TIM2();
    //        fn TIM3();
    }
};

pub fn write_byte(b: u8, tx: &mut stm32f7xx_hal::serial::Tx<stm32f7::stm32f7x6::USART2>) {
    //hprintln!("Wb:{:?}", b).unwrap();
    while let Err(err) = block!(tx.write(b)) {
        hprintln!("Write serial error:{:?}", err).unwrap();
    }
    block!(tx.flush());
}
