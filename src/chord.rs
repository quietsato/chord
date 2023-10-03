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

#[allow(non_camel_case_types)]
trait Chord {
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

#[derive(Debug, Default)]
struct Major<R: Note>(R);

#[derive(Debug, Default)]
struct Minor<R: Note>(R);

macro_rules! impl_triad {
    ($t:ty, $sig:tt) => {
        impl Chord for Major<$sig<$t>> {
            type P1 = <$sig<<MajorKey<$t> as Key>::I> as Note>::T;
            type m2 = ();
            type P2 = ();
            type m3 = ();
            type P3 = <$sig<<MajorKey<$t> as Key>::III> as Note>::T;
            type P4 = ();
            type d5 = ();
            type P5 = <$sig<<MajorKey<$t> as Key>::V> as Note>::T;
            type A5 = ();
            type M6d7 = ();
            type m7 = ();
            type M7 = ();
            fn name(&self) -> String {
                <$sig<$t>>::default().name()
            }
        }
        impl Chord for Minor<$sig<$t>> {
            type P1 = <$sig<<MinorKey<$t> as Key>::I> as Note>::T;
            type m2 = ();
            type P2 = ();
            type m3 = <$sig<<MinorKey<$t> as Key>::III> as Note>::T;
            type P3 = ();
            type P4 = ();
            type d5 = ();
            type P5 = <$sig<<MinorKey<$t> as Key>::V> as Note>::T;
            type A5 = ();
            type M6d7 = ();
            type m7 = ();
            type M7 = ();
            fn name(&self) -> String {
                format!("{}m", <$sig<$t>>::default().name())
            }
        }
    };
    ($t:ty) => {
        impl Chord for Major<$t> {
            type P1 = <MajorKey<$t> as Key>::I;
            type m2 = ();
            type P2 = ();
            type m3 = ();
            type P3 = <MajorKey<$t> as Key>::III;
            type P4 = ();
            type d5 = ();
            type P5 = <MajorKey<$t> as Key>::V;
            type A5 = ();
            type M6d7 = ();
            type m7 = ();
            type M7 = ();
            fn name(&self) -> String {
                <$t>::default().name()
            }
        }
        impl Chord for Minor<$t> {
            type P1 = <MinorKey<$t> as Key>::I;
            type m2 = ();
            type P2 = ();
            type m3 = <MinorKey<$t> as Key>::III;
            type P3 = ();
            type P4 = ();
            type d5 = ();
            type P5 = <MinorKey<$t> as Key>::V;
            type A5 = ();
            type M6d7 = ();
            type m7 = ();
            type M7 = ();
            fn name(&self) -> String {
                format!("{}m", <$t>::default().name())
            }
        }
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
struct Aug<R: Note>(Major<R>);

impl<R: Note> Chord for Aug<R>
where
    Major<R>: Chord,
    Sharp<<Major<R> as Chord>::P5>: Note,
{
    type P1 = <Major<R> as Chord>::P1;
    type m2 = ();
    type P2 = ();
    type m3 = ();
    type P3 = <Major<R> as Chord>::P3;
    type P4 = ();
    type d5 = ();
    type P5 = ();
    type A5 = <Sharp<<Major<R> as Chord>::P5> as Note>::T;
    type M6d7 = ();
    type m7 = ();
    type M7 = ();
    fn name(&self) -> String {
        format!("{}aug", self.0.name())
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

        let chord = <Aug<C>>::default();
        <<Aug<C> as Chord>::P1>::default();
        <<Aug<C> as Chord>::P3>::default();
        <<Aug<C> as Chord>::A5>::default();
        println!("{}: {}", chord.name(), chord.notes());
    }
}
