use nb;
//use stm32f7::stm32f7x6 as pac;
use processor_hal::device as pac;
use stm32f7xx_hal as processor_hal;
//use core::fmt::Write;
//use cortex_m_rt::entry;
use crate::ehal::digital::v2::OutputPin;
//use crate::ehal::spi::{Mode, Phase, Polarity};
use cortex_m::peripheral::DWT;
use cortex_m_semihosting::hprintln;
use nb::block;
use processor_hal::{
    //   gpio::*,
    prelude::*,
    serial::{self, Event, Rx, Serial, Tx},
    spi::{self, Spi},
    //    timer::{Event as TimerEvent, Timer},
};
use satin_hal::dac::{Dac, DacWord, DacWriter};

//pub const CONST_TIMER_FREQ: u32 = 1;

/// SPI mode
/*pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
}; */

pub struct SatinBoard {
    //      led_discovery: hal::gpio::gpiob::PB7<hal::gpio::Output<hal::gpio::PushPull>>
    pub led1:
        processor_hal::gpio::gpioh::PH3<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub gate_out:
        processor_hal::gpio::gpioa::PA9<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub clock_out:
        processor_hal::gpio::gpioc::PC6<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub note_velocity_dac: Dac,
    pub note_voltage_switch:
        processor_hal::gpio::gpioa::PA0<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub velocity_voltage_switch:
        processor_hal::gpio::gpioa::PA1<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub spi2: processor_hal::spi::Spi<
        pac::SPI2,
        (
            processor_hal::gpio::gpiod::PD3<
                processor_hal::gpio::Alternate<processor_hal::gpio::AF5>,
            >,
            processor_hal::spi::NoMiso,
            processor_hal::gpio::gpioi::PI3<
                processor_hal::gpio::Alternate<processor_hal::gpio::AF5>,
            >,
        ),
        processor_hal::spi::Enabled<u16>,
    >,
    pub dac1cv_clear:
        processor_hal::gpio::gpioi::PI2<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,

    pub nss_dac1:
        processor_hal::gpio::gpioa::PA8<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub nss_dac2:
        processor_hal::gpio::gpiod::PD8<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub nss_dac3:
        processor_hal::gpio::gpiod::PD0<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub nss_dac4:
        processor_hal::gpio::gpiod::PD1<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>,
    pub tx: Tx<pac::USART2>,
    pub rx: Rx<pac::USART2>,
    pub rx_buf: satinapi::midi::MidiBuffer,
}

