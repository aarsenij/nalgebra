#[test]
use std::num::{Real, One, abs};
#[test]
use std::rand::{random};
#[test]
use std::cmp::ApproxEq;
#[test]
use traits::inv::Inv;
#[test]
use traits::rotation::Rotation;
// #[test]
// use traits::dim::d7;
#[test]
use dim1::vec1::Vec1;
#[test]
use dim1::mat1::Mat1;
#[test]
use dim2::mat2::Mat2;
#[test]
use dim3::mat3::Mat3;
// #[test]
// use ndim::nmat::NMat;
#[test]
use adaptors::rotmat::Rotmat;

macro_rules! test_inv_mat_impl(
  ($t:ty) => (
    for 10000.times
    {
      let randmat : $t = random();

      assert!((randmat.inverse() * randmat).approx_eq(&One::one()));
    }
  );
)

#[test]
fn test_inv_mat1()
{ test_inv_mat_impl!(Mat1<f64>); }

#[test]
fn test_inv_mat2()
{ test_inv_mat_impl!(Mat2<f64>); }

#[test]
fn test_inv_mat3()
{ test_inv_mat_impl!(Mat3<f64>); }

// FIXME: ICE
// #[test]
// fn test_inv_nmat()
// { test_inv_mat_impl!(NMat<d7, f64>); }

#[test]
fn test_rotation2()
{
  for 10000.times
  {
    let randmat = One::one::<Rotmat<Mat2<f64>>>();
    let ang     = &Vec1::new(abs::<f64>(random()) % Real::pi());

    assert!(randmat.rotated(ang).rotation().approx_eq(ang));
  }
}
