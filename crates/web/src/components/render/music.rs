use crate::bindings::get_player;
use leptos::*;
use leptos_dom::helpers::IntervalHandle;
use scamper_rs::{
    ast::Function,
    modules::music::{Composition, Duration, Mod},
};

#[derive(Debug, Clone, Copy)]
struct Note {
    time: f64,
    duration: f64,
    note: f64,
    instrument: u8,
    velocity: f64,
}

#[derive(Debug, Clone)]
struct Trigger {
    time: f64,
    callback: Function,
}

fn duration_to_time_ms(beat: Duration, bpm: f64, duration: Duration) -> f64 {
    let duration = duration.numerator / duration.denominator;
    let beat_duration = beat.numerator / beat.denominator;
    duration / (beat_duration * bpm) * 60.0 * 1000.0
}

fn freq_to_note(freq: f64) -> f64 {
    (freq / 440.0).log2() * 12.0 + 69.0
}

fn process_compositions(
    compositions: Vec<Composition>,
    beat: Duration,
    bpm: f64,
    velocity: f64,
    start_time: f64,
    instrument: u8,
    is_parallel: bool,
) -> (Option<Vec<Note>>, Option<Vec<Trigger>>, f64) {
    let mut notes = Vec::new();
    let mut triggers = Vec::new();
    let mut end_time = if is_parallel { 0.0 } else { start_time };

    for composition in compositions {
        let (notes_, triggers_, end_time_) = process_composition(
            beat,
            bpm,
            velocity,
            if is_parallel { start_time } else { end_time },
            instrument,
            composition,
        );

        if let Some(notes_) = notes_ {
            notes.extend(notes_);
        }
        if let Some(triggers_) = triggers_ {
            triggers.extend(triggers_);
        }

        if is_parallel {
            end_time = end_time.max(end_time_);
        } else {
            end_time = end_time_;
        }
    }

    notes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    triggers.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    (Some(notes), Some(triggers), end_time)
}

