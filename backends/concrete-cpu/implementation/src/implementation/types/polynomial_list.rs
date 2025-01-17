use crate::implementation::Container;

use super::polynomial::Polynomial;

#[derive(Debug, Clone)]
pub struct PolynomialList<C: Container> {
    pub data: C,
    pub count: usize,
    pub polynomial_size: usize,
}

impl<C: Container> PolynomialList<C> {
    pub fn new(data: C, polynomial_size: usize, count: usize) -> Self {
        debug_assert_eq!(data.len(), polynomial_size * count);
        Self {
            data,
            count,
            polynomial_size,
        }
    }

    fn container_len(&self) -> usize {
        self.data.len()
    }

    pub fn into_data(self) -> C {
        self.data
    }
}

impl PolynomialList<&[u64]> {
    pub fn iter_polynomial(&self) -> impl DoubleEndedIterator<Item = Polynomial<&'_ [u64]>> {
        self.data
            .chunks_exact(self.polynomial_size)
            .map(|a| Polynomial::new(a, self.polynomial_size))
    }

    // Creates an iterator over borrowed sub-lists.
    pub fn sublist_iter(
        &self,
        count: usize,
    ) -> impl DoubleEndedIterator<Item = PolynomialList<&[u64]>> {
        let polynomial_size = self.polynomial_size;

        debug_assert_eq!(self.count % count, 0);

        self.data
            .chunks_exact(count * polynomial_size)
            .map(move |sub| PolynomialList {
                data: sub,
                polynomial_size,
                count,
            })
    }
    pub fn as_view(&self) -> PolynomialList<&[u64]> {
        PolynomialList {
            data: self.data,
            count: self.count,
            polynomial_size: self.polynomial_size,
        }
    }
}

impl PolynomialList<&mut [u64]> {
    pub fn iter_polynomial(
        &mut self,
    ) -> impl DoubleEndedIterator<Item = Polynomial<&'_ mut [u64]>> {
        self.data
            .chunks_exact_mut(self.polynomial_size)
            .map(|a| Polynomial::new(a, self.polynomial_size))
    }

    pub fn as_mut_view(&mut self) -> PolynomialList<&mut [u64]> {
        PolynomialList {
            data: self.data,
            count: self.count,
            polynomial_size: self.polynomial_size,
        }
    }

    pub fn as_view(&self) -> PolynomialList<&[u64]> {
        PolynomialList {
            data: self.data,
            count: self.count,
            polynomial_size: self.polynomial_size,
        }
    }
}
