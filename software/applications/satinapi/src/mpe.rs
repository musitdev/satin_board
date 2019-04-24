use crate::midi::{MidiMessage, MidiMessageStatus};

const CONTINUOUS_PER_NOTE_CONTROL_CHANGE: u8 = 0x4A;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MPEMessage {
    ChannelMod {
        //main channel can be 1 ou 16
        main_channel: u8,
        channel_used: u8,
    },
    PitchBendRangeMSB {
        channel: u8,
        range: u8,
    },
    PitchBend {
        channel: u8,
        value: u8,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MPEControl {
    XAxis,
    YAxis,
    ZAxis,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ControlValue {
    U8(u8),
    U14(u16),
    U16(u16),
}

impl ControlValue {
    pub fn new_u14(msb: u8, lsb: u8) -> ControlValue {
        let mut val: u16 = (msb & 0x7f).into();
        val = val << 7;
        val += Into::<u16>::into(lsb & 0x7f);
        ControlValue::U14(val)
    }
    pub fn new_u16(msb: u8, lsb: u8) -> ControlValue {
        let mut val: u16 = msb.into();
        val = val << 8;
        val += Into::<u16>::into(lsb);
        ControlValue::U16(val)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MPEEvent {
    NoEvent,
    GlobalNoteChange {
        control: MPEControl,
        value: ControlValue,
    },
    PerNoteChange {
        control: MPEControl,
        value: ControlValue,
    },
    NoteOn {
        note: u8,
        velocity: u8,
    },
    NoteOff,
    OtherMPE(MidiMessage),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MonoNoteMPEManager {
    main_channel: u8,
    number_of_note_channels: u8,
    note_channel: Option<u8>,
}

impl MonoNoteMPEManager {
    pub fn new() -> MonoNoteMPEManager {
        MonoNoteMPEManager {
            //default zone always main channel at 0.
            main_channel: 0,
            number_of_note_channels: 15,
            note_channel: None,
        }
    }

    pub fn manage_message(&mut self, message: MidiMessage) -> MPEEvent {
        let channel = message.get_channel();
        match (
            Into::<MidiMessageStatus>::into(message.status),
            message.data1,
            message.data2,
        ) {
            //only per channel note on of off
            (MidiMessageStatus::EventNoteOff, _, _) | (MidiMessageStatus::EventNoteOn, _, 0) => {
                if let Some(note_channel) = self.note_channel {
                    if note_channel == channel {
                        self.note_channel = None;
                        MPEEvent::NoteOff
                    } else {
                        MPEEvent::OtherMPE(message)
                    }
                } else {
                    MPEEvent::NoteOff
                }
            }
            (MidiMessageStatus::EventNoteOn, data1, data2) => {
                if let Some(note_channel) = self.note_channel {
                    if note_channel == channel {
                        MPEEvent::NoteOn {
                            note: data1,
                            velocity: data2,
                        }
                    } else {
                        MPEEvent::OtherMPE(message)
                    }
                } else {
                    self.note_channel = Some(channel);
                    MPEEvent::NoteOn {
                        note: data1,
                        velocity: data2,
                    }
                }
            }
            data @ _ => {
                if let Some(note_channel) = self.note_channel {
                    if note_channel == channel {
                        match data {
                            (
                                MidiMessageStatus::EventControlChange,
                                CONTINUOUS_PER_NOTE_CONTROL_CHANGE,
                                data2,
                            ) if channel == 0 || channel == 15 => MPEEvent::GlobalNoteChange {
                                control: MPEControl::YAxis,
                                value: ControlValue::U8(data2),
                            },
                            //CC74 (for Y-axis movement)
                            (
                                MidiMessageStatus::EventControlChange,
                                CONTINUOUS_PER_NOTE_CONTROL_CHANGE,
                                data2,
                            ) => MPEEvent::PerNoteChange {
                                control: MPEControl::YAxis,
                                value: ControlValue::U8(data2),
                            },
                            //
                            (MidiMessageStatus::EventVelocityChange, _, data2) => {
                                MPEEvent::PerNoteChange {
                                    //data1 is the note. TODO verify the note with the channel one.
                                    control: MPEControl::ZAxis,
                                    value: ControlValue::U8(data2),
                                }
                            }
                            //(for X-axis movement)
                            (MidiMessageStatus::PitchBend, data1, data2) => {
                                MPEEvent::PerNoteChange {
                                    control: MPEControl::XAxis,
                                    value: ControlValue::new_u14(data1, data2),
                                }
                            }
                            //Channel Pressure (for finger pressure) X_Axis
                            (MidiMessageStatus::AfterTouch, data1, data2) => {
                                MPEEvent::PerNoteChange {
                                    control: MPEControl::XAxis,
                                    value: ControlValue::new_u14(data1, data2),
                                }
                            }
                            _ => MPEEvent::NoEvent,
                            /*
                            MidiMessageStatus::EventNoteOn => 0x90,
                            MidiMessageStatus::EventNoteOff => 0x90,
                            MidiMessageStatus::EventVelocityChange => 0xA0, ou Polyphonic aftertouch
                            MidiMessageStatus::EventControlChange => 0xB0,
                            MidiMessageStatus::EventProgramChange => 0xC0,
                            MidiMessageStatus::AfterTouch => 0xD0,
                            MidiMessageStatus::PitchBend => 0xE0,
                            MidiMessageStatus::StartProprietary => 0xF0,
                            MidiMessageStatus::SongPosition => 0xF2,
                            MidiMessageStatus::SongSelect => 0xF3,
                            MidiMessageStatus::TuneRequest => 0xF6,
                            MidiMessageStatus::EndProprietary => 0xF7,
                            MidiMessageStatus::Sync => 0xF8,
                            MidiMessageStatus::Start => 0xFA,
                            MidiMessageStatus::Continue => 0xFB,
                            MidiMessageStatus::Stop => 0xFC,
                            MidiMessageStatus::ActiveSense => 0xFE,
                            MidiMessageStatus::Reset => 0xFF, */
                        }
                    } else {
                        MPEEvent::OtherMPE(message)
                    }
                } else {
                    MPEEvent::OtherMPE(message)
                }
            }
        }

        //event on other channel
    }
}
