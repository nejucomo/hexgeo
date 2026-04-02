use derive_more::{From, Into};

#[derive(Copy, Clone, Debug, From, Into)]
pub struct Axials {
    pub q: isize,
    pub r: isize,
}

impl Axials {
    pub const CENTER: Axials = Axials::new(0, 0);

    pub const fn new(q: isize, r: isize) -> Self {
        Axials { q, r }
    }
}
