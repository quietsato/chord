use std::fmt::Debug;

pub trait Note: Debug + Default {
    type R: Note;
    /// Flat
    type F: Note;
    /// Sharp
    type S: Note;
    /// Semitone
    type ST: Note;
    /// Tone
    type T: Note;
    fn name(&self) -> String;
    fn id(&self) -> usize;
    fn new() -> Self::R {
        Default::default()
    }
    fn s(&self) -> Self::S {
        Default::default()
    }
    fn f(&self) -> Self::F {
        Default::default()
    }
}

impl Note for () {
    type R = Self;
    type F = Self;
    type S = Self;
    type ST = Self;
    type T = Self;
    fn name(&self) -> String {
        "".into()
    }
    fn id(&self) -> usize {
        std::usize::MAX
    }
}

#[derive(Debug, Default)]
pub struct C;
#[derive(Debug, Default)]
pub struct D;
#[derive(Debug, Default)]
pub struct E;
#[derive(Debug, Default)]
pub struct F;
#[derive(Debug, Default)]
pub struct G;
#[derive(Debug, Default)]
pub struct A;
#[derive(Debug, Default)]
pub struct B;
#[derive(Debug, Default)]
pub struct Sharp<N: Note>(Box<N>);
#[derive(Debug, Default)]
pub struct Flat<N: Note>(Box<N>);

macro_rules! impl_note {
    ($t:ty, $id:expr, $n:ty) => {
        impl Note for $t {
            type R = $t;
            type F = Flat<$t>;
            type S = Sharp<$t>;
            type ST = $n;
            type T = <Sharp<$n> as Note>::R;
            fn name(&self) -> String {
                stringify!($t).into()
            }
            fn id(&self) -> usize {
                $id
            }
        }
    };
}

impl_note!(C, 0, Flat<D>);
impl_note!(D, 2, Flat<E>);
impl_note!(E, 4, F);
impl_note!(F, 5, Flat<G>);
impl_note!(G, 7, Flat<A>);
impl_note!(A, 9, Flat<B>);
impl_note!(B, 11, C);

macro_rules! impl_note_for_sharp {
    (sharp(sharp($t:ty)) = $s:ty) => {
        impl Note for Sharp<$t> {
            type R = Sharp<$t>;
            type F = $t;
            type S = $s;
            type ST = $s;
            type T = Sharp<$s>;
            fn name(&self) -> String {
                format!("{}♯ ", self.0.name())
            }
            fn id(&self) -> usize {
                (self.0.id() + 1) % 12
            }
        }
    };
}

impl_note_for_sharp!(sharp(sharp(C)) = D);
impl_note_for_sharp!(sharp(sharp(D)) = E);
impl_note_for_sharp!(sharp(sharp(E)) = Sharp<F>);
impl_note_for_sharp!(sharp(sharp(F)) = G);
impl_note_for_sharp!(sharp(sharp(G)) = A);
impl_note_for_sharp!(sharp(sharp(A)) = B);
impl_note_for_sharp!(sharp(sharp(B)) = Sharp<C>);

impl<N> Note for Sharp<Sharp<N>>
where
    N: Note,
    Sharp<N>: Note,
{
    type R = <<Sharp<N> as Note>::S as Note>::R;
    type F = N::R;
    type S = <<<Sharp<N> as Note>::S as Note>::S as Note>::R;
    type ST = <<<Sharp<N> as Note>::S as Note>::ST as Note>::R;
    type T = <<<Sharp<N> as Note>::S as Note>::T as Note>::R;
    fn name(&self) -> String {
        self.0.s().name()
    }
    fn id(&self) -> usize {
        self.0.s().id()
    }
}

impl<N> Note for Sharp<Flat<N>>
where
    N: Note,
    Flat<N>: Note,
{
    type R = N::R;
    type F = <N::F as Note>::R;
    type S = <N::S as Note>::R;
    type ST = <N::ST as Note>::R;
    type T = <N::T as Note>::R;
    fn name(&self) -> String {
        self.0.name()
    }
    fn id(&self) -> usize {
        self.0.id()
    }
}

macro_rules! impl_note_for_flat {
    ($t:ty, $f:ty) => {
        impl Note for Flat<$t> {
            type R = Flat<$t>;
            type F = $f;
            type S = $t;
            type ST = <<$t as Note>::ST as Note>::F;
            type T = <<$t as Note>::ST as Note>::R;
            fn name(&self) -> String {
                format!("{}♭ ", self.0.name())
            }
            fn id(&self) -> usize {
                (self.0.id() + 11) % 12
            }
        }
    };
}

