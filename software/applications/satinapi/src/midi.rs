// These are midi status message types as sent on the wire
const STATUS_EVENT_NOTE_OFF: u8 = 0x80;
const STATUS_EVENT_NOTE_ON: u8 = 0x90;
const STATUS_EVENT_VELOCITY_CHANGE: u8 = 0xA0;
const STATUS_EVENT_CONTROL_CHANGE: u8 = 0xB0;
const STATUS_EVENT_PROGRAM_CHANGE: u8 = 0xC0;
const STATUS_AFTER_TOUCH: u8 = 0xD0;
const STATUS_PITCH_CHANGE: u8 = 0xE0;
const STATUS_START_PROPRIETARY: u8 = 0xF0;
const STATUS_SONG_POSITION: u8 = 0xF2;
const STATUS_SONG_SELECT: u8 = 0xF3;
const STATUS_TUNE_REQUEST: u8 = 0xF6;
const STATUS_END_PROPRIETARY: u8 = 0xF7;
const STATUS_SYNC: u8 = 0xF8;
const STATUS_START: u8 = 0xFA;
const STATUS_CONTINUE: u8 = 0xFB;
const STATUS_STOP: u8 = 0xFC;
const STATUS_ACTIVE_SENSE: u8 = 0xFE;
const STATUS_RESET: u8 = 0xFF;

/// Represents a Midi message.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
}

impl MidiMessage {
    pub fn is_not_on(&self) -> bool {
      self.status == STATUS_EVENT_NOTE_ON
    }

    pub fn is_not_off(&self) -> bool {
      self.status == STATUS_EVENT_NOTE_OFF
    }
}

impl From<[u8; 3]> for MidiMessage {
    fn from(raw: [u8; 3]) -> Self {
        MidiMessage {
            status: raw[0],
            data1: raw[1],
            data2: raw[2],
        }
    }
}
impl Into<[u8; 3]> for MidiMessage {
    fn into(self) -> [u8; 3] {
        [self.status, self.data1, self.data2]
    }
}
impl Into<[u8; 3]> for &MidiMessage {
    fn into(self) -> [u8; 3] {
        [self.status, self.data1, self.data2]
    }
}

/// Represents a time stamped midi event. See also `MidiMessage`
/// timestamp is inited at the start of the card and is updated every ms.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiEvent {
    pub message: MidiMessage,
    pub timestamp: u64,
}

pub struct MidiBuffer {
    data: [u8; 3],
    index: usize,
}

impl MidiBuffer {
    pub fn new() -> MidiBuffer {
        Default::default()
    }

    //add the specified byte to the buffer. Return a MidiEvent when it can be constructed.
    pub fn push_byte(&mut self, byte: u8) -> Option<MidiMessage> {
        self.data[self.index] = byte;
        self.index += 1;
        if self.index == 3 {
            self.index = 0;
            let ret = [self.data[0], self.data[1], self.data[2]];
            Some(MidiMessage::from(ret))
        } else {
            None
        }
    }
}

impl Default for MidiBuffer {
    fn default() -> MidiBuffer {
        MidiBuffer {
            data: [0, 0, 0],
            index: 0,
        }
    }
}