impl SatinBoard {
    pub fn read_one_byte(
        &mut self,
    ) -> Result<Option<satinapi::midi::MidiEvent>, processor_hal::serial::Error> {
        let c = block!(self.rx.read())?;
        if let Some(message) = self.rx_buf.push_byte(c) {
            Ok(Some(satinapi::midi::MidiEvent {
                message,
                timestamp: 0,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn write_byte_on_serial(&mut self, byte: u8) -> Result<(), processor_hal::serial::Error> {
        //hprintln!("Wb:{:?}", b).unwrap();
        block!(self.tx.write(byte))?;
        block!(self.tx.flush())?;
        Ok(())
    }

    pub fn output_note(&mut self, note: u8, velocity: u8) {
        if velocity == 0 {
            self.output_note_off();
        } else {
            let cv_note = crate::satinapi::cv::convert_note_to_cv(note, 0);
            //hprintln!("no:{:?} cv:{:?} v:{}", note, cv_note, velocity).unwrap();
            //enable gate out
            self.gate_out.set_high().unwrap();
            //for test set dac1 level depend on the note. TODO convertion table.
            //note
            self.note_velocity_dac
                .dac1()
                .write(DacWord::B8_ALIGN_R(cv_note));
            //Velocity
            self.note_velocity_dac
                .dac2()
                .write(DacWord::B8_ALIGN_R(velocity * 2));
        }
    }
    pub fn output_note_off(&mut self) {
        //disable gate out
        self.gate_out.set_low().unwrap();
    }

    pub fn write_sp2_cv1(&mut self, data: u16) -> Result<(), nb::Error<processor_hal::spi::Error>> {
        //use dac1 nns to low to selec the dac.
        //        self.dac1cv_clear.set_low();
        //        self.dac1cv_clear.set_high();
        write_spi2(data, &mut self.spi2, &mut self.nss_dac1)?;
        Ok(())
    }
    pub fn write_sp2_cv2(&mut self, data: u16) -> Result<(), nb::Error<processor_hal::spi::Error>> {
        //use dac1 nns to low to selec the dac.
        //        self.dac1cv_clear.set_low();
        //        self.dac1cv_clear.set_high();
        write_spi2(data, &mut self.spi2, &mut self.nss_dac2)?;
        Ok(())
    }
    pub fn write_sp2_cv3(&mut self, data: u16) -> Result<(), nb::Error<processor_hal::spi::Error>> {
        //use dac1 nns to low to selec the dac.
        //        self.dac1cv_clear.set_low();
        //        self.dac1cv_clear.set_high();
        write_spi2(data, &mut self.spi2, &mut self.nss_dac3)?;
        Ok(())
    }
    pub fn write_sp2_cv4(&mut self, data: u16) -> Result<(), nb::Error<processor_hal::spi::Error>> {
        //use dac1 nns to low to selec the dac.
        //        self.dac1cv_clear.set_low();
        //        self.dac1cv_clear.set_high();
        write_spi2(data, &mut self.spi2, &mut self.nss_dac4)?;
        Ok(())
    }
}

fn write_spi2<PIN>(
    data: u16,
    spi2: &mut processor_hal::spi::Spi<
        pac::SPI2,
        (
            processor_hal::gpio::gpiod::PD3<
                processor_hal::gpio::Alternate<processor_hal::gpio::AF5>,
            >,
            processor_hal::spi::NoMiso,
            processor_hal::gpio::gpioi::PI3<
                processor_hal::gpio::Alternate<processor_hal::gpio::AF5>,
            >,
        ),
        processor_hal::spi::Enabled<u16>,
    >, /*&mut Spi<
           stm32f7::stm32f7x6::SPI2,
           (
               gpiod::PD3<Alternate<AF5>>,
               spi::NoMiso,
               gpioi::PI3<Alternate<AF5>>,
           ),
       >, */
    nss: &mut PIN,
) -> Result<(), nb::Error<processor_hal::spi::Error>>
where
    PIN: OutputPin,
{
    //    let valeur_dac: u16 = 0x3FFF & data; //clear control bit D15, D14
    //    let valeur_dac: u16 = 0x4000 | valeur_dac; //set controle bit D14 up

    let word: u16 = (0b01 << 14) |   // write-through mode
            (data & 0x3fff); // data bits

    nss.set_low()
        .unwrap_or_else(|_| hprintln!("write_spi2 nss set low error.").unwrap());
    if let Err(err) = spi2.write(&[word]) {
        return Err(nb::Error::Other(err));
    }

    /*    if let Err(err) = spi2.read() {
        //read one time at the end to end the transation and way the end of the send
        match err {
            nb::Error::WouldBlock => (), //do  nothing
            _ => return Err(nb::Error::Other(spi::Error::EndTranscationReadError)),
        }
    }
    if let Err(err) = spi2.read() {
        //read one time at the end to end the transation and way the end of the send
        match err {
            nb::Error::WouldBlock => (), //do  nothing
            _ => return Err(nb::Error::Other(spi::Error::EndTranscationReadError)),
        }
    } */
    nss.set_high()
        .unwrap_or_else(|_| hprintln!("write_spi2 nss set high error.").unwrap());

    Ok(())
}

pub fn init_board(device: pac::Peripherals) -> SatinBoard {
    let mut rcc = device.RCC.constrain();

    let gpiod = device.GPIOD.split();
    let gpioi = device.GPIOI.split();
    //init SPIT before sysclk int that borrow rcc
    //Dac1, Dac2, Dac 3 Dac 4 are controled via SPI2
    //SPI2:
    /*SPI2 GPIO Configuration
    PD3     ------> SPI2_SCK
    pas de SPI2 MISO.
    PI3     ------> SPI2_MOSI
    */
    let sck = gpiod.pd3.into_alternate_af5();
    //    let miso = NoMiso;
    let mosi = gpioi.pi3.into_alternate_af5();

    let spi2 = Spi::new(device.SPI2, (sck, spi::NoMiso, mosi)).enable::<u16>(
        &mut rcc,
        spi::ClockDivider::DIV32,
        embedded_hal::spi::MODE_0,
    );

    /*    Spi::spi2(
        device.SPI2,
        (sck, miso, mosi),
        MODE,
        // 1.mhz(),
        100.khz().into(),
        clocks,
    ); */

    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    //write DWT to activate RTFM timer.
    unsafe { (*DWT::ptr()).lar.write(0xC5ACCE55) };

    //        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();
    //    let mut systick = Timer::tim2(device.TIM2, CONST_TIMER_FREQ.hz(), clocks, &mut rcc.apb1);
    //    systick.listen(TimerEvent::TimeOut);

    //serial init
    let gpioa = device.GPIOA.split();
    let gpiob = device.GPIOB.split();
    let gpioc = device.GPIOC.split();

    let tx = gpiod.pd5.into_alternate_af7();
    let rx = gpiod.pd6.into_alternate_af7();
    //        let mut serial = Serial::usart2(device.USART2, (tx, rx), 115_200.bps(), clocks, false);
    let mut serial = Serial::new(
        device.USART2,
        (tx, rx),
        clocks,
        serial::Config {
            baud_rate: 31_250.bps(),
            oversampling: serial::Oversampling::By16,
        },
    );
    //        serial.listen(Event::Txe);
    serial.listen(Event::Rxne);

    let (tx, rx) = serial.split();

    //Dac and SPI GPIO init

    let gpiog = device.GPIOG.split();
    let gpioh = device.GPIOH.split();

    //init led 1 to high
    let mut led1: processor_hal::gpio::gpioh::PH3<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioh.ph3.into_push_pull_output();
    led1.set_high()
        .unwrap_or_else(|_| hprintln!("init_board led1 set high error.").unwrap());

    //init STM32 dac
    //SWITCH_NOTE 0-10V / -5,5V: PA0
    let mut note_voltage_switch: processor_hal::gpio::gpioa::PA0<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioa.pa0.into_push_pull_output();
    note_voltage_switch
        .set_low()
        .unwrap_or_else(|_| hprintln!("init_board note_voltage_switch set low error.").unwrap());

    //  SWITCH_VELOCITE 0-10V / -5,5V: PA1
    let mut velocity_voltage_switch: processor_hal::gpio::gpioa::PA1<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioa.pa1.into_push_pull_output();
    velocity_voltage_switch.set_low().unwrap_or_else(|_| {
        hprintln!("init_board velocity_voltage_switch set low error.").unwrap()
    });

    //Dac1 pin CV Note: ST_DAC_OUT1 PA4
    //    let dac1_pin: () = gpioa.pa4.into_analog_output();
    //Dac2 pin CV vélocity: ST_DAC_OUT2: PA5
    //    let dac2_pin = gpioa.pa5.into_analog_output();

    let dac = Dac::new(device.DAC);
    let mut dac1 = dac.create_dac_channel1();
    dac1.enable();
    dac1.disable_output_buffer();
    dac1.disable_trigger();

    let mut dac2 = dac.create_dac_channel2();
    dac2.enable();
    dac2.disable_output_buffer();
    dac2.disable_trigger();

    //test write
    dac1.write(DacWord::B8_ALIGN_R(0));
    dac2.write(DacWord::B8_ALIGN_R(0));

    hprintln!("end dac init").unwrap();

    //Configure Gate / Clock
    //clock out PC6
    let mut clock_out: processor_hal::gpio::gpioc::PC6<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioc.pc6.into_push_pull_output();
    clock_out
        .set_low()
        .unwrap_or_else(|_| hprintln!("init_board clock_out set low error.").unwrap());

    // gate out PA9
    let mut gate_out: processor_hal::gpio::gpioa::PA9<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioa.pa9.into_push_pull_output();
    gate_out
        .set_high()
        .unwrap_or_else(|_| hprintln!("init_board gate_out set_high error.").unwrap());

    //Gate PA9 + invertion gate: PI8
    let mut invertion_gate: processor_hal::gpio::gpioi::PI8<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioi.pi8.into_push_pull_output();
    invertion_gate
        .set_low()
        .unwrap_or_else(|_| hprintln!("init_board invertion_gate set low error.").unwrap());

    //clock out PC6 INVERSION_CLOCK PI9
    let mut invertion_clock: processor_hal::gpio::gpioi::PI9<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioi.pi9.into_push_pull_output();
    invertion_clock
        .set_low()
        .unwrap_or_else(|_| hprintln!("init_board invertion_clock set low error.").unwrap());

    //4 SPI CV dac.
    //Dac1 CS PA8
    let mut nss_dac1 = gpioa.pa8.into_push_pull_output();
    nss_dac1
        .set_high()
        .unwrap_or_else(|_| hprintln!("init_board nss_dac1 set_high error.").unwrap()); //no data

    //Dac2 CS PD8
    let mut nss_dac2 = gpiod.pd8.into_push_pull_output();
    nss_dac2
        .set_high()
        .unwrap_or_else(|_| hprintln!("init_board nss_dac2 set_high error.").unwrap()); //no data

    //DAC3_CS PD0
    let mut nss_dac3 = gpiod.pd0.into_push_pull_output();
    nss_dac3
        .set_high()
        .unwrap_or_else(|_| hprintln!("init_board nss_dac3 set_high error.").unwrap()); //no data

    //DAC4_CS PD1
    let mut nss_dac4 = gpiod.pd1.into_push_pull_output();
    nss_dac4
        .set_high()
        .unwrap_or_else(|_| hprintln!("init_board nss_dac4 set_high error.").unwrap()); //no data

    //DAC-CV clear init PI2
    let mut dac1cv_clear: processor_hal::gpio::gpioi::PI2<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioi.pi2.into_push_pull_output();
    dac1cv_clear
        .set_high()
        .unwrap_or_else(|_| hprintln!("init_board dac1cv_clear set_high error.").unwrap());

    //DAC-CV CV3:

    //SWITCH_CV_n1 switch 0-10 / -5,5v  PB14
    let mut cv3_voltage_switch: processor_hal::gpio::gpiob::PB14<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpiob.pb14.into_push_pull_output();
    cv3_voltage_switch
        .set_high()
        .unwrap_or_else(|_| hprintln!("init_board cv3_voltage_switch set_high error.").unwrap());

    //SWITCH_CVn2_OUTSwitch 0-5v ou (0-10 / -5,5v)  PB11
    let mut cv3_end_voltage_switch: processor_hal::gpio::gpiob::PB11<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpiob.pb11.into_push_pull_output();
    cv3_end_voltage_switch.set_high().unwrap_or_else(|_| {
        hprintln!("init_board cv3_end_voltage_switch set_high error.").unwrap()
    });

    //SWITCH_CV_n3 switch 0-10 / -5,5v  PH12
    let mut cv4_voltage_switch: processor_hal::gpio::gpioh::PH12<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpioh.ph12.into_push_pull_output();
    cv4_voltage_switch
        .set_low()
        .unwrap_or_else(|_| hprintln!("init_board cv4_voltage_switch set_low error.").unwrap());

    //SWITCH_CVn4_OUTSwitch 0-5v ou (0-10 / -5,5v)  PB12
    let mut cv4_end_voltage_switch: processor_hal::gpio::gpiob::PB12<
        processor_hal::gpio::Output<processor_hal::gpio::PushPull>,
    > = gpiob.pb12.into_push_pull_output();
    cv4_end_voltage_switch
        .set_low()
        .unwrap_or_else(|_| hprintln!("init_board cv4_end_voltage_switch set_low error.").unwrap());

    //switch CV(5-6) out or  stéreo DAC:   SWITCH_CV_STEREO OUT: PG2
    let mut cv_st_out_switch = gpiog.pg2.into_push_pull_output();
    cv_st_out_switch
        .set_low()
        .unwrap_or_else(|_| hprintln!("init_board cv_st_out_switch set_low error.").unwrap()); //no data

    let board = SatinBoard {
        //      led_discovery: processor_hal::gpio::gpiob::PB7<processor_hal::gpio::Output<processor_hal::gpio::PushPull>>
        led1: led1,
        gate_out,
        clock_out,
        note_velocity_dac: dac,
        note_voltage_switch,
        velocity_voltage_switch,
        spi2,
        dac1cv_clear,
        nss_dac1,
        nss_dac2,
        nss_dac3,
        nss_dac4,
        tx,
        rx,
        rx_buf: satinapi::midi::MidiBuffer::new(),
    };
    hprintln!("end board init").unwrap();

    board
}
