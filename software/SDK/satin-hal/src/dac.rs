//use stm32f7::stm32f7x6 as pac;
use stm32f7xx_hal as processor_hal;

use processor_hal::device::DAC;
use processor_hal::device::RCC;

#[derive(PartialEq, PartialOrd, Clone, Eq, Ord)]
pub enum DacWord {
    B8_ALIGN_R(u8),
    B12_ALIGN_R(u16),
    B12_ALIGN_L(u16),
}

/// Write DAC interface
///
/// Some DAC interfaces support different data sizes (8 bits, 12 bits, etc.) with different alignement.
/// data use DacWord type to define the world size and alignement.
pub trait DacWriter {
    /// write a single word from the DAC interface using specified data
    fn write(&mut self, data: DacWord);
}

pub struct Dac {
    dac: DAC,
}

impl Dac {
    pub fn new(dac: DAC) -> Dac {
        //Init dac
        // NOTE(unsafe) This executes only during initialisation
        let rcc = unsafe { &(*RCC::ptr()) };
        rcc.apb1enr.modify(|_, w| w.dacen().enabled()); //enable dac

        Dac { dac: dac }
    }

    pub fn create_dac_channel1(&self) -> Dac1 {
        //offset 4 for GPIOA::PA4 dac1
        unsafe {
            &(*processor_hal::device::GPIOA::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << 4)) | (0b11 << 4)));
            &(*processor_hal::device::GPIOA::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << 4)) | (0b00 << 4)));
        };

        Dac1 { dac: &self.dac }
    }

    pub fn dac1(&self) -> Dac1 {
        Dac1 { dac: &self.dac }
    }
    pub fn create_dac_channel2(&self) -> Dac2 {
        //offset 4 for GPIOA::PA4 dac1
        unsafe {
            &(*processor_hal::device::GPIOA::ptr())
                .moder
                .modify(|r, w| w.bits((r.bits() & !(0b11 << 5)) | (0b11 << 5)));
            &(*processor_hal::device::GPIOA::ptr())
                .pupdr
                .modify(|r, w| w.bits((r.bits() & !(0b11 << 5)) | (0b00 << 5)));
        };

        Dac2 { dac: &self.dac }
    }

    pub fn dac2(&self) -> Dac2 {
        Dac2 { dac: &self.dac }
    }
}

pub struct Dac1<'a> {
    dac: &'a DAC,
}

impl<'a> Dac1<'a> {
    pub fn enable(&mut self) {
        self.dac.cr.modify(|_, w| w.en1().enabled());
    }

    pub fn disable(&mut self) {
        self.dac.cr.modify(|_, w| w.en1().disabled());
    }

    pub fn enable_output_buffer(&mut self) {
        self.dac.cr.modify(|_, w| w.boff1().enabled());
    }

    pub fn disable_output_buffer(&mut self) {
        self.dac.cr.modify(|_, w| w.boff1().disabled());
    }

    pub fn enable_trigger(&mut self) {
        self.dac.cr.modify(|_, w| w.ten1().enabled());
    }

    pub fn disable_trigger(&mut self) {
        self.dac.cr.modify(|_, w| w.ten1().disabled());
    }
}

pub struct Dac2<'a> {
    dac: &'a DAC,
}

impl<'a> Dac2<'a> {
    pub fn enable(&mut self) {
        self.dac.cr.modify(|_, w| w.en2().enabled());
    }

    pub fn disable(&mut self) {
        self.dac.cr.modify(|_, w| w.en2().disabled());
    }

    pub fn enable_output_buffer(&mut self) {
        self.dac.cr.modify(|_, w| w.boff2().enabled());
    }

    pub fn disable_output_buffer(&mut self) {
        self.dac.cr.modify(|_, w| w.boff2().disabled());
    }

    pub fn enable_trigger(&mut self) {
        self.dac.cr.modify(|_, w| w.ten2().enabled());
    }

    pub fn disable_trigger(&mut self) {
        self.dac.cr.modify(|_, w| w.ten2().disabled());
    }
}

impl<'a> DacWriter for Dac1<'a> {
    fn write(&mut self, data: DacWord) {
        match data {
            DacWord::B8_ALIGN_R(d) => self.dac.dhr8r1.modify(|_, w| w.dacc1dhr().bits(d)),
            DacWord::B12_ALIGN_R(d) => self
                .dac
                .dhr12r1
                .modify(|_, w| unsafe { w.dacc1dhr().bits(d) }),
            DacWord::B12_ALIGN_L(d) => self
                .dac
                .dhr12l1
                .modify(|_, w| unsafe { w.dacc1dhr().bits(d) }),
        }
    }
}

impl<'a> DacWriter for Dac2<'a> {
    fn write(&mut self, data: DacWord) {
        match data {
            DacWord::B8_ALIGN_R(d) => self.dac.dhr8r2.modify(|_, w| w.dacc2dhr().bits(d)),
            DacWord::B12_ALIGN_R(d) => self
                .dac
                .dhr12r2
                .modify(|_, w| unsafe { w.dacc2dhr().bits(d) }),
            DacWord::B12_ALIGN_L(d) => self
                .dac
                .dhr12l2
                .modify(|_, w| unsafe { w.dacc2dhr().bits(d) }),
        }
    }
}
