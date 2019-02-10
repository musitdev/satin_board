//start on satin board
//./openocd-0.10.0/bin/openocd -f interface/stlink-v2.cfg -f target/stm32f7x.cfg
//../../../programs/gcc-arm-none-eabi-6-2017-q2-update/bin/arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/test1
// configure stty -F /dev/ttyS0 speed 9600 cs8 -cstopb -parenb &&
//cat /dev/ttyS0
//od -x < /dev/ttyS0 hexdump

//on discovery
// serial:
//        static RX: Rx<stm32f7x::USART6>;
//        static TX: Tx<stm32f7x::USART6>;
//    let tx = gpiog.pg14.into_af7(&mut gpiog.moder, &mut gpiog.afrh);
//    let rx = gpiog.pg9.into_af7(&mut gpiog.moder, &mut gpiog.afrh);

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;
extern crate cortex_m_semihosting;

use stm32f7xx_hal as hal;

//use core::fmt::Write as FmtWrite;
use cortex_m_semihosting::hio;

use core::fmt::Write;
use crate::hal::{
    prelude::*,
    device,
    delay::Delay,
    serial::Serial,
};
use cortex_m_rt::entry;

#[entry]
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
}