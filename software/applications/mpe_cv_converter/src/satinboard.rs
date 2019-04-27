use stm32f7;
use stm32f7xx_hal as hal;
//use core::fmt::Write;
//use cortex_m_rt::entry;
use crate::ehal::spi::{Mode, Phase, Polarity};
use crate::hal::{
    dac::{Dac, DacWord, DacWriter},
    delay::Delay,
    device,
    gpio::*,
    prelude::*,
    serial::{Error, Event, Rx, Serial, Tx},
    spi::{NoMiso, Spi},
    timer::{Event as TimerEvent, Timer},
};
use cortex_m_semihosting::hprintln;

pub const CONST_TIMER_FREQ: u32 = 1;

pub fn init_board(
    device: stm32f7::stm32f7x6::Peripherals,
) -> (
    stm32f7xx_hal::serial::Tx<stm32f7::stm32f7x6::USART2>,
    stm32f7xx_hal::serial::Rx<stm32f7::stm32f7x6::USART2>,
    //    Timer<stm32f7::stm32f7x6::TIM2>,
    SatinBoard,
) {
    let mut rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    //        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();
    //    let mut systick = Timer::tim2(device.TIM2, CONST_TIMER_FREQ.hz(), clocks, &mut rcc.apb1);
    //    systick.listen(TimerEvent::TimeOut);

    //serial init
    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();
    let gpioc = device.GPIOC.split();
    let gpiod = device.GPIOD.split();

    let tx = gpiod.pd5.into_alternate_af7();
    let rx = gpiod.pd6.into_alternate_af7();
    //        let mut serial = Serial::usart2(device.USART2, (tx, rx), 115_200.bps(), clocks, false);
    let mut serial = Serial::usart2(device.USART2, (tx, rx), 31_250.bps(), clocks, true);

    //        serial.listen(Event::Txe);
    serial.listen(Event::Rxne);

    let (tx, rx) = serial.split();

    //Dac and SPI GPIO init

    let gpiog = device.GPIOG.split();
    let gpioh = device.GPIOH.split();
    let gpioi = device.GPIOI.split();

    //init led 1 to high
    let mut led1: hal::gpio::gpioh::PH3<hal::gpio::Output<hal::gpio::PushPull>> =
        gpioh.ph3.into_push_pull_output();
    led1.set_high();

    //init STM32 dac
    //SWITCH_NOTE 0-10V / -5,5V: PA0
    let mut note_voltage_switch: hal::gpio::gpioa::PA0<hal::gpio::Output<hal::gpio::PushPull>> =
        gpioa.pa0.into_push_pull_output();
    note_voltage_switch.set_low();

    //  SWITCH_VELOCITE 0-10V / -5,5V: PA1
    let mut velocity_voltage_switch: hal::gpio::gpioa::PA1<hal::gpio::Output<hal::gpio::PushPull>> =
        gpioa.pa1.into_push_pull_output();
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

    let mut spi2 = Spi::spi2(
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
    let mut cv3_end_voltage_switch: hal::gpio::gpiob::PB11<hal::gpio::Output<hal::gpio::PushPull>> =
        gpiob.pb11.into_push_pull_output();
    cv3_end_voltage_switch.set_low();

    //SWITCH_CV_n4 switch 0-10 / -5,5v  PH12
    let mut cv4_voltage_switch: hal::gpio::gpioh::PH12<hal::gpio::Output<hal::gpio::PushPull>> =
        gpioh.ph12.into_push_pull_output();
    cv4_voltage_switch.set_low();

    //SWITCH_CVn4_OUTSwitch 0-5v ou (0-10 / -5,5v)  PB12
    let mut cv4_end_voltage_switch: hal::gpio::gpiob::PB12<hal::gpio::Output<hal::gpio::PushPull>> =
        gpiob.pb12.into_push_pull_output();
    cv4_end_voltage_switch.set_low();

    //switch CV(5-6) out or  stéreo DAC:   SWITCH_CV_STEREO OUT: PG2
    let mut cv_st_out_switch = gpiog.pg2.into_push_pull_output();
    cv_st_out_switch.set_high(); //no data

    let board = SatinBoard {
        //		led_discovery: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>>
        led1: led1,
        gate_out,
        clock_out,
        note_velocity_dac: dac,
        note_voltage_switch,
        velocity_voltage_switch,
        //        spi2,
        nss_dac1,
    };

    (tx, rx, board)
}

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

pub struct SatinBoard {
    //		led_discovery: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>>
    pub led1: hal::gpio::gpioh::PH3<hal::gpio::Output<hal::gpio::PushPull>>,
    pub gate_out: hal::gpio::gpioa::PA9<hal::gpio::Output<hal::gpio::PushPull>>,
    pub clock_out: hal::gpio::gpioc::PC6<hal::gpio::Output<hal::gpio::PushPull>>,
    pub note_velocity_dac: Dac,
    pub note_voltage_switch: hal::gpio::gpioa::PA0<hal::gpio::Output<hal::gpio::PushPull>>,
    pub velocity_voltage_switch: hal::gpio::gpioa::PA1<hal::gpio::Output<hal::gpio::PushPull>>,
    /*    pub spi2: Spi<
        SPI2,
        (
            gpiod::PD3<Alternate<AF5>>,
            NoMiso,
            gpioi::PI3<Alternate<AF5>>,
        ),
    >, */
    pub nss_dac1: hal::gpio::gpioa::PA8<hal::gpio::Output<hal::gpio::PushPull>>,
}

impl SatinBoard {
    pub fn out_put_note(&mut self, note: u8, velocity: u8) {
        //hprintln!("mpe:{:?}", note).unwrap();
        //enable gate out
        self.gate_out.set_high();
        //for test set dac1 level depend on the note. TODO convertion table.
        //note
        self.note_velocity_dac
            .dac1()
            .write(DacWord::B8_ALIGN_R(note));
        //Velocity
        self.note_velocity_dac
            .dac2()
            .write(DacWord::B8_ALIGN_R(velocity));
    }

    pub fn write_sp2_cv1(&mut self, data: u16) {
        //use dac1 nns to low to selec the dac.
        self.nss_dac1.set_low();

        //        self.spi2.write(&data).unwrap();

        //  HAL_SPI_Transmit(&hspi2,(uint8_t*)data_array, sizeof(data_array), (uint32_t)1000);

        self.nss_dac1.set_high();
    }
}