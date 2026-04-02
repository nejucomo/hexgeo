use std::ops::{Deref, Index, IndexMut};

use crate::Axials;
use crate::radial::RadialIndexMap;

/// Store objects mapped to a hexagonal disc (aka "hex of hexes") of [Axials]
///
/// # Logical Layout
///
/// The layout is determined by a radius of tiles (a [usize]), with [Axials::ORIGIN] representing the center of the board.
///
/// # Storage
///
/// The storage overhead is a [Vec] sized to the number of hexes, plus a [RadialIndexMap] for quick index translation to/from [Axials].
///
/// - **TODO: Is the [RadialIndexMap] overhead/complexity really worth it?**
#[derive(Clone, Debug)]
pub struct RadialBoard<T> {
    rim: RadialIndexMap,
    data: Vec<T>,
}

impl<T> RadialBoard<T> {
    /// Create a new board with a given `radius` full of default `T` values in each hex
    pub fn new_defaults(radius: usize) -> Self
    where
        T: Default,
    {
        let bounds = RadialIndexMap::new(radius);

        let mut data = Vec::with_capacity(bounds.count());
        data.resize_with(bounds.count(), T::default);

        RadialBoard { rim: bounds, data }
    }

    /// Access the [RadialIndexMap]
    ///
    /// Note: This is also the [Deref] target, so methods such as [RadialIndexMap::radius] can be called directly on [RadialBoard] values/references.
    pub fn rim(&self) -> &RadialIndexMap {
        &self.rim
    }

    /// Get a reference to the value at the given [Axials]
    pub fn get(&self, ax: Axials) -> Option<&T> {
        self.rim.axial_to_index(ax).map(|ix| &self.data[ix])
    }

    /// Get a mutable reference to the value at the given [Axials]
    pub fn get_mut(&mut self, ax: Axials) -> Option<&mut T> {
        self.rim.axial_to_index(ax).map(|ix| &mut self.data[ix])
    }
}

impl<T> Deref for RadialBoard<T> {
    type Target = RadialIndexMap;

    fn deref(&self) -> &Self::Target {
        &self.rim
    }
}

impl<T> Index<Axials> for RadialBoard<T> {
    type Output = T;

    fn index(&self, ax: Axials) -> &Self::Output {
        &self.data[self.rim.require_axial_to_index::<Self>(ax)]
    }
}

impl<T> IndexMut<Axials> for RadialBoard<T> {
    fn index_mut(&mut self, ax: Axials) -> &mut Self::Output {
        &mut self.data[self.rim.require_axial_to_index::<Self>(ax)]
    }
}
