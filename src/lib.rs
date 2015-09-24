extern crate num;
use num::traits::{Zero, One, Signed, ToPrimitive};
use num::bigint::{BigInt, ToBigInt};
use num::rational::BigRational;

fn factorial(nf: f32) -> BigInt
{
    let n = nf as i64; // hack, let's do the cast here
    let mut acc = One::one();
    for i in 2 .. n + 1
    {
        acc = acc * i.to_bigint().unwrap();
    }
    acc
}

fn minus_one_pow(n: i64) -> i64
{
    1 - n % 2 * 2
}

fn to_bigint(n: f32) -> BigInt
{
    (n as i64).to_bigint().unwrap()
}

// could optimize the type usage better if i knew what i'm doing
pub fn clebschgordansq(j1: f32, m1: f32, j2: f32, m2: f32, j12: f32, m12: f32) -> (i32, BigRational)
{
    if m1 + m2 != m12 || j1 + j2 < j12 || (j1 - j2).abs() > j12 || (2. * (j1 + j2 + j12)) % 2. != 0.
    {
        return (0, Zero::zero())
    }

    let kmin = -(0_f32.min(j12 - j2 + m1).min(j12 - j1 - m2).trunc());
    let kmax = (j1 + j2 - j12).min(j1 - m1).min(j2 + m2).trunc();
    if kmin > kmax
    {
        return (0, Zero::zero())
    }

    let mut c1 = kmin.trunc();
    let mut c2 = (j1 + j2 - j12 - kmin).trunc();
    let mut c3 = (j1 - m1 - kmin).trunc();
    let mut c4 = (j2 + m2 - kmin).trunc();
    let mut c5 = (j12 - j2 + m1 + kmin).trunc();
    let mut c6 = (j12 - j1 - m2 + kmin).trunc();
    let mut c = BigRational::new(
        minus_one_pow(kmin as i64).to_bigint().unwrap(),
        factorial(c1) * factorial(c2) * factorial(c3) * factorial(c4) * factorial(c5) * factorial(c6)
    );
    let mut r = c.clone();

    for _ in kmin as i64 .. kmax as i64  // k unused, removed the +1
    {
        c1 += 1.;
        c5 += 1.;
        c6 += 1.;
        c = c * BigRational::new(
            to_bigint(-c2 * c3 * c4),  // could overflow, don't know the size of values
            to_bigint(c1 * c5 * c6)
        );
        c2 -= 1.;
        c3 -= 1.;
        c4 -= 1.;
        r = r + c.clone();  // ops::AddAssign is still on RFC
    }

    let sign = r.signum().to_integer().to_i32().unwrap(); // this sucks
    let rsq = r.clone() * r; // ugly, pow() not implemented for BigInt

    let val = BigRational::new(
        to_bigint(2. * j12 + 1.) *
        factorial(j12 + j1 - j2) *
        factorial(j12 - j1 + j2) *
        factorial(j1 + j2 - j12) *
        factorial(j12 + m12) * // reordered things here because BigRational * BigInt isn't implemented
        factorial(j12 - m12) *
        factorial(j1 - m1) *
        factorial(j1 + m1) *
        factorial(j2 - m2) *
        factorial(j2 + m2),
        factorial(j1 + j2 + j12 + 1.)
    ) * rsq;

    (sign, val)
}


/*
// can't sqrt() a BigNum, so this doesn't work. maybe should turn it back into float?
pub fn clebschgordan(j1: f32, m1: f32, j2: f32, m2: f32, j12: f32, m12: f32) -> BigRational
{
    let (s, r) = clebschgordansq(j1, m1, j2, m2, j12, m12);
    s * r.sqrt()
}
*/
