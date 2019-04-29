#![no_std]

pub mod cv;
pub mod midi;
pub mod mpe;
//mod utils;

//pub use midi::{MidiMessage, MidiEvent, MidiBuffer};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
