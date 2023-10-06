use std::fmt::Debug;

pub trait Note: Debug + Default {
    type T: Note;
    type S: Note;
    type F: Note;
    type N: Note;
    type SS: Note;
    fn name(&self) -> String;
    fn id(&self) -> usize;
    fn new() -> Self::T {
        Self::T::default()
    }
    fn s(&self) -> Self::S {
        Self::S::default()
    }
    fn f(&self) -> Self::F {
        Self::F::default()
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
            type T = $t;
            type S = Sharp<$t>;
            type F = Flat<$t>;
            type N = $n;
            type SS = <Sharp<$n> as Note>::T;
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
            type T = Sharp<$t>;
            type S = $s;
            type F = $t;
            type N = $s;
            type SS = Sharp<$s>;
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
    type T = <<Sharp<N> as Note>::S as Note>::T;
    type S = <<Self::T as Note>::S as Note>::T;
    type F = N::T;
    type N = <N::N as Note>::T;
    type SS = <N::SS as Note>::T;
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
    type T = N::T;
    type S = <N::S as Note>::T;
    type F = <N::F as Note>::T;
    type N = <N::N as Note>::T;
    type SS = <N::SS as Note>::T;
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
            type T = Flat<$t>;
            type S = $t;
            type F = $f;
            type N = $t;
            type SS = <$t as Note>::N;
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
    Flat<N::N>: Note,
{
    type T = <<Flat<N> as Note>::F as Note>::T;
    type S = N::T;
    type F = <<Self::T as Note>::F as Note>::T;
    type N = N::N;
    type SS = Flat<N::N>;
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
    type T = N::T;
    type S = <N::S as Note>::T;
    type F = <N::F as Note>::T;
    type N = <N::N as Note>::T;
    type SS = <N::SS as Note>::T;
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

    fn test_types() {
        <M1<C>>::default();
        <m2<C>>::default();
        <M2<C>>::default();
        <m3<C>>::default();
        <M3<C>>::default();
        <M4<C>>::default();
        <d5<C>>::default();
        <M5<C>>::default();
        <A5<C>>::default();
        <M6<C>>::default();
        <d7<C>>::default();
        <m7<C>>::default();
        <M7<C>>::default();
    }
}
