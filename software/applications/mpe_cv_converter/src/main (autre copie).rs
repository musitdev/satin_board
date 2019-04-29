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
    //    static mut LED: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>> = ();
    //    static mut LED: hal::gpio::gpioh::PH3<hal::gpio::Output<hal::gpio::PushPull>> = ();

    static mut TIMESTAMP: u64 = 0;
    //TODO change to scheduled task
    //    static mut EVENT_TIMER: Timer<stm32f7::stm32f7x6::TIM2> = ();
    static mut MESSAGE_BUF: ArrayDeque<[satinapi::midi::MidiEvent; 4], Wrapping> = ();
    static mut MPE_MANAGER: satinapi::mpe::MonoNoteMPEManager = ();
    static mut RX_BUF: satinapi::midi::MidiBuffer = ();
    //    static mut MIDIOUT_BUF: heapless::spsc::Queue<u8, heapless::consts::U32> = ();
    static mut TX: Tx<stm32f7::stm32f7x6::USART2> = ();
    static mut RX: Rx<stm32f7::stm32f7x6::USART2> = ();
    static mut BOARD: crate::satinboard::SatinBoard = ();

    #[init(schedule = [tick])]
    fn init() -> init::LateResources {
        hprintln!("deb").unwrap();

        //light led.

        let device: stm32f7::stm32f7x6::Peripherals = device;
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

        schedule.tick(Instant::now() + 8_000_000.cycles()).unwrap();

        let gpioa = device.GPIOA.split();
        let gpiod = device.GPIOD.split();
        //Dac1 CS PA8
        let mut nss = gpiod.pd14.into_push_pull_output();
        nss.set_high(); //no data
        let sck = gpioa.pa5.into_alternate_af5();
        let miso = NoMiso;
        let mosi = gpioa.pa7.into_alternate_af5();

        let spi1 = Spi::spi1(
            device.SPI1,
            (sck, miso, mosi),
            crate::satinboard::MODE,
            // 1.mhz(),
            100.khz().into(),
            clocks,
        );

        nss.set_low(); //no data
        if let Err(err) = spi1.send_16b(56098) {
            hprintln!("spi err:{:?}", err).unwrap();
        } //MSB

        //    spi2.send(valeur_dac as u8)?; //LSB
        nss.set_high();

        let (tx, rx, satinboard) = crate::satinboard::init_board(device);

        init::LateResources {
            //            EVENT_TIMER: systick,
            TX: tx,
            RX: rx,
            RX_BUF: satinapi::midi::MidiBuffer::new(),
            MESSAGE_BUF: ArrayDeque::new(),
            //DAC = dac;
            BOARD: satinboard,
            MPE_MANAGER: satinapi::mpe::MonoNoteMPEManager::new(),
        }

        //     SWITCH_CV_STEREO_EN:PH13
        //        LED = led1;
        //        EVENT_TIMER = systick;
        /*        TX = tx;
        RX = rx;
        RX_BUF = satinapi::midi::MidiBuffer::new();
        MESSAGE_BUF = ArrayDeque::new();
        //DAC = dac;
        MPE_MANAGER = satinapi::mpe::MonoNoteMPEManager::new();
        BOARD = satinboard; */
        //        let gpiob = device.GPIOB.split();
        //        let mut led: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>> =
        //        gpiob.pb7.into_push_pull_output();
        //        led.set_high();

        //init gpio
        /*        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();
        let gpiod = device.GPIOD.split();
        let gpiog = device.GPIOG.split();
        let gpioh = device.GPIOH.split();
        let gpioi = device.GPIOI.split();

        let mut led1: hal::gpio::gpioh::PH3<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioh.ph3.into_push_pull_output();
        led1.set_high();
        */

        /*        let mut rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

        //init the board
        let satinboard = crate::satinboard::SatinBoard::new(&device, clocks);

        //        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();
        let mut systick = Timer::tim2(device.TIM2, CONST_TIMER_FREQ.hz(), clocks, &mut rcc.apb1);
        systick.listen(TimerEvent::TimeOut);

        //serial init
        let gpiod = device.GPIOD.split();
        let tx = gpiod.pd5.into_alternate_af7();
        let rx = gpiod.pd6.into_alternate_af7();
        //        let mut serial = Serial::usart2(device.USART2, (tx, rx), 115_200.bps(), clocks, false);
        let mut serial = Serial::usart2(device.USART2, (tx, rx), 31_250.bps(), clocks, true);

        //        serial.listen(Event::Txe);
        serial.listen(Event::Rxne);

        let (tx, rx) = serial.split(); */

        //init dac
        //hprintln!("before dac nit").unwrap();
        /*        let gpioa = device.GPIOA.split();
        gpioa.pa4.into_floating_input();
        gpioa.pa5.into_floating_input();
        rcc.apb1.enr().modify(|_, w| w.dacen().enabled()); //enable dac
        let dac =  device.DAC;
        //enable channel 1 and 2
        dac.cr.modify(|_, w| w.en1().enabled());
        dac.cr.modify(|_, w| w.en2().enabled());
        //DAC channel1 and 2 output buffer disable
        dac.cr.modify(|_, w| w. boff1().disabled());
        dac.cr.modify(|_, w| w. boff2().disabled());
        //DAC channel1 and 2 trigger disable
        dac.cr.modify(|_, w| w.ten1().disabled());
        dac.cr.modify(|_, w| w.ten2().disabled());

        // write test data 8bit Right aligned
        dac.dhr8r1.modify(|_, w| w.dacc1dhr().bits(250));
        dac.dhr8r2.modify(|_, w| w.dacc2dhr().bits(50)); */

        //init STM32 dac
        //SWITCH_NOTE 0-10V / -5,5V: PA0
        /*       let mut note_voltage_switch: hal::gpio::gpioa::PA0<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioa.pa0.into_push_pull_output();
        note_voltage_switch.set_low();

        //  SWITCH_VELOCITE 0-10V / -5,5V: PA1
        let mut velocity_voltage_switch: hal::gpio::gpioa::PA1<
            hal::gpio::Output<hal::gpio::PushPull>,
        > = gpioa.pa1.into_push_pull_output();
        velocity_voltage_switch.set_low();

        //Dac1 pin CV Note: ST_DAC_OUT1 PA4
        let dac1_pin = gpioa.pa4.into_analog_output();
        //Dac2 pin CV vélocity: ST_DAC_OUT2: PA5
        let dac2_pin = gpioa.pa5.into_analog_output();

        let dac = Dac::new(device.DAC);
        let mut dac1 = dac.create_dac_channel1(dac1_pin);
        dac1.enable();
        dac1.disable_output_buffer();
        dac1.disable_trigger();

        let mut dac2 = dac.create_dac_channel2(dac2_pin);
        dac2.enable();
        dac2.disable_output_buffer();
        dac2.disable_trigger();

        //test write
        dac1.write(DacWord::B8_ALIGN_R(128));
        dac2.write(DacWord::B8_ALIGN_R(67));

        hprintln!("end dac init").unwrap();

        //Configure Gate / Clock
        //clock out PC6
        let mut clock_out: hal::gpio::gpioc::PC6<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioc.pc6.into_push_pull_output();

        // gate out PA9
        let mut gate_out: hal::gpio::gpioa::PA9<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioa.pa9.into_push_pull_output();

        //Gate PA9 + invertion gate: PI8
        let mut invertion_gate: hal::gpio::gpioi::PI8<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioi.pi8.into_push_pull_output();
        invertion_gate.set_low();

        //clock out PC6 INVERSION_CLOCK PI9
        let mut invertion_clock: hal::gpio::gpioi::PI9<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioi.pi9.into_push_pull_output();
        invertion_clock.set_low();

        //4 SPI CV dac.
        //Dac1 CS PA8
        let mut nss_dac1 = gpioa.pa8.into_push_pull_output();
        nss_dac1.set_high(); //no data

        //Dac2 CS PD8
        let mut nss_dac2 = gpiod.pd8.into_push_pull_output();
        nss_dac2.set_high(); //no data

        //DAC3_CS PD0
        let mut nss_dac3 = gpiod.pd0.into_push_pull_output();
        nss_dac3.set_high(); //no data

        //DAC4_CS PD1
        let mut nss_dac4 = gpiod.pd1.into_push_pull_output();
        nss_dac4.set_high(); //no data

        //Dac1, Dac2, Dac 3 Dac 4 are controled via SPI2
        //SPI2:
        /*SPI2 GPIO Configuration
        PD3     ------> SPI2_SCK
        pas de SPI2 MISO.
        PI3     ------> SPI2_MOSI
        */
        let sck = gpiod.pd3.into_alternate_af5();
        let miso = NoMiso;
        let mosi = gpioi.pi3.into_alternate_af5();

        let mut spi = Spi::spi2(
            device.SPI2,
            (sck, miso, mosi),
            MODE,
            // 1.mhz(),
            100.khz().into(),
            clocks,
        );

        //DAC-CV clear init PI2
        let mut dac1cv_clear: hal::gpio::gpioi::PI2<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioi.pi2.into_push_pull_output();
        dac1cv_clear.set_high();

        //DAC-CV CV3:

        //SWITCH_CV_n3 switch 0-10 / -5,5v  PB14
        let mut cv3_voltage_switch: hal::gpio::gpiob::PB14<hal::gpio::Output<hal::gpio::PushPull>> =
            gpiob.pb14.into_push_pull_output();
        cv3_voltage_switch.set_low();

        //SWITCH_CVn3_OUTSwitch 0-5v ou (0-10 / -5,5v)  PB11
        let mut cv3_end_voltage_switch: hal::gpio::gpiob::PB11<
            hal::gpio::Output<hal::gpio::PushPull>,
        > = gpiob.pb11.into_push_pull_output();
        cv3_end_voltage_switch.set_low();

        //SWITCH_CV_n4 switch 0-10 / -5,5v  PH12
        let mut cv4_voltage_switch: hal::gpio::gpioh::PH12<hal::gpio::Output<hal::gpio::PushPull>> =
            gpioh.ph12.into_push_pull_output();
        cv4_voltage_switch.set_low();

        //SWITCH_CVn4_OUTSwitch 0-5v ou (0-10 / -5,5v)  PB12
        let mut cv4_end_voltage_switch: hal::gpio::gpiob::PB12<
            hal::gpio::Output<hal::gpio::PushPull>,
        > = gpiob.pb12.into_push_pull_output();
        cv4_end_voltage_switch.set_low();

        //switch CV(5-6) out or  stéreo DAC:   SWITCH_CV_STEREO OUT: PG2
        let mut cv_st_out_switch = gpiog.pg2.into_push_pull_output();
        cv_st_out_switch.set_high(); //no data
        */

        //One stéreo audio dac. TODO
        //Mut DAC3 PH14
        //DEMP1_DAC3 PH15
        //DEMP0_DAC3 PI0
        //FMT_DAC3 PI1
        //SCK_DAC3 PI4
        //BCK_DAC3 PI5
        //DATA_DAC3 PI6
        //LRCK_DAC3 PI7
        //ZEROA_DAC_stereo PG9
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
        //hprintln!("on{}", ison).unwrap();
        //     resources.EVENT_TIMER.start(CONST_TIMER_FREQ.hz());
        schedule.tick(Instant::now() + 8_000_000.cycles()).unwrap();
    }

    #[interrupt(priority = 4, resources=[RX,RX_BUF, MESSAGE_BUF, TIMESTAMP, ON], spawn = [manage_midi_input, show_error, show_string])]
    fn USART2() {
        //hprintln!("USART2.").unwrap();
        // Read each character from serial as it comes in
        loop {
            //            hprintln!("RL.").unwrap();
            match resources.RX.read() {
                Ok(c) => {
                    //hprintln!("c:{}", c).unwrap();
                    if let Some(message) = resources.RX_BUF.push_byte(c) {
                        let event = satinapi::midi::MidiEvent {
                            message,
                            timestamp: *resources.TIMESTAMP,
                        };
                        // TODO: handle buffer being full
                        if resources.MESSAGE_BUF.push_front(event).is_some() {
                            spawn
                                .show_string("Error input midi buf queue full lost byte.".into())
                                .unwrap();
                        } else {
                            //TODO use message to send the event.
                            spawn.manage_midi_input().unwrap(); //fail if the 4 capacity task is currently executing.
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
                        _ => spawn.show_error(e).unwrap(),
                        // currently no way to easily clear the overrun flag, if you hit this
                        // it'll be stuck here
                        /*                        nb::Error::Other(hal::serial::Error::Overrun) => {
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
                        } */
                    }
                    break;
                }
            }
        }
    }

    #[task(priority = 2, resources=[MESSAGE_BUF, TX, BOARD, MPE_MANAGER], spawn = [show_error, show_string], capacity = 4)]
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
                    //hprintln!("mpe:{:?}", note).unwrap();
                    //enable gate out
                    /*                    resources.BOARD.gate_out.set_high();
                    //for test set dac1 level depend on the note. TODO convertion table.
                    //note
                    resources
                        .BOARD
                        .note_velocity_dac
                        .dac1()
                        .write(DacWord::B8_ALIGN_R(note));
                    //Velocity
                    resources
                        .BOARD
                        .note_velocity_dac
                        .dac2()
                        .write(DacWord::B8_ALIGN_R(velocity)); */
                }
                satinapi::mpe::MPEEvent::NoteOff => {
                    resources.BOARD.output_note_off();
                }
                satinapi::mpe::MPEEvent::OtherMPE(midi_event) => {
                    //TODO send to the output
                    /*                    spawn
                    .show_string("manage_midi_input MPEEvent::OtherMPE.".into())
                    .unwrap(); */
                }
            };

            let buf: [u8; 3] = midi_event.message.into();
            resources.TX.lock(|tx| {
                buf.iter().for_each(|b| {
                    //write_byte(*b, &mut tx);
                })
            });

            //hprintln!("send event with time:{}", event.timestamp).unwrap();
        }
    }

    #[task]
    fn show_error(value: nb::Error<stm32f7xx_hal::serial::Error>) {
        hprintln!("serail error:{:?}", value).unwrap();
    }

    #[task]
    fn show_string(value: String<U64>) {
        hprintln!("{:?}", value).unwrap();
    }

    // Interrupt handlers used to dispatch software tasks. One interrupt per task.
    extern "C" {
        fn USART3();
        fn TIM3();
    }
};

