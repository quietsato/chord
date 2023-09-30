use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Note {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
    Sharp(Box<Note>),
    Flat(Box<Note>),
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Note {
    pub fn name(&self) -> String {
        match self {
            Self::C => "C".into(),
            Self::D => "D".into(),
            Self::E => "E".into(),
            Self::F => "F".into(),
            Self::G => "G".into(),
            Self::A => "A".into(),
            Self::B => "B".into(),
            Self::Sharp(n) => format!("{}♯ ", n.name()),
            Self::Flat(n) => format!("{}♭ ", n.name()),
        }
    }
    pub fn id(&self) -> usize {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
            Self::Sharp(n) => (n.id() + 1) % 12,
            Self::Flat(n) => (n.id() + 11) % 12,
        }
    }
    pub fn s(&self) -> Self {
        match self {
            Self::Sharp(n) => match &**n {
                Self::C => Self::D,
                Self::D => Self::E,
                Self::E => Self::Sharp(Self::F.into()),
                Self::F => Self::G,
                Self::G => Self::A,
                Self::A => Self::B,
                Self::B => Self::Sharp(Self::C.into()),
                n => n.s().s(),
            },
            Self::Flat(n) => *n.clone(),
            n => Self::Sharp(Box::new(n.clone())),
        }
    }
    pub fn f(&self) -> Note {
        match self {
            Self::Flat(n) => match &**n {
                Self::C => Self::Flat(Self::B.into()),
                Self::D => Self::C,
                Self::E => Self::D,
                Self::F => Self::Flat(Self::E.into()),
                Self::G => Self::F,
                Self::A => Self::G,
                Self::B => Self::A,
                nn => nn.f().f(),
            },
            Self::Sharp(n) => *n.clone(),
            n => Self::Flat(Box::new(n.clone())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    #[test_case(Note::C, Note::D)]
    #[test_case(Note::D, Note::E)]
    #[test_case(Note::E, Note::Sharp(Box::new(Note::F)))]
    #[test_case(Note::F, Note::G)]
    #[test_case(Note::G, Note::A)]
    #[test_case(Note::A, Note::B)]
    #[test_case(Note::B, Note::Sharp(Box::new(Note::C)))]
    fn test_sharp_sharp(note: Note, expected: Note) {
        println!("{:?} -> {:?} -> {:?}", note, note.s(), note.s().s());
        println!("{} -> {} -> {}", note, note.s(), note.s().s());
        assert_eq!(note.s().s(), expected)
    }

    #[test_case(Note::C, Note::Flat(Box::new(Note::B)))]
    #[test_case(Note::D, Note::C)]
    #[test_case(Note::E, Note::D)]
    #[test_case(Note::F, Note::Flat(Box::new(Note::E)))]
    #[test_case(Note::G, Note::F)]
    #[test_case(Note::A, Note::G)]
    #[test_case(Note::B, Note::A)]
    fn test_flat_flat(note: Note, expected: Note) {
        println!("{:?} -> {:?} -> {:?}", note, note.f(), note.f().f());
        println!("{} -> {} -> {}", note, note.f(), note.f().f());
        assert_eq!(note.f().f(), expected)
    }

    #[test]
    fn resolve_sharps_and_flats() {
        assert_eq!(
            Note::Sharp(Note::Flat(Note::Sharp(Note::Sharp(Note::C.into()).into()).into()).into())
                .s()
                .name(),
            "D♯ "
        )
    }
}
