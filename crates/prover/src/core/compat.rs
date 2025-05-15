use std::ops::IndexMut;

/// A trait that provides a version-independent way to get multiple mutable references
pub trait GetManyMut<T> {
    /// Get multiple mutable references to elements at the given indices
    fn get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]>;
}

/// A trait that provides a version-independent way to get multiple disjoint mutable references
pub trait GetDisjointMut<T> {
    /// Get multiple disjoint mutable references to elements at the given indices
    fn get_disjoint_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]>;
}

impl<T> GetManyMut<T> for [T] {
    fn get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]> {
        // Check if any indices are out of bounds
        if indices.iter().any(|&i| i >= self.len()) {
            return None;
        }

        // Check if any indices are duplicates
        for i in 0..N {
            for j in (i + 1)..N {
                if indices[i] == indices[j] {
                    return None;
                }
            }
        }

        // Get the mutable references
        let mut result = Vec::with_capacity(N);
        for &index in &indices {
            result.push(&mut self[index]);
        }

        // Convert Vec to array
        result.try_into().ok()
    }
}

impl<T> GetDisjointMut<T> for [T] {
    fn get_disjoint_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]> {
        self.get_many_mut(indices)
    }
}

impl<T> GetManyMut<T> for Vec<T> {
    fn get_many_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]> {
        self.as_mut_slice().get_many_mut(indices)
    }
}

impl<T> GetDisjointMut<T> for Vec<T> {
    fn get_disjoint_mut<const N: usize>(&mut self, indices: [usize; N]) -> Option<[&mut T; N]> {
        self.as_mut_slice().get_disjoint_mut(indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_many_mut() {
        let mut v = vec![1, 2, 3, 4, 5];
        let [a, b] = v.get_many_mut([0, 2]).unwrap();
        *a = 10;
        *b = 30;
        assert_eq!(v, vec![10, 2, 30, 4, 5]);
    }

    #[test]
    fn test_get_many_mut_out_of_bounds() {
        let mut v = vec![1, 2, 3];
        assert!(v.get_many_mut([0, 3]).is_none());
    }

    #[test]
    fn test_get_many_mut_duplicate_indices() {
        let mut v = vec![1, 2, 3];
        assert!(v.get_many_mut([0, 0]).is_none());
    }
} 