pub fn write_byte(b: u8, tx: &mut stm32f7xx_hal::serial::Tx<stm32f7::stm32f7x6::USART2>) {
    //hprintln!("Wb:{:?}", b).unwrap();
    while let Err(err) = tx.write(b) {
        match err {
            nb::Error::WouldBlock => (),
            _ => {
                hprintln!("Write serial error:{:?}", err).unwrap();
                break;
            }
        }
    }
    while let Err(nb::Error::WouldBlock) = tx.flush() {}
    //hprintln!("WEb:{:?}", b).unwrap();
}

/*#[entry]
fn main() -> ! {
    let p = device::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
    let mut delay = Delay::new(cp.SYST, clocks);

    //init LED
    let gpiob = p.GPIOB.split();
    let mut led = gpiob.pb7.into_push_pull_output();
 //   let gpioh = p.GPIOH.split();
 //   let mut led = gpioh.ph2.into_push_pull_output();

// init Serial
    let gpiog = p.GPIOG.split();
    let tx = gpiog.pg14.into_alternate_af8();
    let rx = gpiog.pg9.into_alternate_af8();

    let serial = Serial::usart6(p.USART6, (tx, rx), 115_200.bps(), clocks);
 //   let serial = Serial::usart6(p.USART6, (tx, rx), 31_250.bps(), clocks);
   let (mut tx, _) = serial.split();

    let _hello: &str = "Hello, I'm a STM32F7xx!\r\n";

    let mut counter = 10;
    loop {
        led.set_high();
        delay.delay_ms(500_u16);
        led.set_low();
        delay.delay_ms(500_u16);
//        match tx.write_str(hello) {
        match tx.write(counter) {
            Ok(()) => (),
            Err(err) => {
                let mut stdout = &mut hio::hstdout().unwrap();
                write!(&mut stdout, "W{:?}", err).expect("Can't write");
            }
        }
        counter +=1;

    }
} */