impl_note_for_flat!(C, Flat<B>);
impl_note_for_flat!(D, C);
impl_note_for_flat!(E, D);
impl_note_for_flat!(F, Flat<E>);
impl_note_for_flat!(G, F);
impl_note_for_flat!(A, G);
impl_note_for_flat!(B, A);

impl<N> Note for Flat<Flat<N>>
where
    N: Note,
    Flat<N>: Note,
    Flat<N::R>: Note,
{
    type R = <<Flat<N> as Note>::F as Note>::R;
    type F = <<Self::R as Note>::F as Note>::R;
    type S = <N::F as Note>::R;
    type ST = <N::F as Note>::R;
    type T = N::R;
    fn name(&self) -> String {
        self.0.f().name()
    }
    fn id(&self) -> usize {
        self.0.f().id()
    }
}

impl<N> Note for Flat<Sharp<N>>
where
    N: Note,
    Sharp<N>: Note,
{
    type R = N::R;
    type F = <N::F as Note>::R;
    type S = <N::S as Note>::R;
    type ST = <N::ST as Note>::R;
    type T = <N::T as Note>::R;
    fn name(&self) -> String {
        self.0.name()
    }
    fn id(&self) -> usize {
        self.0.id()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::key::*;
    use test_case::test_case;

    #[test_case(C, "C")]
    #[test_case(<Sharp<Sharp<D>>>::new(), "E")]
    #[test_case(<Flat<Flat<G>>>::new(), "F")]
    #[test_case(<Flat<Flat<Flat<G>>>>::new(), "F♭ ")]
    fn test_note_name(note: impl Note, expected: &str) {
        assert_eq!(&note.name(), expected);
    }

    #[test_case(E, Flat::<F>::new())]
    fn test_note_id(n1: impl Note, n2: impl Note) {
        assert_eq!(n1.id(), n2.id());
    }

    #[test]
    fn test_flat_sharp() {
        <<Flat<Flat<C>> as Note>::R>::default();
        <<Flat<C> as Note>::R>::default();
        <<C as Note>::R>::default();
        <<Sharp<C> as Note>::R>::default();
        <<Sharp<Sharp<C>> as Note>::R>::default();
    }

    #[test]
    fn test_interval_resolve() {
        // C
        <<P1 as IntervalResolve<C>>::R>::default();
        <<m2 as IntervalResolve<C>>::R>::default();
        <<M2 as IntervalResolve<C>>::R>::default();
        <<m3 as IntervalResolve<C>>::R>::default();
        <<M3 as IntervalResolve<C>>::R>::default();
        <<P4 as IntervalResolve<C>>::R>::default();
        <<d5 as IntervalResolve<C>>::R>::default();
        <<P5 as IntervalResolve<C>>::R>::default();
        <<A5 as IntervalResolve<C>>::R>::default();
        <<M6 as IntervalResolve<C>>::R>::default();
        <<m7 as IntervalResolve<C>>::R>::default();
        <<M7 as IntervalResolve<C>>::R>::default();

        // C♯
        <<P1 as IntervalResolve<Sharp<C>>>::R>::default();
        <<m2 as IntervalResolve<Sharp<C>>>::R>::default();
        <<M2 as IntervalResolve<Sharp<C>>>::R>::default();
        <<m3 as IntervalResolve<Sharp<C>>>::R>::default();
        <<M3 as IntervalResolve<Sharp<C>>>::R>::default();
        <<P4 as IntervalResolve<Sharp<C>>>::R>::default();
        <<d5 as IntervalResolve<Sharp<C>>>::R>::default();
        <<P5 as IntervalResolve<Sharp<C>>>::R>::default();
        <<A5 as IntervalResolve<Sharp<C>>>::R>::default();
        <<M6 as IntervalResolve<Sharp<C>>>::R>::default();
        <<m7 as IntervalResolve<Sharp<C>>>::R>::default();
        <<M7 as IntervalResolve<Sharp<C>>>::R>::default();

        // F Minor
        <<P1 as IntervalResolve<F>>::R>::default();
        <<m2 as IntervalResolve<F>>::R>::default();
        <<M2 as IntervalResolve<F>>::R>::default();
        <<m3 as IntervalResolve<F>>::R>::default();
        <<M3 as IntervalResolve<F>>::R>::default();
        <<P4 as IntervalResolve<F>>::R>::default();
        <<d5 as IntervalResolve<F>>::R>::default();
        <<P5 as IntervalResolve<F>>::R>::default();
        <<A5 as IntervalResolve<F>>::R>::default();
        <<M6 as IntervalResolve<F>>::R>::default();
        <<m7 as IntervalResolve<F>>::R>::default();
        <<M7 as IntervalResolve<F>>::R>::default();
    }
}
