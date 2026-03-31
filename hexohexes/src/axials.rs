use derive_more::{From, Into};
use derive_new::new;

#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct Axials {
    pub q: isize,
    pub r: isize,
}
