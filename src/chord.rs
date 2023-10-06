use crate::key::*;
use crate::note::*;

#[allow(dead_code)]
type ChordTuple<C> = (
    <C as Chord>::P1,
    <C as Chord>::m2,
    <C as Chord>::M2,
    <C as Chord>::m3,
    <C as Chord>::M3,
    <C as Chord>::P4,
    <C as Chord>::d5,
    <C as Chord>::P5,
    <C as Chord>::A5,
    <C as Chord>::M6,
    <C as Chord>::m7,
    <C as Chord>::M7,
);

#[allow(non_camel_case_types)]
pub trait Chord: Default {
    type R: Note;
    type P1: Note;
    type m2: Note;
    type M2: Note;
    type m3: Note;
    type M3: Note;
    type P4: Note;
    type d5: Note;
    type P5: Note;
    type A5: Note;
    type M6: Note;
    type m7: Note;
    type M7: Note;
    fn name(&self) -> String;
    fn notes_tuple(&self) -> ChordTuple<Self> {
        Default::default()
    }
    fn notes(&self) -> String {
        [
            Self::P1::default().name(),
            Self::m2::default().name(),
            Self::M2::default().name(),
            Self::m3::default().name(),
            Self::M3::default().name(),
            Self::P4::default().name(),
            Self::d5::default().name(),
            Self::P5::default().name(),
            Self::A5::default().name(),
            Self::M6::default().name(),
            Self::m7::default().name(),
            Self::M7::default().name(),
        ]
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| format!("{s:<3}"))
        .collect::<Vec<_>>()
        .join(" ")
    }
}

pub trait TriadChord: Chord {}
impl<R: Note> Chord for Major<R> {
    type R = <P1 as IntervalResolve<R>>::R;
    type P1 = <P1 as IntervalResolve<R>>::R;
    type m2 = ();
    type M2 = ();
    type m3 = ();
    type M3 = <M3 as IntervalResolve<R>>::R;
    type P4 = ();
    type d5 = ();
    type P5 = <P5 as IntervalResolve<R>>::R;
    type A5 = ();
    type M6 = ();
    type m7 = ();
    type M7 = ();
    fn name(&self) -> String {
        R::default().name()
    }
}

#[derive(Debug, Default)]
pub struct Major<R: Note>(R);
impl<R: Note> TriadChord for Major<R> {}

#[derive(Debug, Default)]
pub struct Minor<R: Note>(R);
impl<R: Note> TriadChord for Minor<R> {}

impl<R: Note> Chord for Minor<R> {
    type R = R;
    type P1 = <P1 as IntervalResolve<R>>::R;
    type m2 = ();
    type M2 = ();
    type m3 = <m3 as IntervalResolve<R>>::R;
    type M3 = ();
    type P4 = ();
    type d5 = ();
    type P5 = <P5 as IntervalResolve<R>>::R;
    type A5 = ();
    type M6 = ();
    type m7 = ();
    type M7 = ();
    fn name(&self) -> String {
        format!("{}m", R::default().name())
    }
}

#[derive(Debug, Default)]
struct Aug<R: Note>(R);

impl<R: Note> Chord for Aug<R>
where
    MajorKey<R>: Key,
{
    type R = R;
    type P1 = <P1 as IntervalResolve<R>>::R;
    type m2 = ();
    type M2 = ();
    type m3 = ();
    type M3 = <M3 as IntervalResolve<R>>::R;
    type P4 = ();
    type d5 = ();
    type P5 = ();
    type A5 = <<P5 as IntervalResolve<R>>::R as Note>::S;
    type M6 = ();
    type m7 = ();
    type M7 = ();
    fn name(&self) -> String {
        format!("{}aug", self.0.name())
    }
}

#[derive(Debug, Default)]
struct Dim<R: Note>(R);
impl<R: Note> Chord for Dim<R>
where
    MajorKey<R>: Key,
{
    type R = R;
    type P1 = <P1 as IntervalResolve<R>>::R;
    type m2 = ();
    type M2 = ();
    type m3 = <m3 as IntervalResolve<R>>::R;
    type M3 = ();
    type P4 = ();
    type d5 = <d5 as IntervalResolve<R>>::R;
    type P5 = ();
    type A5 = ();
    type M6 = ();
    type m7 = ();
    type M7 = ();
    fn name(&self) -> String {
        format!("{}dim", self.0.name())
    }
}

