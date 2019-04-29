use crate::midi::MidiMessage;

//8 bit convertion table for 0-5v 12 notes.
const CV_TABLE: [u8; 12] = [0, 3, 5, 8, 11, 13, 16, 19, 21, 24, 27, 29];

pub fn convert_note_to_cv(base_note: u8, pitch_bend: usize) -> u8 {
    let mut base: i32 = ((base_note / 12) - 2) as i32;
    if base < 0 {
        base = 0;
    }
    //mutliply by 32 to have a range of 8 octabe (256 / 8 = 32)
    base = base * 32;
    let index = (base_note % 12) as usize;
    let cv: u8 = (base as u8) + CV_TABLE[index];
    cv
}
