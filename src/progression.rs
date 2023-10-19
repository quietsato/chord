use crate::chord::*;
use crate::key::*;

pub trait Progression {
    type Progression: Default;
    fn prog(&self) -> Self::Progression {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct PopPunkProgression<K: Key>(K);

impl<K> Progression for PopPunkProgression<K>
where
    K: Key,
{
    type Progression = (Major<K::I>, Major<K::V>, Minor<K::VI>, Major<K::IV>);
}

#[derive(Debug, Default)]
pub struct KomuroProgression<K: Key>(K);

impl<K> Progression for KomuroProgression<K>
where
    K: Key,
{
    type Progression = (Minor<K::VI>, Major<K::IV>, Major<K::V>, Major<K::I>);
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
{
    type Progression = (
        MajorSeventh<K::IV>,
        Seventh<K::V>,
        MinorSeventh<K::III>,
        Minor<K::VI>,
    );
}

#[derive(Debug, Default)]
pub struct JustTheTwoOfUsProgression<K: Key>(K);

impl<K> Progression for JustTheTwoOfUsProgression<K>
where
    K: Key,
{
    type Progression = (
        MajorSeventh<K::IV>,
        Seventh<K::III>,
        MinorSeventh<K::VI>,
        MinorSeventh<K::V>,
        Seventh<K::I>,
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::note::*;

    #[test]
    fn test_pop_punk_progression() {
        let prog = <PopPunkProgression<MajorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
        );
        let prog = <PopPunkProgression<MinorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
        );
    }

    #[test]
    fn test_komuro_progression() {
        let prog = <KomuroProgression<MajorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
        );
        let prog = <KomuroProgression<MinorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
        );
    }

    #[test]
    fn test_canon_progression() {
        let prog = <CanonProgression<MajorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
            prog.4.notes_tuple(),
            prog.5.notes_tuple(),
            prog.6.notes_tuple(),
            prog.7.notes_tuple(),
        );
        let prog = <CanonProgression<MinorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
            prog.4.notes_tuple(),
            prog.5.notes_tuple(),
            prog.6.notes_tuple(),
            prog.7.notes_tuple(),
        );
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

    #[test]
    fn test_just_the_two_of_us_progression() {
        let prog = <JustTheTwoOfUsProgression<MajorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
            prog.4.notes_tuple(),
        );
        let prog = <JustTheTwoOfUsProgression<MinorKey<C>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
            prog.4.notes_tuple(),
        );
        let prog = <JustTheTwoOfUsProgression<MajorKey<Flat<A>>>>::default().prog();
        dbg!(
            prog.0.notes_tuple(),
            prog.1.notes_tuple(),
            prog.2.notes_tuple(),
            prog.3.notes_tuple(),
            prog.4.notes_tuple(),
        );
    }
}
