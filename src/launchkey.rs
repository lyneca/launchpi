use midir::{MidiInput, MidiOutput};

pub struct LaunchKey {
    note_in: MidiInput,
    control_in: MidiInput,
    control_out: MidiOutput
}

impl LaunchKey {
    pub fn new() -> LaunchKey {
        println!("{}", MidiInput::new("test").unwrap().port_count());
        LaunchKey {
            note_in: MidiInput::new("note in").unwrap(),
            control_in: MidiInput::new("control in").unwrap(),
            control_out: MidiOutput::new("control out").unwrap()
        }
    }
}
