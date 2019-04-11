// Copyright 2018 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
//
// This library is free software; you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation; either version
// 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this library;
// if not, see <http://www.gnu.org/licenses/>.

//! Timeval.

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::time::Duration;
use super::{UnixTs, MonotonicTs, MicroSeconds, USEC_INVALID};

/// Bit to set in `timeval`’s `tv_usec` attribute to mark that the `timeval` is in monotonic time
const PA_TIMEVAL_RTCLOCK: i64 = 1 << 30;

/// Wrapper for `libc::timeval`, attaching various methods and trait implementations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Timeval(pub libc::timeval); // Warning, this must remain directly transmutable with the inner libc::timeval

impl PartialEq for Timeval {
    fn eq(&self, other: &Self) -> bool {
        self.0.tv_sec == other.0.tv_sec && self.0.tv_usec == other.0.tv_usec
    }
}
impl Eq for Timeval {}

impl Ord for Timeval {
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { capi::pa_timeval_cmp(&self.0, &other.0) } {
            0 => Ordering::Equal,
            r if r < 0 => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
}

impl PartialOrd for Timeval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Timeval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "timeval {{ tv_sec: {}, tv_usec: {} }}", self.0.tv_sec, self.0.tv_usec)
    }
}

impl Timeval {
    /// Create a new instance, with values provided.
    pub fn new(sec: libc::time_t, usec: libc::suseconds_t) -> Self {
        Timeval(libc::timeval { tv_sec: sec, tv_usec: usec })
    }

    /// Create a new instance, with value of zero.
    pub fn new_zero() -> Self {
        Timeval::new(0, 0)
    }

    /// Calculate the difference between the two specified timeval structs.
    pub fn diff(a: &Self, b: &Self) -> MicroSeconds {
        MicroSeconds(unsafe { capi::pa_timeval_diff(&a.0, &b.0) })
    }

    /// Return the time difference between now and self
    pub fn age(&self) -> MicroSeconds {
        MicroSeconds(unsafe { capi::pa_timeval_age(&self.0) })
    }

    /// Set to the specified (monotonic) value
    ///
    /// The `rtclock` boolean is used for indicating support of the rtclock (monotonic time). If
    /// `true` then the conversion from `MicroSeconds` to `Timeval` is done, and a special ‘rt’ flag
    /// bit is set in `Timeval`’s inner `tv_usec` attribute. If `false`, then instead the timestamp
    /// is converted to a Unix wallclock timestamp.
    ///
    /// Asserts that `v` is not `USEC_INVALID`
    pub(crate) fn set_rt(&mut self, v: MicroSeconds, rtclock: bool) -> &mut Self {
        /* This is a copy of PA’s internal `pa_timeval_rtstore()` function */

        assert_ne!(v, USEC_INVALID);

        *self = v.into();

        match rtclock {
            true => { self.0.tv_usec |= PA_TIMEVAL_RTCLOCK as libc::suseconds_t; },
            false => { self.wallclock_from_rtclock(); },
        }
        self
    }

    pub(crate) fn wallclock_from_rtclock(&mut self) -> &mut Self {
        /* This is a copy of PA’s internal `wallclock_from_rtclock()` function */

        let wc_now = (UnixTs::now()).0;
        let rt_now = Timeval::from((MonotonicTs::now()).0);

        let _ = match rt_now.cmp(self) {
            Ordering::Less => { wc_now.add(Timeval::diff(self, &rt_now)) },
            _              => { wc_now.sub(Timeval::diff(&rt_now, self)) },
        };

        *self = wc_now;
        self
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        let other_us = MicroSeconds::from(other);
        self_us.checked_add(other_us).and_then(|i| Some(i.into()))
    }

    pub fn checked_add_us(self, rhs: MicroSeconds) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        self_us.checked_add(rhs).and_then(|i| Some(i.into()))
    }

    pub fn checked_add_duration(self, rhs: Duration) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        let rhs_us = MicroSeconds::from(rhs);
        self_us.checked_add(rhs_us).and_then(|i| Some(i.into()))
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        let other_us = MicroSeconds::from(other);
        self_us.checked_sub(other_us).and_then(|i| Some(i.into()))
    }

    pub fn checked_sub_us(self, rhs: MicroSeconds) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        self_us.checked_sub(rhs).and_then(|i| Some(i.into()))
    }

    pub fn checked_sub_duration(self, rhs: Duration) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        let rhs_us = MicroSeconds::from(rhs);
        self_us.checked_sub(rhs_us).and_then(|i| Some(i.into()))
    }

    pub fn checked_mul(self, rhs: u32) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        self_us.checked_mul(rhs).and_then(|i| Some(i.into()))
    }

    pub fn checked_div(self, rhs: u32) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        self_us.checked_div(rhs).and_then(|i| Some(i.into()))
    }

    pub fn checked_rem(self, rhs: u32) -> Option<Self> {
        let self_us = MicroSeconds::from(self);
        self_us.checked_rem(rhs).and_then(|i| Some(i.into()))
    }
}

impl Add for Timeval {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.checked_add(other).unwrap()
    }
}
impl AddAssign for Timeval {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.checked_add(rhs).unwrap();
    }
}

impl Add<MicroSeconds> for Timeval {
    type Output = Self;

    fn add(self, rhs: MicroSeconds) -> Self {
        self.checked_add_us(rhs).unwrap()
    }
}
impl AddAssign<MicroSeconds> for Timeval {
    fn add_assign(&mut self, rhs: MicroSeconds) {
        *self = self.checked_add_us(rhs).unwrap();
    }
}

impl Add<Duration> for Timeval {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self {
        self.checked_add_duration(rhs).unwrap()
    }
}
impl AddAssign<Duration> for Timeval {
    fn add_assign(&mut self, rhs: Duration) {
        *self = self.checked_add_duration(rhs).unwrap();
    }
}

impl Sub for Timeval {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.checked_sub(other).unwrap()
    }
}
impl SubAssign for Timeval {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.checked_sub(rhs).unwrap();
    }
}

impl Sub<MicroSeconds> for Timeval {
    type Output = Self;

    fn sub(self, rhs: MicroSeconds) -> Self {
        self.checked_sub_us(rhs).unwrap()
    }
}
impl SubAssign<MicroSeconds> for Timeval {
    fn sub_assign(&mut self, rhs: MicroSeconds) {
        *self = self.checked_sub_us(rhs).unwrap();
    }
}

impl Sub<Duration> for Timeval {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self {
        self.checked_sub_duration(rhs).unwrap()
    }
}
impl SubAssign<Duration> for Timeval {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = self.checked_sub_duration(rhs).unwrap();
    }
}

impl Mul<u32> for Timeval {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        self.checked_mul(rhs).unwrap()
    }
}
impl MulAssign<u32> for Timeval {
    fn mul_assign(&mut self, rhs: u32) {
        *self = self.checked_mul(rhs).unwrap();
    }
}

impl Div<u32> for Timeval {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        self.checked_div(rhs).unwrap()
    }
}
impl DivAssign<u32> for Timeval {
    fn div_assign(&mut self, rhs: u32) {
        *self = self.checked_div(rhs).unwrap();
    }
}

impl Rem<u32> for Timeval {
    type Output = Self;

    fn rem(self, rhs: u32) -> Self {
        self.checked_rem(rhs).unwrap()
    }
}
impl RemAssign<u32> for Timeval {
    fn rem_assign(&mut self, rhs: u32) {
        *self = self.checked_rem(rhs).unwrap();
    }
}
