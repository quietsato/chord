use crate::note::*;
use std::fmt::Debug;

pub trait Interval {}
pub trait IntervalResolve<N> {
    type R: Note;
}

macro_rules! impl_interval {
    ($t:tt, $($b:tt)+) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Default)]
        pub struct $t;
        impl Interval for $t {}
        impl<N: Note> IntervalResolve<N> for $t {
             type R = $($b)+;
        }
    };
}

impl_interval!(P1, N::R);
impl_interval!(m2, <<P1 as IntervalResolve<N>>::R as Note>::ST);
impl_interval!(M2, <<P1 as IntervalResolve<N>>::R as Note>::T);
impl_interval!(m3, <<M2 as IntervalResolve<N>>::R as Note>::ST);
impl_interval!(M3, <<M2 as IntervalResolve<N>>::R as Note>::T);
impl_interval!(P4, <<M3 as IntervalResolve<N>>::R as Note>::ST);
impl_interval!(d5, <<P4 as IntervalResolve<N>>::R as Note>::ST);
impl_interval!(P5, <<P4 as IntervalResolve<N>>::R as Note>::T);
impl_interval!(A5, <<P5 as IntervalResolve<N>>::R as Note>::ST);
impl_interval!(M6, <<P5 as IntervalResolve<N>>::R as Note>::T);
impl_interval!(m7, <<M6 as IntervalResolve<N>>::R as Note>::ST);
impl_interval!(M7, <<M6 as IntervalResolve<N>>::R as Note>::T);

#[allow(dead_code)]
pub type KeyTuple<K> = (
    <K as Key>::I,
    <K as Key>::II,
    <K as Key>::III,
    <K as Key>::IV,
    <K as Key>::V,
    <K as Key>::VI,
    <K as Key>::VII,
);

pub trait Key: Default {
    type I: Note;
    type II: Note;
    type III: Note;
    type IV: Note;
    type V: Note;
    type VI: Note;
    type VII: Note;
    fn i(&self) -> Self::I {
        Default::default()
    }
    fn ii(&self) -> Self::II {
        Default::default()
    }
    fn iii(&self) -> Self::III {
        Default::default()
    }
    fn iv(&self) -> Self::IV {
        Default::default()
    }
    fn v(&self) -> Self::V {
        Default::default()
    }
    fn vi(&self) -> Self::VI {
        Default::default()
    }
    fn vii(&self) -> Self::VII {
        Default::default()
    }
    fn key_tuple(&self) -> KeyTuple<Self> {
        Default::default()
    }
}

#[derive(Debug, Default)]
pub struct MajorKey<Tonic: Note>(Tonic);

#[derive(Debug, Default)]
pub struct MinorKey<Tonic: Note>(Tonic);

macro_rules! impl_key {
    ($t:tt) => {
        impl Key for MajorKey<$t> {
            type I = <$t as Note>::R;
            type II = <Self::I as Note>::T;
            type III = <Self::II as Note>::T;
            type IV = <Self::III as Note>::ST;
            type V = <Self::IV as Note>::T;
            type VI = <Self::V as Note>::T;
            type VII = <Self::VI as Note>::T;
        }
        impl Key for MinorKey<$t> {
            type I = <$t as Note>::R;
            type II = <Self::I as Note>::T;
            type III = <Self::II as Note>::ST;
            type IV = <Self::III as Note>::T;
            type V = <Self::IV as Note>::T;
            type VI = <Self::V as Note>::ST;
            type VII = <Self::VI as Note>::T;
        }
    };
}

impl_key!(C);
impl_key!(D);
impl_key!(E);
impl_key!(F);
impl_key!(G);
impl_key!(A);
impl_key!(B);

macro_rules! impl_key_for_signature {
    (inner $n:tt, $key:tt, $tonic:tt, $sig:tt) => {
        type $n = <$sig<<$key<$tonic> as Key>::$n> as Note>::R;
    };
    ($key:tt, $tonic:tt, $sig:tt) => {
        impl Key for $key<$sig<$tonic>> {
            impl_key_for_signature!(inner I, $key, $tonic, $sig);
            impl_key_for_signature!(inner II, $key, $tonic, $sig);
            impl_key_for_signature!(inner III, $key, $tonic, $sig);
            impl_key_for_signature!(inner IV, $key, $tonic, $sig);
            impl_key_for_signature!(inner V, $key, $tonic, $sig);
            impl_key_for_signature!(inner VI, $key, $tonic, $sig);
            impl_key_for_signature!(inner VII, $key, $tonic, $sig);
        }
    };
}
impl_key_for_signature!(MajorKey, C, Sharp);
impl_key_for_signature!(MajorKey, F, Sharp);
impl_key_for_signature!(MajorKey, C, Flat);
impl_key_for_signature!(MajorKey, D, Flat);
impl_key_for_signature!(MajorKey, E, Flat);
impl_key_for_signature!(MajorKey, G, Flat);
impl_key_for_signature!(MajorKey, A, Flat);
impl_key_for_signature!(MajorKey, B, Flat);