#[derive(Debug, Default)]
pub struct Seventh<R: Note>(R);
#[derive(Debug, Default)]
pub struct MajorSeventh<R: Note>(R);
#[derive(Debug, Default)]
pub struct MinorSeventh<R: Note>(R);

impl<R: Note> Chord for Seventh<R>
where
    Major<R>: TriadChord,
{
    type R = R;
    type P1 = <P1 as IntervalResolve<R>>::R;
    type m2 = ();
    type M2 = ();
    type m3 = ();
    type M3 = <M3 as IntervalResolve<R>>::R;
    type P4 = ();
    type d5 = ();
    type P5 = <P5 as IntervalResolve<R>>::R;
    type A5 = ();
    type M6 = ();
    type m7 = <m7 as IntervalResolve<R>>::R;
    type M7 = ();
    fn name(&self) -> String {
        format!("{}7", self.0.name())
    }
}
impl<R: Note> Chord for MajorSeventh<R>
where
    Major<R>: TriadChord,
{
    type R = R;
    type P1 = <P1 as IntervalResolve<R>>::R;
    type m2 = ();
    type M2 = ();
    type m3 = ();
    type M3 = <M3 as IntervalResolve<R>>::R;
    type P4 = ();
    type d5 = ();
    type P5 = <P5 as IntervalResolve<R>>::R;
    type A5 = ();
    type M6 = ();
    type m7 = ();
    type M7 = <M7 as IntervalResolve<R>>::R;
    fn name(&self) -> String {
        format!("{}maj7", self.0.name())
    }
}
impl<R: Note> Chord for MinorSeventh<R> {
    type R = <P1 as IntervalResolve<R>>::R;
    type P1 = <P1 as IntervalResolve<R>>::R;
    type m2 = ();
    type M2 = ();
    type m3 = <m3 as IntervalResolve<R>>::R;
    type M3 = ();
    type P4 = ();
    type d5 = ();
    type P5 = <P5 as IntervalResolve<R>>::R;
    type A5 = ();
    type M6 = ();
    type m7 = <m7 as IntervalResolve<R>>::R;
    type M7 = ();
    fn name(&self) -> String {
        format!("{}m7", self.0.name())
    }
}

#[derive(Debug, Default)]
pub struct Sus2<C: Chord>(C);
#[derive(Debug, Default)]
pub struct Sus4<C: Chord>(C);

impl<C: Chord> Chord for Sus2<C> {
    type R = C::R;
    type P1 = <P1 as IntervalResolve<C::R>>::R;
    type m2 = C::m2;
    type M2 = <M2 as IntervalResolve<C::R>>::R;
    type m3 = ();
    type M3 = ();
    type P4 = C::P4;
    type d5 = C::d5;
    type P5 = C::P5;
    type A5 = C::A5;
    type M6 = C::M6;
    type m7 = C::m7;
    type M7 = C::M7;
    fn name(&self) -> String {
        format!("{}sus2", self.0.name())
    }
}

impl<C: Chord> Chord for Sus4<C> {
    type R = C::R;
    type P1 = <P1 as IntervalResolve<C::R>>::R;
    type m2 = C::m2;
    type M2 = C::M2;
    type m3 = ();
    type M3 = ();
    type P4 = <P4 as IntervalResolve<C::R>>::R;
    type d5 = C::d5;
    type P5 = C::P5;
    type A5 = C::A5;
    type M6 = C::M6;
    type m7 = C::m7;
    type M7 = C::M7;
    fn name(&self) -> String {
        format!("{}sus4", self.0.name())
    }
}

#[derive(Debug, Default)]
pub struct Omit1<C: Chord>(C);
#[derive(Debug, Default)]
pub struct Omit3<C: Chord>(C);
#[derive(Debug, Default)]
pub struct Omit5<C: Chord>(C);

