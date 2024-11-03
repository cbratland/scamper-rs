use crate::{
    ast::{Contract, Function, Natural, Value},
    interpreter::Env,
    make_range_checker,
};
use scamper_macros::{function, ForeignValue};

pub fn add_to(env: &mut Env) {
    env.register("dur", dur);
    env.register("numerator", numerator);
    env.register("denominator", denominator);

    env.register_value("wn", Duration::new(1.0, 1.0));
    env.register_value("hn", Duration::new(1.0, 2.0));
    env.register_value("qn", Duration::new(1.0, 4.0));
    env.register_value("en", Duration::new(1.0, 8.0));
    env.register_value("sn", Duration::new(1.0, 16.0));
    env.register_value("tn", Duration::new(1.0, 32.0));

    env.register("pitch?", is_pitch_class);
    env.register("octave?", is_octave);
    env.register("note-value?", is_note_value);

    env.register("note", note);
    env.register("note-freq", note_freq);
    env.register("repeat", repeat);
    env.register("empty", empty);
    env.register("rest", rest);
    env.register("trigger", trigger);
    env.register("par", par);
    env.register("seq", seq);
    env.register("pickup", pickup);

    env.register("mod?", mod_q);
    env.register_value("percussion", Mod::Percussion);
    env.register("tempo", tempo);
    env.register("dynamics", dynamics);
    env.register("instrument", instrument);
    env.register("mod", modify);

    env.register("composition?", composition_q);

    // todo: likely need to register these from the web interface
    //   ..or expose some sort of cross-platform "player" instance with configuration options
    // env.register("load-instrument", load_instrument);
    // env.register("load-percussion", load_percussion);
    // env.register("use-high-quality-instruments", use_high_quality_instruments);
    // env.register("play-composition", play_composition);
}

#[derive(Debug, Clone, Copy, ForeignValue)]
pub struct Duration {
    pub numerator: f64,
    pub denominator: f64,
}

impl Duration {
    pub fn new(numerator: f64, denominator: f64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub value: f64,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct Pickup {
    pub pickup: Box<Composition>,
    pub notes: Box<Composition>,
}

#[derive(Debug, Clone)]
pub struct Tempo {
    pub beat: Duration,
    pub bpm: f64,
}

#[derive(Debug, Clone, ForeignValue)]
pub enum Mod {
    Percussion,
    Tempo(Tempo),
    Dynamics(f64),
    Instrument(u8), // 0-127
}

#[derive(Debug, Clone, ForeignValue)]
pub enum Composition {
    Note(Note),
    NoteFreq(Note),
    Empty,
    Rest(Duration),
    Trigger(Function),
    Parallel(Vec<Composition>),
    Sequence(Vec<Composition>),
    Pickup(Pickup),
    Mod(Box<Composition>, Mod),
}

struct NoteC;
impl Contract for NoteC {
    fn check(&self, value: &crate::ast::Value) -> bool {
        if let Some(value) = value.numeric() {
            value >= 0.0 && value <= 127.0
        } else {
            false
        }
    }

    fn name(&self) -> &'static str {
        "midi note (0--127)"
    }
}

#[function]
fn dur(numerator: f64, denominator: f64) -> Duration {
    Duration {
        numerator,
        denominator,
    }
}

#[function]
fn numerator(duration: Duration) -> f64 {
    duration.numerator
}

#[function]
fn denominator(duration: Duration) -> f64 {
    duration.denominator
}

#[function]
fn is_pitch_class(string: String) -> bool {
    let chars: Vec<char> = string.chars().collect();

    // must be 1-3 characters
    if chars.is_empty() || chars.len() > 3 {
        return false;
    }

    // first character must be A-G or a-g
    match chars[0] {
        'A'..='G' | 'a'..='g' => (),
        _ => return false,
    }

    // check for accidental(s)
    if chars.len() > 1 {
        let accidental = chars[1];
        if accidental != '#' && accidental != 'b' {
            return false;
        }

        if chars.len() == 3 && chars[2] != '#' && chars[2] != 'b' {
            return false;
        }
    }

    true
}

#[function]
fn is_octave(n: f64) -> bool {
    n >= 0.0 && n <= 10.0 && n.fract() != 0.0
}

#[function]
fn is_note_value(n: f64) -> bool {
    n >= 0.0 && n <= 1.0
}

#[function(contract(0, NoteC))]
fn note(note: f64, duration: Duration) -> Composition {
    Composition::Note(Note {
        value: note,
        duration,
    })
}

#[function]
fn note_freq(freq: f64, duration: Duration) -> Composition {
    Composition::NoteFreq(Note {
        value: freq,
        duration,
    })
}

#[function(contract(0, Natural))]
fn repeat(n: f64, composition: Composition) -> Composition {
    if n == 0.0 {
        return Composition::Empty;
    }
    let mut compositions = Vec::new();
    for _ in 0..n as usize {
        compositions.push(composition.clone());
    }
    Composition::Sequence(compositions)
}

#[function]
fn empty() -> Composition {
    Composition::Empty
}

#[function]
fn rest(duration: Duration) -> Composition {
    Composition::Rest(duration)
}

#[function]
fn trigger(func: Function) -> Composition {
    Composition::Trigger(func)
}

#[function]
fn par(compositions: &[Composition]) -> Composition {
    Composition::Parallel(compositions.to_vec())
}

#[function]
fn seq(compositions: &[Composition]) -> Composition {
    Composition::Sequence(compositions.to_vec())
}

#[function]
fn pickup(pickup: Composition, notes: Composition) -> Composition {
    Composition::Pickup(Pickup {
        pickup: Box::new(pickup),
        notes: Box::new(notes),
    })
}

#[function]
fn mod_q(value: Value) -> bool {
    match value {
        Value::Foreign(fv) => fv.is::<Mod>(),
        _ => false,
    }
}

#[function]
fn tempo(beat: Duration, bpm: f64) -> Mod {
    Mod::Tempo(Tempo { beat, bpm })
}

make_range_checker!(Dynamic, 0.0, 127.0);

#[function(contract(0, Dynamic))]
fn dynamics(amount: f64) -> Mod {
    Mod::Dynamics(amount)
}

make_range_checker!(Instrument, 0.0, 127.0);

#[function(contract(0, Instrument))]
fn instrument(program: f64) -> Mod {
    Mod::Instrument(program as u8)
}

#[function]
fn modify(mod_: Mod, composition: Composition) -> Composition {
    Composition::Mod(Box::new(composition), mod_)
}

#[function]
fn composition_q(value: Value) -> bool {
    match value {
        Value::Foreign(fv) => fv.is::<Composition>(),
        _ => false,
    }
}