fn process_composition(
    beat: Duration,
    bpm: f64,
    velocity: f64,
    start_time: f64,
    instrument: u8,
    composition: Composition,
) -> (Option<Vec<Note>>, Option<Vec<Trigger>>, f64) {
    match composition {
        Composition::Empty => (None, None, start_time),
        Composition::Note(note) => {
            let note = Note {
                time: start_time,
                duration: duration_to_time_ms(beat, bpm, note.duration),
                note: note.value,
                instrument,
                velocity: velocity / 127.0,
            };
            (Some(vec![note]), None, start_time + note.duration)
        }
        Composition::NoteFreq(note) => {
            let note = Note {
                time: start_time,
                duration: duration_to_time_ms(beat, bpm, note.duration),
                note: freq_to_note(note.value),
                instrument,
                velocity: velocity / 127.0,
            };
            (Some(vec![note]), None, start_time + note.duration)
        }
        Composition::Rest(duration) => {
            let duration = duration_to_time_ms(beat, bpm, duration);
            (None, None, start_time + duration)
        }
        Composition::Trigger(callback) => {
            let trigger = Trigger {
                time: start_time,
                callback,
            };
            (None, Some(vec![trigger]), start_time)
        }
        Composition::Parallel(compositions) => process_compositions(
            compositions,
            beat,
            bpm,
            velocity,
            start_time,
            instrument,
            true,
        ),
        Composition::Sequence(compositions) => process_compositions(
            compositions,
            beat,
            bpm,
            velocity,
            start_time,
            instrument,
            false,
        ),
        Composition::Pickup(pickup) => {
            let (mut pickup_notes, pickup_triggers, pickup_end_time) =
                process_composition(beat, bpm, velocity, start_time, instrument, *pickup.pickup);
            let pickup_duration = pickup_end_time - start_time;

            // if the pickup would start in negative time,  rebase the composition to start with the pickup instead
            // todo: are we sure this is even correct? i should test it
            let (notes_notes, notes_triggers, notes_end_time) =
                if start_time - pickup_duration < 0.0 {
                    if let Some(notes) = pickup_notes {
                        pickup_notes = Some(
                            notes
                                .iter()
                                .map(|note| Note {
                                    time: note.time + pickup_duration,
                                    ..*note
                                })
                                .collect(),
                        );
                    }
                    process_composition(
                        beat,
                        bpm,
                        velocity,
                        pickup_duration,
                        instrument,
                        *pickup.notes,
                    )
                } else {
                    if let Some(notes) = pickup_notes {
                        pickup_notes = Some(
                            notes
                                .iter()
                                .map(|note| Note {
                                    time: note.time - pickup_duration,
                                    ..*note
                                })
                                .collect(),
                        );
                    }
                    process_composition(beat, bpm, velocity, start_time, instrument, *pickup.notes)
                };

            // combine results from pickup and notes
            let notes = if let Some(mut pickup_notes) = pickup_notes {
                if let Some(notes_notes) = notes_notes {
                    pickup_notes.extend(notes_notes);
                }
                Some(pickup_notes)
            } else {
                notes_notes
            };

            let triggers = if let Some(mut pickup_triggers) = pickup_triggers {
                if let Some(notes_triggers) = notes_triggers {
                    pickup_triggers.extend(notes_triggers);
                }
                Some(pickup_triggers)
            } else {
                notes_triggers
            };

            (notes, triggers, notes_end_time)
        }
        Composition::Mod(composition, modification) => match modification {
            Mod::Percussion => {
                process_composition(beat, bpm, velocity, start_time, instrument, *composition)
            }
            Mod::Tempo(tempo) => process_composition(
                tempo.beat,
                tempo.bpm,
                velocity,
                start_time,
                instrument,
                *composition,
            ),
            Mod::Dynamics(amount) => {
                process_composition(beat, bpm, amount, start_time, instrument, *composition)
            }
            Mod::Instrument(instr) => {
                process_composition(beat, bpm, velocity, start_time, instr, *composition)
            }
        },
    }
}

fn read_composition(composition: Composition) -> (Vec<Note>, Vec<Trigger>) {
    let (notes, triggers, _) =
        process_composition(Duration::new(1.0, 4.0), 120.0, 64.0, 0.0, 0, composition);
    (notes.unwrap_or_default(), triggers.unwrap_or_default())
}

#[component]
pub fn CompositionView(composition: Composition) -> impl IntoView {
    let (notes, triggers) = read_composition(composition);

    let trigger_handle = create_rw_signal(None::<Option<IntervalHandle>>);

    let play = move |_| {
        let player = get_player();
        let notes = notes.clone();
        let triggers = triggers.clone();

        for note in notes {
            player.play_note(
                note.time,
                note.duration,
                note.note,
                note.instrument,
                note.velocity,
            );
        }

        if !triggers.is_empty() {
            let trigger_index = create_rw_signal(0);

            let handle = set_interval_with_handle(
                move || {
                    let current_time = player.get_time();

                    while trigger_index.get() < triggers.len() {
                        let trigger = &triggers[trigger_index.get()];
                        if current_time >= trigger.time {
                            // todo: should handle output/error somehow?
                            let _ = trigger.callback.call(&[]);
                            trigger_index.set(trigger_index.get() + 1);
                        } else {
                            break;
                        }
                    }

                    if trigger_index.get() >= triggers.len() {
                        trigger_handle.set(None);
                    }
                },
                std::time::Duration::from_millis(10),
            );

            trigger_handle.set(Some(handle.ok()));
        }
    };

    let stop = move |_| {
        let player = get_player();
        player.stop();

        if let Some(Some(handle)) = trigger_handle.get() {
            handle.clear();
        }
        trigger_handle.set(None);
    };

    view! {
        <span>
            <button on:click=play>"▶"</button>
            <button on:click=stop>"■"</button>
        </span>
    }
}
