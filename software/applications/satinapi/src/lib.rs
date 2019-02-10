#![no_std]

mod midi;
//mod utils;

pub use midi::{MidiMessage, MidiEvent, MidiBuffer};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
