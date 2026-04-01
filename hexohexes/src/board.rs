use std::ops::{Deref, Index, IndexMut};

use crate::{AxialBounds, Axials};

#[derive(Clone, Debug)]
pub struct Board<T> {
    bounds: AxialBounds,
    data: Vec<T>,
}

impl<T> Board<T> {
    pub fn new_defaults(radius: usize) -> Self
    where
        T: Default,
    {
        let bounds = AxialBounds::new(radius);

        let mut data = Vec::with_capacity(bounds.count());
        data.resize_with(bounds.count(), T::default);

        Board { bounds, data }
    }

    pub fn bounds(&self) -> &AxialBounds {
        &self.bounds
    }

    pub fn get(&self, ax: Axials) -> Option<&T> {
        self.bounds.axial_to_index(ax).map(|ix| &self.data[ix])
    }

    pub fn get_mut(&mut self, ax: Axials) -> Option<&mut T> {
        self.bounds.axial_to_index(ax).map(|ix| &mut self.data[ix])
    }
}

impl<T> Deref for Board<T> {
    type Target = AxialBounds;

    fn deref(&self) -> &Self::Target {
        &self.bounds
    }
}

impl<T> Index<Axials> for Board<T> {
    type Output = T;

    fn index(&self, ax: Axials) -> &Self::Output {
        &self.data[self.bounds.require_axial_to_index::<Self>(ax)]
    }
}

impl<T> IndexMut<Axials> for Board<T> {
    fn index_mut(&mut self, ax: Axials) -> &mut Self::Output {
        &mut self.data[self.bounds.require_axial_to_index::<Self>(ax)]
    }
}
