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
extern crate stm32f7;
extern crate satinapi;

//mod dac;


use rtfm::{app};
use stm32f7xx_hal as hal;
//use core::fmt::Write;
//use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln}; //, hio
use crate::hal::{
    prelude::*,
    device,
    delay::Delay,
    timer::{Timer, Event as TimerEvent},
    serial::{Rx, Serial, Tx, Error, Event},
};
use heapless;

const CONST_TIMER_FREQ: u32 = 1;


#[app(device = stm32f7::stm32f7x6)]
const APP: () = {
//    static mut ON: bool = false;
//    static mut LED: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>> = ();
    static mut TIMESTAMP: u64 = 0;
    static mut EVENT_TIMER: Timer<stm32f7::stm32f7x6::TIM2> = ();
    static mut MESSAGE_BUF: heapless::spsc::Queue<satinapi::MidiEvent, heapless::consts::U32> = ();
    static mut TX: Tx<stm32f7::stm32f7x6::USART2> = ();
    static mut RX: Rx<stm32f7::stm32f7x6::USART2> = ();
    static mut RX_BUF : satinapi::MidiBuffer = ();

    #[init] //(schedule = [blink])
    fn init() {
        hprintln!("deb").unwrap();

        //light led.

        let device: stm32f7::stm32f7x6::Peripherals = device;
        let gpiob = device.GPIOB.split();
        let mut led: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>> = gpiob.pb7.into_push_pull_output();
        led.set_high();

        let mut rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();
        let mut systick = Timer::tim2(device.TIM2, CONST_TIMER_FREQ.khz(), clocks, &mut rcc.apb1);
        systick.listen(TimerEvent::TimeOut);

        //serial init
        let gpiod = device.GPIOD.split();
        let tx = gpiod.pd5.into_alternate_af7();
        let rx = gpiod.pd6.into_alternate_af7();
        let mut serial = Serial::usart2(device.USART2, (tx, rx), 115_200.bps(), clocks);
 //   let serial = Serial::usart2(p.USART6, (tx, rx), 31_250.bps(), clocks);
       serial.listen(Event::Rxne);

        let (tx, rx) = serial.split();

        //init dac
        hprintln!("before dac nit").unwrap();
        let gpioa = device.GPIOA.split();
        gpioa.pa4.into_floating_input();
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
        dac.dhr8r1.modify(|_, w| w.dacc1dhr().bits(127));
        dac.dhr8r2.modify(|_, w| w.dacc2dhr().bits(127));
        hprintln!("end dac init").unwrap();

        
        EVENT_TIMER = systick;
        TX = tx;
        RX = rx;
        RX_BUF = satinapi::MidiBuffer::new();
        MESSAGE_BUF = heapless::spsc::Queue::new();
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
     #[interrupt(resources = [EVENT_TIMER, TIMESTAMP])]
    fn TIM2() {
        *resources.TIMESTAMP = *resources.TIMESTAMP + 1;
        resources.EVENT_TIMER.start(CONST_TIMER_FREQ.khz());
    } 

    #[interrupt(resources=[RX,RX_BUF, MESSAGE_BUF, TIMESTAMP], spawn = [manage_midi_input])]
    fn USART2() {
//        hprintln!("USART2.").unwrap();
        // Read each character from serial as it comes in
        match resources.RX.read() {
            Ok(c) => {
                if let Some(message) = resources.RX_BUF.push_byte(c) {
                    let event = satinapi::MidiEvent{
                        message,
                        timestamp: *resources.TIMESTAMP,
                    };
                    // TODO: handle buffer being full                
                    if resources.MESSAGE_BUF.enqueue(event).is_err() {
                        hprintln!("input serial queue full lost byte.").unwrap();
                    } else {
                        spawn.manage_midi_input().unwrap();
                    }
                }
            }
            Err(e) => {
                match e {
                    nb::Error::WouldBlock => {
                        hprintln!("wouldblock.").unwrap();
                    }
                    // currently no way to easily clear the overrun flag, if you hit this
                    // it'll be stuck here
                    nb::Error::Other(hal::serial::Error::Overrun) => {
                        hprintln!("input serial queue overrun error.").unwrap();
                    },
                    nb::Error::Other(hal::serial::Error::Framing) => {
                        hprintln!("input serial queue Framing error.").unwrap();
                    },
                    nb::Error::Other(hal::serial::Error::Noise) => {
                        hprintln!("input serial queue Noise error.").unwrap();
                    },
                    nb::Error::Other(hal::serial::Error::Parity) => {
                        hprintln!("input serial queue Parity error.").unwrap();
                    },
                    _ => {
                        hprintln!("input serial queue other error.").unwrap();
                    }
                }
            }
        }
    }
    
    #[task(resources=[MESSAGE_BUF, TX], capacity = 4)]
    fn manage_midi_input() {
        //hprintln!("manage_usart_input").unwrap();
        while let Some(event) = resources.MESSAGE_BUF.dequeue()    { 
             //hprintln!("send event with time:{}", event.timestamp).unwrap();
             let buf:[u8; 3] = event.message.into();  
             let mut index = 0; 
             while index < 3 {
                match resources.TX.write(buf[index]) {
                    Ok(()) => index += 1,
                    Err(err) => {
                        match err {
                            nb::Error::WouldBlock => {
                                //hprintln!("wouldblock.").unwrap();
                            }
                            _ =>  { 
                                hprintln!("Write serial error:{:?}", err).unwrap();
                                break;
                            },

                        }
                       
                    },
                }               
             }   

        }
    }

    // Interrupt handlers used to dispatch software tasks
    extern "C" {
        fn USART1();
    }
};


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