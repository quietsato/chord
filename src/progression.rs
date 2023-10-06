use crate::chord::*;
use crate::key::*;

pub trait Progression {
    type Progression: Default;
    fn prog(&self) -> Self::Progression {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct OneOfUsProgression<K: Key>(K);

impl<K> Progression for OneOfUsProgression<K>
where
    K: Key,
{
    type Progression = (Major<K::I>, Major<K::V>, Minor<K::VI>, Major<K::IV>);
}

#[derive(Debug, Default)]
pub struct CanonProgression<K: Key>(K);

impl<K> Progression for CanonProgression<K>
where
    K: Key,
{
    type Progression = (
        Major<K::I>,
        Major<K::V>,
        Minor<K::VI>,
        Minor<K::III>,
        Major<K::IV>,
        Major<K::I>,
        Major<K::IV>,
        Major<K::V>,
    );
}

#[derive(Debug, Default)]
pub struct RoyalRoadProgression<K: Key>(K);

impl<K> Progression for RoyalRoadProgression<K>
where
    K: Key,
    Major<K::IV>: TriadChord,
    Major<K::V>: TriadChord,
    Minor<K::VI>: Chord,
    Minor<K::III>: TriadChord,
{
    type Progression = (
        MajorSeventh<K::IV>,
        Seventh<K::V>,
        MinorSeventh<K::III>,
        Minor<K::VI>,
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::note::*;

    #[test]
    fn test_one_of_us_progression() {
        let prog = <OneOfUsProgression<MajorKey<C>>>::default();
        dbg!(prog.prog());
        let prog = <OneOfUsProgression<MinorKey<C>>>::default();
        dbg!(prog.prog());
    }

    #[test]
    fn test_canon_progression() {
        let prog = <CanonProgression<MajorKey<C>>>::default();
        dbg!(prog.prog());
        let prog = <CanonProgression<MinorKey<C>>>::default();
        dbg!(prog.prog());
    }

    #[test]
    fn test_royal_road_progression() {
        let prog = <RoyalRoadProgression<MajorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
        );
        let prog = <RoyalRoadProgression<MinorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
        );
    }
}
