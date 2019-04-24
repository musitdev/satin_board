use crate::midi::MidiMessage;

//12 bit convertion table for 0-5v 12 notes.
const cv_table: [u16; 12] = [
    2000, 2119, 2245, 2378, 2520, 2670, 2828, 2997, 3175, 3364, 3564, 3775,
];

pub fn convert_note_to_cv(base_note: MidiMessage, pitch_bend: usize) -> u16 {
    cv_table[base_note % 12]
}