impl<C: Chord> Chord for Omit1<C> {
    type R = C::R;
    type P1 = ();
    type m2 = C::m2;
    type M2 = C::M2;
    type m3 = C::m3;
    type M3 = C::M3;
    type P4 = C::P4;
    type d5 = C::d5;
    type P5 = C::P5;
    type A5 = C::A5;
    type M6 = C::M6;
    type m7 = C::m7;
    type M7 = C::M7;
    fn name(&self) -> String {
        format!("{}omit1", C::default().name())
    }
}
impl<C: Chord> Chord for Omit3<C> {
    type R = C::R;
    type P1 = C::P1;
    type m2 = C::m2;
    type M2 = C::M2;
    type m3 = ();
    type M3 = ();
    type P4 = C::P4;
    type d5 = C::d5;
    type P5 = C::P5;
    type A5 = C::A5;
    type M6 = C::M6;
    type m7 = C::m7;
    type M7 = C::M7;
    fn name(&self) -> String {
        format!("{}omit3", C::default().name())
    }
}
impl<C: Chord> Chord for Omit5<C> {
    type R = C::R;
    type P1 = C::P1;
    type m2 = C::m2;
    type M2 = C::M2;
    type m3 = C::m3;
    type M3 = C::M3;
    type P4 = C::P4;
    type d5 = C::d5;
    type P5 = ();
    type A5 = C::A5;
    type M6 = C::M6;
    type m7 = C::m7;
    type M7 = C::M7;
    fn name(&self) -> String {
        format!("{}omit5", C::default().name())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(<Major<C>>::default().name(), "C");
        assert_eq!(<Major<D>>::default().name(), "D");
        assert_eq!(<Major<Sharp<C>>>::default().name(), "C♯ ");
        assert_eq!(<Major<Flat<C>>>::default().name(), "C♭ ");
        assert_eq!(<Minor<C>>::default().name(), "Cm");
        assert_eq!(<Minor<Sharp<C>>>::default().name(), "C♯ m");
        assert_eq!(<Minor<Flat<C>>>::default().name(), "C♭ m");
        <Major<C>>::default().notes_tuple();
        <Major<B>>::default().notes_tuple();
        <Major<C>>::default().notes_tuple();
        <Major<B>>::default().notes_tuple();
        <Minor<D>>::default().notes_tuple();
        <Minor<E>>::default().notes_tuple();
        <Minor<Flat<D>>>::default().notes_tuple();

        <Aug<C>>::default().notes_tuple();
        assert_eq!(<Aug<C>>::default().name(), "Caug");

        <Dim<B>>::default().notes_tuple();
        <Dim<C>>::default().notes_tuple();
        <Dim<G>>::default().notes_tuple();
        assert_eq!(<Dim<C>>::default().name(), "Cdim");

        <Seventh<C>>::default().notes_tuple();
        <MajorSeventh<C>>::default().notes_tuple();
        <MinorSeventh<C>>::default().notes_tuple();
        assert_eq!(<Seventh<C>>::default().name(), "C7");
        assert_eq!(<MajorSeventh<C>>::default().name(), "Cmaj7");
        assert_eq!(<MinorSeventh<C>>::default().name(), "Cm7");

        <Sus4<Major<C>>>::default().notes_tuple();
        <Sus4<Minor<C>>>::default().notes_tuple();
        <Sus4<Seventh<C>>>::default().notes_tuple();
        <Sus2<Major<C>>>::default().notes_tuple();
        <Sus2<Minor<C>>>::default().notes_tuple();
        <Sus2<Seventh<C>>>::default().notes_tuple();
        assert_eq!(<Sus4<Major<C>>>::default().name(), "Csus4");
        assert_eq!(<Sus4<Minor<C>>>::default().name(), "Cmsus4");
        assert_eq!(<Sus4<Seventh<C>>>::default().name(), "C7sus4");
        assert_eq!(<Sus4<MajorSeventh<C>>>::default().name(), "Cmaj7sus4");
        assert_eq!(<Sus4<MinorSeventh<C>>>::default().name(), "Cm7sus4");
        assert_eq!(<Sus2<Major<C>>>::default().name(), "Csus2");

        <Omit1<Major<C>>>::default().notes_tuple();
        <Omit3<Major<C>>>::default().notes_tuple();
        <Omit5<Major<C>>>::default().notes_tuple();
        <Omit5<Sus4<MajorSeventh<C>>>>::default().notes_tuple();
        assert_eq!(
            <Omit5<Sus4<MajorSeventh<C>>>>::default().name(),
            "Cmaj7sus4omit5"
        );
    }
}
