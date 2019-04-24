extern crate satinapi;

use satinapi::midi::{MidiMessage, MidiMessageStatus};
use satinapi::mpe::{MPEEvent, MonoNoteMPEManager};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
    let f = match File::open("dump.txt") {
        Ok(v) => v,
        Err(e) => {
            println!("file open error:{:?}", e);
            return;
        }
    };
    let mut mpe_manager = MonoNoteMPEManager::new();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l);
        let message = read_line(&l);
        //        let midi_status = Into::<MidiMessageStatus>::into(message.status);
        //        println!("midi_status:{:?}", midi_status);
        //        println!("midi {:?}", message);
        let event = mpe_manager.manage_message(message);
        println!("event {:?}", event);
    }

    new_u14(0x6E, 0x3F);
}

//864,173.5769,24:0,2,Control change,74,64,
fn read_line(line: &str) -> MidiMessage {
    let mut records = line.split(' ');
    let status = records.next().unwrap();
    let data1 = records.next().unwrap();
    let data2 = records.next().unwrap();
    let data: [u8; 3] = [
        u8::from_str_radix(status, 16).unwrap(),
        u8::from_str_radix(data1, 16).unwrap(),
        u8::from_str_radix(data2, 16).unwrap(),
    ];
    data.into()
}

pub fn new_u14(msb: u8, lsb: u8) {
    let mut val: u16 = (msb & 0x7f).into();
    println!("val1:{:?}", val);
    val = val << 7;
    println!("val2:{:?}", val);
    let val_int = Into::<u16>::into(lsb & 0x7f);
    println!("val_int:{:?}", val_int);
    val += val_int;
    println!("val3:{:?}", val);
}

fn status_from_str(status: &str) -> MidiMessageStatus {
    match status {
        "Control change" => MidiMessageStatus::EventControlChange,
        "Note on" => MidiMessageStatus::EventNoteOn,
        "Polyphonic aftertouch" => MidiMessageStatus::AfterTouch,
        "Pitch bend" => MidiMessageStatus::PitchBend,
        "Note off" => MidiMessageStatus::EventNoteOff,
        _ => panic!("unknown token:{:?}", status),
    }
}
