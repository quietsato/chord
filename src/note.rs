use std::fmt::Debug;

pub trait Note: Debug + Default {
    type T: Note;
    type S: Note;
    type F: Note;
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
    ($t:ty, $id:expr) => {
        impl Note for $t {
            type T = $t;
            type S = Sharp<$t>;
            type F = Flat<$t>;
            fn name(&self) -> String {
                stringify!($t).into()
            }
            fn id(&self) -> usize {
                $id
            }
        }
    };
}

impl_note!(C, 0);
impl_note!(D, 2);
impl_note!(E, 4);
impl_note!(F, 5);
impl_note!(G, 7);
impl_note!(A, 9);
impl_note!(B, 11);

macro_rules! impl_note_for_sharp {
    (sharp(sharp($t:ty)) = $s:ty) => {
        impl Note for Sharp<$t> {
            type T = Sharp<$t>;
            type S = $s;
            type F = $t;
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
impl_note_for_sharp!(sharp(sharp(B))= Sharp<C>);

impl<N> Note for Sharp<Sharp<N>>
where
    N: Note,
    Sharp<N>: Note,
{
    type T = <<Sharp<N> as Note>::S as Note>::T;
    type S = <<Self::T as Note>::S as Note>::T;
    type F = N::T;
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
    fn name(&self) -> String {
        self.0.name()
    }
    fn id(&self) -> usize {
        self.0.id()
    }
}

macro_rules! impl_note_for_flat {
    ($t:ty, $f:ty) => {
        impl Note for $t {
            type T = $t;
            type S = $t;
            type F = $f;
            fn name(&self) -> String {
                format!("{}♭ ", self.0.name())
            }
            fn id(&self) -> usize {
                (self.0.id() + 11) % 12
            }
        }
    };
}

impl_note_for_flat!(Flat<C>, Flat<B>);
impl_note_for_flat!(Flat<D>, C);
impl_note_for_flat!(Flat<E>, D);
impl_note_for_flat!(Flat<F>, Flat<E>);
impl_note_for_flat!(Flat<G>, F);
impl_note_for_flat!(Flat<A>, G);
impl_note_for_flat!(Flat<B>, A);

impl<N> Note for Flat<Flat<N>>
where
    N: Note,
    Flat<N>: Note,
{
    type T = <<Flat<N> as Note>::F as Note>::T;
    type S = N::T;
    type F = <<Self::T as Note>::F as Note>::T;
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
}
