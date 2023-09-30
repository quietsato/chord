use crate::note::Note;

trait DiatonicScale {}

enum ScalePattern {
    Major,
    Minor,
}

struct Scale<P, N> {}

impl Scale<ScalePattern, Note> {}

#[cfg(test)]
mod test {}
