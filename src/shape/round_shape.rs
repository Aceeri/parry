use crate::math::{Point, Real, Vector};
use crate::shape::SupportMap;
use na::Unit;

#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "cuda", derive(cust_core::DeviceCopy))]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
/// A shape with rounded borders.
pub struct RoundShape<S> {
    /// The shape being rounded.
    pub inner_shape: S,
    /// The radius of the rounded border.
    pub border_radius: Real,
}

impl<S: SupportMap> SupportMap for RoundShape<S> {
    fn local_support_point(&self, dir: &Vector<Real>) -> Point<Real> {
        self.local_support_point_toward(&Unit::new_normalize(*dir))
    }

    fn local_support_point_toward(&self, dir: &Unit<Vector<Real>>) -> Point<Real> {
        self.inner_shape.local_support_point_toward(dir) + **dir * self.border_radius
    }
}
