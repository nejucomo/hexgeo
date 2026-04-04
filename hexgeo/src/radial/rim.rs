use crate::Axials;

/// Precomputed coordinate bounds for quick conversion between axial coords and usize storage indices for a disc / hex of hexes of a given radius
///
/// # Performance Tradeoffs
///
/// This design is optimized for a relatively small radius and stores `O(radius^2)` [usize]s to reduce conversion cost. Note/TODO: there has been no profiling of this pre-computation approach versus a purely arithmetic conversion implementation for varying radii.
#[derive(Debug, Clone)]
pub struct RadialIndexMap {
    radius: usize,

    // row for each r in -R..=R
    row_starts: Vec<usize>,

    // row for each packed index
    index_rows: Vec<usize>,
}

impl RadialIndexMap {
    /// Create a new index map for the given `radius`
    pub fn new(radius: usize) -> Self {
        let radius_i = radius as isize;
        let rows = 2 * radius + 1;
        let len = 1 + 3 * radius * (radius + 1);

        let mut row_starts = Vec::with_capacity(rows);
        let mut index_rows = Vec::with_capacity(len);

        let mut start = 0usize;
        for row in 0..rows {
            let r = row as isize - radius_i;
            let l = row_len(radius_i, r);

            row_starts.push(start);
            index_rows.extend(std::iter::repeat_n(row, l));

            start += l;
        }

        debug_assert_eq!(start, len);
        debug_assert_eq!(index_rows.len(), len);

        Self {
            radius,
            row_starts,
            index_rows,
        }
    }

    /// The radius
    #[inline]
    pub fn radius(&self) -> usize {
        self.radius
    }

    fn irad(&self) -> isize {
        self.radius as isize
    }

    /// The number of hexes within a disc with our radius
    #[inline]
    pub fn count(&self) -> usize {
        self.index_rows.len()
    }

    /// Do the given `axials` refer to a hex on this disc?
    #[inline]
    pub fn contains(&self, axials: Axials) -> bool {
        let Axials { q, r } = axials;
        let rad = self.irad();
        let s = -q - r;
        q.abs() <= rad && r.abs() <= rad && s.abs() <= rad
    }

    /// Iterate over all [Axials] contained on this disc
    pub fn iter_axials(&self) -> impl Iterator<Item = Axials> + '_ {
        (0..self.count()).map(|ix| self.index_to_axial(ix).unwrap())
    }

    #[inline]
    pub(crate) fn axial_to_index(&self, axials: Axials) -> Option<usize> {
        if self.contains(axials) {
            let Axials { q, r } = axials;
            let row = self.row_of_r(r);
            Some(self.row_starts[row] + (q - q_min(self.irad(), r)) as usize)
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn require_axial_to_index<T>(&self, axials: Axials) -> usize {
        if let Some(ix) = self.axial_to_index(axials) {
            ix
        } else {
            let tn = std::any::type_name::<T>();
            let r = self.radius();
            panic!("indexing {tn}, radius {r} with out-of-bounds {axials:?}");
        }
    }

    #[inline]
    pub(crate) fn index_to_axial(&self, index: usize) -> Option<Axials> {
        let &row = self.index_rows.get(index)?;
        let r = self.r_of_row(row);
        let q = q_min(self.irad(), r) + (index - self.row_starts[row]) as isize;
        Some(Axials::new(q, r))
    }

    fn row_of_r(&self, r: isize) -> usize {
        (r + self.irad()) as usize
    }

    fn r_of_row(&self, row: usize) -> isize {
        row as isize - self.irad()
    }
}

fn q_min(radius: isize, r: isize) -> isize {
    if r <= 0 { -radius - r } else { -radius }
}

fn row_len(radius: isize, r: isize) -> usize {
    (2 * radius + 1 - r.abs()) as usize
}
