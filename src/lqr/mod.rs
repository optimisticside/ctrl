use core::cmp::min;

use nalgebra::base::{OMatrix, Scaler};
use nalgebra::core::allocator::Allocator;
use nalgebra::core::dimension::Dim;

/// Calculate the threshold for calculating a matrix's rank. Follows the same algorithm as Eigen.
pub fn calculate_threshold<T: Scalar + SimdPartialOrd>(matrix: &OMatrix<T, R, C>) -> T {
    T.epsilon() * min(matrix.nrows(), matrix.ncols())
}

/// Compute the kernel of a matrix F, and return P such that the rows of P are the basis for the
/// null-space of F, namely PPᵀ = I and PFᵀ = 0.
pub fn compute_kernel<T, R: Dim, C: Dim>(matrix: &OMatrix<T, R, C>) -> OMatrix<T, R, C>
where
    T: Scalar + SimdPartialOrd,
    DefaultAllocator: Allocator<T, R, C>,
{
    let transposed_qr = matrix.transpose().qr();
    let transposed_q = fqr.q();

    let threshold = calculate_threshold(f);
    transposed_q.columns_range(transposed_q.ncols()..(matrix.cols() - transpoed_qr.rank(threshold))).transpose()
}

/// Main linear quadratic regulator calculation routine. Given matrices A, B, Q, R, return matrices
/// K and S where K is the coefficient
pub fn calculate<T, R: Dim, C: Dim>(
    a: &OMatrix<T, R, C>,
    b: &OMatrix<T, R, C>,
    q: &OMatrix<T, R, C>,
    r: &OMatrix<T, R, C>
) -> (OMatrix<T, R, C>, OMatrix<T, R, C>)
where
    T: Scalar + SimdPartialOrd
    DefaultAllocator: Allocator<T, R, C>
{

}
