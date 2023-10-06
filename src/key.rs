use crate::note::*;
use std::fmt::Debug;

pub type M1<N> = <N as Note>::T;
#[allow(non_camel_case_types)]
pub type m2<N> = <M1<N> as Note>::N;
pub type M2<N> = <M1<N> as Note>::SS;
#[allow(non_camel_case_types)]
pub type m3<N> = <M2<N> as Note>::N;
pub type M3<N> = <M2<N> as Note>::SS;
pub type M4<N> = <M3<N> as Note>::N;
#[allow(non_camel_case_types)]
pub type d5<N> = <M4<N> as Note>::N;
pub type M5<N> = <M4<N> as Note>::SS;
pub type A5<N> = <M5<N> as Note>::S;
pub type M6<N> = <M5<N> as Note>::SS;
#[allow(non_camel_case_types)]
pub type d7<N> = <Flat<m7<N>> as Note>::T;
#[allow(non_camel_case_types)]
pub type m7<N> = <M6<N> as Note>::N;
pub type M7<N> = <M6<N> as Note>::SS;

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
}

#[derive(Debug, Default)]
pub struct MajorKey<Tonic: Note>(Tonic);
impl Key for MajorKey<C> {
    type I = C;
    type II = D;
    type III = E;
    type IV = F;
    type V = G;
    type VI = A;
    type VII = B;
}

macro_rules! impl_diatonic_scale {
    (inner $n:tt, $key:tt, $tonic:ty, 1) => {
        type $n = <<$key<$tonic> as Key>::$n as Note>::N;
    };
    (inner $n:tt, $key:tt, $tonic:ty, 2) => {
        type $n = <<$key<$tonic> as Key>::$n as Note>::SS;
    };
    ($key:tt; $tonic:ty, $d:tt, $t:ty) => {
        impl Key for $key<$t> {
            impl_diatonic_scale!(inner I, $key, $tonic, $d);
            impl_diatonic_scale!(inner II, $key, $tonic, $d);
            impl_diatonic_scale!(inner III, $key, $tonic, $d);
            impl_diatonic_scale!(inner IV, $key, $tonic, $d);
            impl_diatonic_scale!(inner V, $key, $tonic, $d);
            impl_diatonic_scale!(inner VI, $key, $tonic, $d);
            impl_diatonic_scale!(inner VII, $key, $tonic, $d);
        }
    };
}
impl_diatonic_scale!(MajorKey; C, 2, D);
impl_diatonic_scale!(MajorKey; D, 2, E);
impl_diatonic_scale!(MajorKey; E, 1, F);
impl_diatonic_scale!(MajorKey; F, 2, G);
impl_diatonic_scale!(MajorKey; G, 2, A);
impl_diatonic_scale!(MajorKey; A, 2, B);

#[derive(Debug, Default)]
pub struct MinorKey<Tonic: Note>(Tonic);
impl Key for MinorKey<A> {
    type I = A;
    type II = B;
    type III = C;
    type IV = D;
    type V = E;
    type VI = F;
    type VII = G;
}
impl_diatonic_scale!(MinorKey; A, 2, B);
impl_diatonic_scale!(MinorKey; B, 1, C);
impl_diatonic_scale!(MinorKey; C, 2, D);
impl_diatonic_scale!(MinorKey; D, 2, E);
impl_diatonic_scale!(MinorKey; E, 1, F);
impl_diatonic_scale!(MinorKey; F, 2, G);

macro_rules! impl_diatonic_scale_for_signature {
    (inner $n:tt, $key:tt, $tonic:tt, $sig:tt) => {
        type $n = <$sig<<$key<$tonic> as Key>::$n> as Note>::T;
    };
    ($key:tt, $tonic:tt, $sig:tt) => {
        impl Key for $key<$sig<$tonic>> {
            impl_diatonic_scale_for_signature!(inner I, $key, $tonic, $sig);
            impl_diatonic_scale_for_signature!(inner II, $key, $tonic, $sig);
            impl_diatonic_scale_for_signature!(inner III, $key, $tonic, $sig);
            impl_diatonic_scale_for_signature!(inner IV, $key, $tonic, $sig);
            impl_diatonic_scale_for_signature!(inner V, $key, $tonic, $sig);
            impl_diatonic_scale_for_signature!(inner VI, $key, $tonic, $sig);
            impl_diatonic_scale_for_signature!(inner VII, $key, $tonic, $sig);
        }
    };
}
impl_diatonic_scale_for_signature!(MajorKey, C, Sharp);
impl_diatonic_scale_for_signature!(MajorKey, F, Sharp);
impl_diatonic_scale_for_signature!(MajorKey, C, Flat);
impl_diatonic_scale_for_signature!(MajorKey, D, Flat);
impl_diatonic_scale_for_signature!(MajorKey, E, Flat);
impl_diatonic_scale_for_signature!(MajorKey, G, Flat);
impl_diatonic_scale_for_signature!(MajorKey, A, Flat);
impl_diatonic_scale_for_signature!(MajorKey, B, Flat);

impl_diatonic_scale_for_signature!(MinorKey, A, Sharp);
impl_diatonic_scale_for_signature!(MinorKey, C, Sharp);
impl_diatonic_scale_for_signature!(MinorKey, D, Sharp);
impl_diatonic_scale_for_signature!(MinorKey, F, Sharp);
impl_diatonic_scale_for_signature!(MinorKey, G, Sharp);
impl_diatonic_scale_for_signature!(MinorKey, A, Flat);
impl_diatonic_scale_for_signature!(MinorKey, B, Flat);
impl_diatonic_scale_for_signature!(MinorKey, E, Flat);

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
