#[macro_use]
extern crate vst;
extern crate rand;

use vst::buffer::AudioBuffer;
use vst::plugin::{Plugin, Info, Category};
use vst::event::Event;
use vst::api::Events;
use rand::random;


#[derive(Default)]
struct Sleep {
    notes: u8
}

impl Plugin for Sleep {
    fn get_info(&self) -> Info {
        Info {
            name: "Sleep".to_string(),
            unique_id: 666,
            inputs: 0,
            outputs: 2,
            category: Category::Synth,
            ..Default::default()
        }
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => {
                    // Check if it's a noteon or noteoff event.
                    // This is difficult to explain without knowing how the MIDI standard works.
                    // Basically, the first byte of data tells us if this signal is a note on event
                    // or a note off event.  You can read more about that here: 
                    // https://www.midi.org/specifications/item/table-1-summary-of-midi-message
                    match ev.data[0] {
                        // note on
                        144 => self.notes += 1u8,
                        // note off
                        128 => self.notes -= 1u8,
                        _ => (),
                    }
                },
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        if self.notes == 0 {return};
        let (_, output_buffer) = buffer.split();

        for output_channel in output_buffer.into_iter() {
            for output_sample in output_channel {
                *output_sample = (random::<f32>() - 0.5f32) * 2f32;
            }
        }
    }
}


plugin_main!(Sleep);