impl_key_for_signature!(MinorKey, A, Sharp);
impl_key_for_signature!(MinorKey, C, Sharp);
impl_key_for_signature!(MinorKey, D, Sharp);
impl_key_for_signature!(MinorKey, F, Sharp);
impl_key_for_signature!(MinorKey, G, Sharp);
impl_key_for_signature!(MinorKey, A, Flat);
impl_key_for_signature!(MinorKey, B, Flat);
impl_key_for_signature!(MinorKey, E, Flat);

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    // Major
    #[test_case(<MajorKey<C>>::default(),        "C   D   E   F   G   A   B  "; "ハ長調")]
    #[test_case(<MajorKey<D>>::default(),        "D   E   F♯  G   A   B   C♯ "; "二長調")]
    #[test_case(<MajorKey<E>>::default(),        "E   F♯  G♯  A   B   C♯  D♯ "; "ホ長調")]
    #[test_case(<MajorKey<F>>::default(),        "F   G   A   B♭  C   D   E  "; "ヘ長調")]
    #[test_case(<MajorKey<G>>::default(),        "G   A   B   C   D   E   F♯ "; "ト長調")]
    #[test_case(<MajorKey<A>>::default(),        "A   B   C♯  D   E   F♯  G♯ "; "イ長調")]
    #[test_case(<MajorKey<B>>::default(),        "B   C♯  D♯  E   F♯  G♯  A♯ "; "ロ長調")]
    #[test_case(<MajorKey<Sharp<C>>>::default(), "C♯  D♯  E♯  F♯  G♯  A♯  B♯ "; "嬰ハ長調")]
    // 嬰二長調
    // 嬰ホ長調
    #[test_case(<MajorKey<Sharp<F>>>::default(), "F♯  G♯  A♯  B   C♯  D♯  E♯ "; "嬰ヘ長調")]
    // 嬰ト長調
    // 嬰イ長調
    // 嬰ロ長調
    #[test_case(<MajorKey<Flat<C>>>::default(),  "C♭  D♭  E♭  F♭  G♭  A♭  B♭ "; "変ハ長調")]
    #[test_case(<MajorKey<Flat<D>>>::default(),  "D♭  E♭  F   G♭  A♭  B♭  C  "; "変二長調")]
    #[test_case(<MajorKey<Flat<E>>>::default(),  "E♭  F   G   A♭  B♭  C   D  "; "変ホ長調")]
    // 変ヘ長調
    #[test_case(<MajorKey<Flat<G>>>::default(),  "G♭  A♭  B♭  C♭  D♭  E♭  F  "; "変ト長調")]
    #[test_case(<MajorKey<Flat<A>>>::default(),  "A♭  B♭  C   D♭  E♭  F   G  "; "変イ長調")]
    #[test_case(<MajorKey<Flat<B>>>::default(),  "B♭  C   D   E♭  F   G   A  "; "変ロ長調")]
    // Minor
    #[test_case(<MinorKey<A>>::default(),        "A   B   C   D   E   F   G  "; "イ短調")]
    #[test_case(<MinorKey<B>>::default(),        "B   C♯  D   E   F♯  G   A  "; "ロ短調")]
    #[test_case(<MinorKey<C>>::default(),        "C   D   E♭  F   G   A♭  B♭ "; "ハ短調")]
    #[test_case(<MinorKey<D>>::default(),        "D   E   F   G   A   B♭  C  "; "二短調")]
    #[test_case(<MinorKey<E>>::default(),        "E   F♯  G   A   B   C   D  "; "ホ短調")]
    #[test_case(<MinorKey<F>>::default(),        "F   G   A♭  B♭  C   D♭  E♭ "; "ヘ短調")]
    #[test_case(<MinorKey<G>>::default(),        "G   A   B♭  C   D   E♭  F  "; "ト短調")]
    #[test_case(<MinorKey<Sharp<A>>>::default(), "A♯  B♯  C♯  D♯  E♯  F♯  G♯ "; "嬰イ短調")]
    // 嬰ロ短調
    #[test_case(<MinorKey<Sharp<C>>>::default(), "C♯  D♯  E   F♯  G♯  A   B  "; "嬰ハ短調")]
    #[test_case(<MinorKey<Sharp<D>>>::default(), "D♯  E♯  F♯  G♯  A♯  B   C♯ "; "嬰二短調")]
    // 嬰ホ短調
    #[test_case(<MinorKey<Sharp<F>>>::default(), "F♯  G♯  A   B   C♯  D   E  "; "嬰ヘ短調")]
    #[test_case(<MinorKey<Sharp<G>>>::default(), "G♯  A♯  B   C♯  D♯  E   F♯ "; "嬰ト短調")]
    #[test_case(<MinorKey<Flat<A>>>::default(),  "A♭  B♭  C♭  D♭  E♭  F♭  G♭ "; "変イ短調")]
    #[test_case(<MinorKey<Flat<B>>>::default(),  "B♭  C   D♭  E♭  F   G♭  A♭ "; "変ロ短調")]
    // 変ハ短調
    // 変二短調
    #[test_case(<MinorKey<Flat<E>>>::default(),  "E♭  F   G♭  A♭  B♭  C♭  D♭ "; "変ホ短調")]
    // 変ヘ短調
    // 変ト短調
    fn test_diatonic_scale(scale: impl Key, expected: &str) {
        assert_eq!(
            [
                scale.i().name(),
                scale.ii().name(),
                scale.iii().name(),
                scale.iv().name(),
                scale.v().name(),
                scale.vi().name(),
                scale.vii().name(),
            ]
            .iter()
            .map(|s| format!("{s:<3}"))
            .collect::<Vec<_>>()
            .join(" "),
            expected,
        );
    }
}
