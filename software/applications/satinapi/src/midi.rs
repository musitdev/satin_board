
/// Represents a Midi message.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
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
	data: [u8;3],
	index: usize,
}

impl MidiBuffer {
	pub fn new() -> MidiBuffer {Default::default()}

	//add the specified byte to the buffer. Return a MidiEvent when it can be constructed.
	pub fn push_byte(&mut self, byte: u8) -> Option<MidiMessage> {
		self.data[self.index] = byte;
		self.index +=1;
		if self.index == 3 {
			self.index =0;
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
			data: [0,0,0],
			index: 0,
		}
    }
}