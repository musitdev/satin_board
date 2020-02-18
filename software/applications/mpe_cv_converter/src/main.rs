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
//extern crate panic_semihosting;
extern crate panic_halt;
//extern crate panic_halt;
extern crate embedded_hal as ehal;
extern crate nb;
extern crate satin_hal;
extern crate satinapi;
//extern crate stm32f7;

mod satinboard;

//mod dac;

//use crate::satinboard::CONST_TIMER_FREQ;
use arraydeque::ArrayDeque;
use arraydeque::Wrapping;
use heapless::consts::U64;
use heapless::String;
use nb::block;
//use rtfm::{app, Instant};
use satinapi::mpe::MPEControl;

use pac::DWT;
use pac::I2C1;
use processor_hal::device as pac;

use crate::ehal::spi::{Mode, Phase, Polarity};
use processor_hal::{
    device,
    gpio::*,
    prelude::*,
    serial::{self, Event, Rx, Serial, Tx},
    spi,
    spi::{NoMiso, Spi},
    //    timer::{Event as TimerEvent, Timer},
};
use stm32f7xx_hal as processor_hal;

use cortex_m_semihosting::hprintln;
use rtfm::cyccnt::{Instant, U32Ext as _};

#[rtfm::app(device = stm32f7xx_hal::device, peripherals = true, monotonic = rtfm::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        #[init(false)]
        ON: bool,
        #[init(0)]
        TIMESTAMP: u64,
        MESSAGE_BUF: ArrayDeque<[satinapi::midi::MidiEvent; 4], Wrapping>,
        MPE_MANAGER: satinapi::mpe::MonoNoteMPEManager,
        RX_BUF: satinapi::midi::MidiBuffer,
        TX: Tx<pac::USART2>,
        RX: Rx<pac::USART2>,
        BOARD: crate::satinboard::SatinBoard,
    }

    #[init(schedule = [tick])]
    fn init(cx: init::Context) -> init::LateResources {
        // Enable cycle counter for rtfmv4 bug
        //       let mut core = cx.core;
        //       core.DWT.enable_cycle_counter();
        // optional, configure the DWT run without a debugger connected
        //cx.core.DCB.enable_trace();

        let device: processor_hal::device::Peripherals = cx.device;

        let (tx, rx, mut satinboard) = crate::satinboard::init_board(device);
        cx.schedule
            .tick(Instant::now() + 8_000_000.cycles())
            .unwrap();

        //for test
        satinboard.write_sp2_cv1(5342).unwrap();
        satinboard.write_sp2_cv2(5342).unwrap();
        satinboard.write_sp2_cv3(5342).unwrap();
        satinboard.write_sp2_cv4(5342).unwrap();

        init::LateResources {
            MESSAGE_BUF: ArrayDeque::new(),
            MPE_MANAGER: satinapi::mpe::MonoNoteMPEManager::new(),
            RX_BUF: satinapi::midi::MidiBuffer::new(),
            TX: tx,
            RX: rx,
            BOARD: satinboard,
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
    fn tick(mut cx: tick::Context) {
        //        static mut BLINK: bool = false;
        cx.resources.TIMESTAMP.lock(|timestamp| *timestamp += 1);
        //        *resources.TIMESTAMP = *resources.TIMESTAMP + 1;
        cx.resources.TX.lock(|tx| {
            [0x90 as u8, 0x35 as u8, 0x15 as u8].iter().for_each(|b| {
                //write_byte(*b, &mut tx);
                /*            if resources.MIDIOUT_BUF.enqueue(*b).is_err() {
                    hprintln!("test send note queue full lost byte.").unwrap();
                } */
            })
        });

        let mut ison = false;
        cx.resources.ON.lock(|on| ison = *on);
        if ison {
            cx.resources
                .BOARD
                .lock(|board| board.led1.set_high())
                .unwrap();
        } else {
            cx.resources
                .BOARD
                .lock(|board| board.led1.set_low())
                .unwrap();
        }
        hprintln!("on{}", ison).unwrap();
        //     resources.EVENT_TIMER.start(CONST_TIMER_FREQ.hz());
        cx.schedule
            .tick(Instant::now() + 8_000_000.cycles())
            .unwrap();

        cx.schedule.tick(cx.scheduled + 8_000_000.cycles()).unwrap();
    }

    #[task(binds = USART2, priority = 4, resources=[RX,RX_BUF, TX, BOARD, MESSAGE_BUF, TIMESTAMP, ON], spawn = [manage_midi_input, show_error, show_string])]
    fn usart2(mut cx: usart2::Context) {
        match block!(cx.resources.RX.read()) {
            Ok(c) => {
                //hprintln!("c:{}", c).unwrap();
                if let Some(message) = cx.resources.RX_BUF.push_byte(c) {
                    let event = satinapi::midi::MidiEvent {
                        message,
                        timestamp: *cx.resources.TIMESTAMP,
                    };

                    if cx.resources.MESSAGE_BUF.push_front(event).is_some() {
                        cx.spawn
                            .show_string("Error input midi buf queue full lost byte.".into())
                            .unwrap();
                    } else {
                        //TODO use message to send the event.
                        cx.spawn.manage_midi_input().unwrap(); //fail if the 4 capacity task is currently executing.
                    }
                }
                *cx.resources.ON = !*cx.resources.ON;
            }
            Err(e) => {
                cx.spawn.show_error(nb::Error::Other(e)).unwrap();
            }
        }
    }

    #[task(priority = 2, resources=[MESSAGE_BUF, TX, BOARD, MPE_MANAGER], spawn = [show_error, show_string], capacity = 4)]
    fn manage_midi_input(mut cx: manage_midi_input::Context) {
        //hprintln!("manage_usart_input").unwrap();
        let mut mess_opt = None;
        cx.resources
            .MESSAGE_BUF
            .lock(|buff| mess_opt = buff.pop_front());
        if let Some(midi_event) = mess_opt {
            let mpe_event = cx.resources.MPE_MANAGER.manage_message(midi_event.message);
            //hprintln!("manage_midi_input mpe_event:{:?}", mpe_event).unwrap();
            //let board: &mut crate::satinboard::SatinBoard = cx.resources.BOARD;
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
                        MPEControl::XAxis => cx
                            .resources
                            .BOARD
                            .lock(|board| board.write_sp2_cv1(value.into())),
                        MPEControl::YAxis => cx
                            .resources
                            .BOARD
                            .lock(|board| board.write_sp2_cv2(value.into())),
                        MPEControl::ZAxis => cx
                            .resources
                            .BOARD
                            .lock(|board| board.write_sp2_cv3(value.into())),
                    } {
                        //TODO manage error
                        hprintln!("SPI error:{:?}", err).unwrap();
                    }
                }
                satinapi::mpe::MPEEvent::NoteOn { note, velocity } => {
                    //hprintln!("mpe:{:?}", note).unwrap();
                    cx.resources
                        .BOARD
                        .lock(|board| board.output_note(note, velocity));
                }
                satinapi::mpe::MPEEvent::NoteOff => {
                    cx.resources.BOARD.lock(|board| board.output_note_off());
                }
                satinapi::mpe::MPEEvent::OtherMPE(midi_event) => {
                    //TODO write to the output.
                    let buf: [u8; 3] = midi_event.into();
                    cx.resources.TX.lock(|tx| {
                        buf.iter().for_each(|b| {
                            //write_byte(*b, &mut tx);
                        })
                    });
                }
            };

            //hprintln!("send event with time:{}", event.timestamp).unwrap();
        }
    }

    #[task]
    fn show_error(mut cx: show_error::Context, value: nb::Error<stm32f7xx_hal::serial::Error>) {
        hprintln!("serail error:{:?}", value).unwrap();
    }

    #[task]
    fn show_string(mut cx: show_string::Context, value: String<U64>) {
        hprintln!("{:?}", value).unwrap();
    }

    // Interrupt handlers used to dispatch software tasks. One interrupt per task.
    extern "C" {
        fn USART1();
        fn USART3();
    //        fn UART4();
    //        fn TIM2();
    //        fn TIM3();
    }
};

pub fn write_byte(b: u8, tx: &mut stm32f7xx_hal::serial::Tx<pac::USART2>) {
    //hprintln!("Wb:{:?}", b).unwrap();
    while let Err(err) = block!(tx.write(b)) {
        hprintln!("Write serial error:{:?}", err).unwrap();
    }
    block!(tx.flush());
}
