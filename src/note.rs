use std::fmt::{Debug, Display};

macro_rules! impl_display_for_note_ext {
    ($t:ty) => {
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
    ($t:ty, $($t2:ty),+) => {
        impl_display_for_note_ext!($t);
        impl_display_for_note_ext!($($t2),+);
    };
}

pub trait NoteExt: Debug + Display {
    fn name(&self) -> String;
    fn id(&self) -> usize;
    fn next(&self) -> Box<dyn NoteExt>;
    fn prev(&self) -> Box<dyn NoteExt>;
}
impl<R: NoteExt> PartialEq<R> for dyn NoteExt {
    fn eq(&self, other: &R) -> bool {
        self.id() == other.id()
    }
}
impl_display_for_note_ext!(Note, Sharp, Flat);

#[derive(Debug, Clone, PartialEq)]
pub enum Note {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteExt for Note {
    fn name(&self) -> String {
        match self {
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::A => "A",
            Self::B => "B",
        }
        .into()
    }
    fn id(&self) -> usize {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        }
    }
    fn next(&self) -> Box<dyn NoteExt> {
        Box::new(Sharp(self.clone()))
    }
    fn prev(&self) -> Box<dyn NoteExt> {
        Box::new(Flat(self.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct Sharp(Note);

impl NoteExt for Sharp {
    fn name(&self) -> String {
        format!("{}♯ ", self.0.name())
    }
    fn id(&self) -> usize {
        (self.0.id() + 1) % 12
    }
    fn next(&self) -> Box<dyn NoteExt> {
        match self {
            Sharp(Note::C) => Box::new(Note::D),
            Sharp(Note::D) => Box::new(Note::E),
            Sharp(Note::E) => Box::new(Sharp(Note::F)),
            Sharp(Note::F) => Box::new(Note::G),
            Sharp(Note::G) => Box::new(Note::A),
            Sharp(Note::A) => Box::new(Note::B),
            Sharp(Note::B) => Box::new(Sharp(Note::C)),
        }
    }
    fn prev(&self) -> Box<dyn NoteExt> {
        Box::new(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Flat(Note);

impl NoteExt for Flat {
    fn name(&self) -> String {
        format!("{}♭ ", self.0.name())
    }
    fn id(&self) -> usize {
        (12 + self.0.id() - 1) % 12
    }
    fn next(&self) -> Box<dyn NoteExt> {
        Box::new(self.0.clone())
    }
    fn prev(&self) -> Box<dyn NoteExt> {
        match self {
            Flat(Note::C) => Box::new(Flat(Note::B)),
            Flat(Note::D) => Box::new(Note::C),
            Flat(Note::E) => Box::new(Note::D),
            Flat(Note::F) => Box::new(Flat(Note::E)),
            Flat(Note::G) => Box::new(Note::F),
            Flat(Note::A) => Box::new(Note::G),
            Flat(Note::B) => Box::new(Note::A),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    #[test_case(Note::C, Note::D)]
    #[test_case(Note::D, Note::E)]
    #[test_case(Note::E, Sharp(Note::F))]
    #[test_case(Note::F, Note::G)]
    #[test_case(Note::G, Note::A)]
    #[test_case(Note::A, Note::B)]
    #[test_case(Note::B, Sharp(Note::C))]
    fn test_sharp_sharp(note: impl NoteExt, expected: impl NoteExt) {
        println!(
            "{:?} -> {:?} -> {:?}",
            note,
            note.next(),
            note.next().next()
        );
        println!("{} -> {} -> {}", note, note.next(), note.next().next());
        assert_eq!(&*note.next().next(), &expected)
    }

    #[test_case(Note::C, Flat(Note::B))]
    #[test_case(Note::D, Note::C)]
    #[test_case(Note::E, Note::D)]
    #[test_case(Note::F, Flat(Note::E))]
    #[test_case(Note::G, Note::F)]
    #[test_case(Note::A, Note::G)]
    #[test_case(Note::B, Note::A)]
    fn test_flat_flat(note: impl NoteExt, expected: impl NoteExt) {
        println!(
            "{:?} -> {:?} -> {:?}",
            note,
            note.prev(),
            note.prev().prev()
        );
        println!("{} -> {} -> {}", note, note.prev(), note.prev().prev());
        assert_eq!(&*note.prev().prev(), &expected)
    }
}
