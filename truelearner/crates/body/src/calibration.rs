/// Non-negative distance from a reading to its body's local normal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Residual(u32);

impl Residual {
    pub const ZERO: Self = Self(0);

    pub const fn new(amount: u32) -> Self {
        Self(amount)
    }

    pub const fn amount(self) -> u32 {
        self.0
    }

    pub const fn is_quiet(self) -> bool {
        self.0 == 0
    }

    pub const fn combine(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }
}

/// A body context curried into one reading-to-residual transformation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Normalizer<B, R> {
    body: B,
    relation: R,
}

pub const fn calibrate<B, R>(body: B, relation: R) -> Normalizer<B, R> {
    Normalizer { body, relation }
}

impl<B, R> Normalizer<B, R> {
    pub const fn body(&self) -> &B {
        &self.body
    }

    pub fn step<T>(&mut self, reading: Option<T>) -> Option<Residual>
    where
        R: FnMut(&B, &T) -> Residual,
    {
        let body = &self.body;
        let relation = &mut self.relation;
        reading.map(|value| relation(body, &value))
    }
}
