use std::usize;

use crate::key::*;
use crate::note::*;

impl Note for () {
    type T = Self;
    type S = Self;
    type F = Self;
    type N = Self;
    type SS = Self;
    fn name(&self) -> String {
        "".into()
    }
    fn id(&self) -> usize {
        usize::MAX
    }
}

#[allow(dead_code)]
type ChordTuple<C> = (
    <C as Chord>::P1,
    <C as Chord>::m2,
    <C as Chord>::P2,
    <C as Chord>::m3,
    <C as Chord>::P3,
    <C as Chord>::P4,
    <C as Chord>::d5,
    <C as Chord>::P5,
    <C as Chord>::A5,
    <C as Chord>::M6d7,
    <C as Chord>::m7,
    <C as Chord>::M7,
);

#[allow(non_camel_case_types)]
pub trait Chord: Default {
    type R: Note;
    type P1: Note;
    type m2: Note;
    type P2: Note;
    type m3: Note;
    type P3: Note;
    type P4: Note;
    type d5: Note;
    type P5: Note;
    type A5: Note;
    type M6d7: Note;
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
            Self::P2::default().name(),
            Self::m3::default().name(),
            Self::P3::default().name(),
            Self::P4::default().name(),
            Self::d5::default().name(),
            Self::P5::default().name(),
            Self::A5::default().name(),
            Self::M6d7::default().name(),
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

#[derive(Debug, Default)]
pub struct Major<R: Note>(R);

#[derive(Debug, Default)]
pub struct Minor<R: Note>(R);

macro_rules! impl_triad {
    ($t:ty, $sig:tt) => {
        impl Chord for Major<$sig<$t>> {
            type R = <$sig<$t> as Note>::T;
            type P1 = M1<$sig<$t>>;
            type m2 = ();
            type P2 = ();
            type m3 = ();
            type P3 = M3<$sig<$t>>;
            type P4 = ();
            type d5 = ();
            type P5 = M5<$sig<$t>>;
            type A5 = ();
            type M6d7 = ();
            type m7 = ();
            type M7 = ();
            fn name(&self) -> String {
                <$sig<$t>>::default().name()
            }
        }
        impl TriadChord for Major<$sig<$t>> {}
        impl Chord for Minor<$sig<$t>> {
            type R = <$sig<$t> as Note>::T;
            type P1 = M1<$sig<$t>>;
            type m2 = ();
            type P2 = ();
            type m3 = m3<$sig<$t>>;
            type P3 = ();
            type P4 = ();
            type d5 = ();
            type P5 = M5<$sig<$t>>;
            type A5 = ();
            type M6d7 = ();
            type m7 = ();
            type M7 = ();
            fn name(&self) -> String {
                format!("{}m", <$sig<$t>>::default().name())
            }
        }
        impl TriadChord for Minor<$sig<$t>> {}
    };
    ($t:ty) => {
        impl Chord for Major<$t> {
            type R = $t;
            type P1 = M1<$t>;
            type m2 = ();
            type P2 = ();
            type m3 = ();
            type P3 = M3<$t>;
            type P4 = ();
            type d5 = ();
            type P5 = M5<$t>;
            type A5 = ();
            type M6d7 = ();
            type m7 = ();
            type M7 = ();
            fn name(&self) -> String {
                <$t>::default().name()
            }
        }
        impl TriadChord for Major<$t> {}
        impl Chord for Minor<$t> {
            type R = $t;
            type P1 = M1<$t>;
            type m2 = ();
            type P2 = ();
            type m3 = m3<$t>;
            type P3 = ();
            type P4 = ();
            type d5 = ();
            type P5 = M5<$t>;
            type A5 = ();
            type M6d7 = ();
            type m7 = ();
            type M7 = ();
            fn name(&self) -> String {
                format!("{}m", <$t>::default().name())
            }
        }
        impl TriadChord for Minor<$t> {}
        impl_triad!($t, Sharp);
        impl_triad!($t, Flat);
    };
}

impl_triad!(C);
impl_triad!(D);
impl_triad!(E);
impl_triad!(F);
impl_triad!(G);
impl_triad!(A);
impl_triad!(B);

#[derive(Debug, Default)]
struct Aug<R: Note>(R);

impl<R: Note> Chord for Aug<R>
where
    MajorKey<R>: Key,
{
    type R = R;
    type P1 = M1<R>;
    type m2 = ();
    type P2 = ();
    type m3 = ();
    type P3 = M3<R>;
    type P4 = ();
    type d5 = ();
    type P5 = ();
    type A5 = A5<R>;
    type M6d7 = ();
    type m7 = ();
    type M7 = ();
    fn name(&self) -> String {
        format!("{}aug", self.0.name())
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
    type P1 = M1<R>;
    type m2 = ();
    type P2 = ();
    type m3 = ();
    type P3 = M3<R>;
    type P4 = ();
    type d5 = ();
    type P5 = M5<R>;
    type A5 = ();
    type M6d7 = ();
    type m7 = m7<R>;
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
    type P1 = M1<R>;
    type m2 = ();
    type P2 = ();
    type m3 = ();
    type P3 = M3<R>;
    type P4 = ();
    type d5 = ();
    type P5 = M5<R>;
    type A5 = ();
    type M6d7 = ();
    type m7 = ();
    type M7 = M7<R>;
    fn name(&self) -> String {
        format!("{}maj7", self.0.name())
    }
}
impl<R: Note> Chord for MinorSeventh<R> {
    type R = R;
    type P1 = M1<R>;
    type m2 = ();
    type P2 = ();
    type m3 = m3<R>;
    type P3 = ();
    type P4 = ();
    type d5 = ();
    type P5 = M5<R>;
    type A5 = ();
    type M6d7 = ();
    type m7 = m7<R>;
    type M7 = ();
    fn name(&self) -> String {
        format!("{}m7", self.0.name())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let chord = <Minor<Flat<D>>>::default();
        println!("{}: {}", chord.name(), chord.notes());

        let chord = <Minor<D>>::default();
        println!("{}: {}", chord.name(), chord.notes());

        let chord = <Major<B>>::default();
        println!("{}: {}", chord.name(), chord.notes());

        let chord = <Major<Sharp<C>>>::default();
        println!("{}: {}", chord.name(), chord.notes());
        <Major<C>>::default().notes_tuple();

        let chord = <Aug<C>>::default();
        <Aug<C>>::default().notes_tuple();
        println!("{}: {}", chord.name(), chord.notes());

        <Seventh<C>>::default().notes_tuple();
        <MajorSeventh<C>>::default().notes_tuple();
        <MinorSeventh<C>>::default().notes_tuple();
    }
}
