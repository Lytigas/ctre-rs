//! Rust bindings for the CTRE Phoenix CCI libraries.

mod bindings;

#[doc(inline)]
pub use bindings::root::*;

// root stuff
use ctre::phoenix;
use std::fmt;
impl phoenix::ErrorCode {
    /// Returns `true` if the error code is `OK`.
    #[inline]
    pub fn is_ok(self) -> bool {
        self == phoenix::ErrorCode::OK
    }

    /// Returns `true` if the error code is not `OK`.
    #[inline]
    pub fn is_err(self) -> bool {
        self != phoenix::ErrorCode::OK
    }

    /// Returns `err` if `self` is `OK`, otherwise returns `self`.
    /// Intended for use by the `ctre` crate only.
    pub fn or(self, err: Self) -> Self {
        match self {
            phoenix::ErrorCode::OK => err,
            _ => self,
        }
    }

    /// Returns an `Ok` if the error code is `OK`, or an `Err` otherwise.
    pub fn into_res(self) -> Result<(), Self> {
        match self {
            phoenix::ErrorCode::OK => Ok(()),
            _ => Err(self),
        }
    }
}
impl Into<std::result::Result<(), phoenix::ErrorCode>> for phoenix::ErrorCode {
    fn into(self) -> std::result::Result<(), phoenix::ErrorCode> {
        self.into_res()
    }
}

impl std::error::Error for phoenix::ErrorCode {
    fn description(&self) -> &str {
        "Error in CTRE Phoenix"
    }
}

impl fmt::Display for phoenix::ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // yeah uhm CTRE pls fix your Logger CCI
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "try_trait")]
impl std::ops::Try for phoenix::ErrorCode {
    type Ok = ();
    type Error = Self;

    fn into_result(self) -> Result<(), Self> {
        match self {
            ErrorCode::OK => Ok(()),
            _ => Err(self),
        }
    }

    fn from_error(v: Self) -> Self {
        v
    }
    fn from_ok(v: ()) -> Self {
        ErrorCode::OK
    }
}

// canifier stuff

#[doc(hidden)]
pub enum _CANifierHandle {}
/// A handle representing a CANifier.
pub type CANifierHandle = *mut _CANifierHandle;

// motor stuff
#[doc(hidden)]
pub enum _MotHandle {}
/// A handle representing a motor controller.
pub type MotHandle = *mut _MotHandle;

use phoenix::motorcontrol;
impl Default for motorcontrol::DemandType {
    #[inline]
    fn default() -> motorcontrol::DemandType {
        motorcontrol::DemandType::Neutral
    }
}
impl Default for motorcontrol::FollowerType {
    #[inline]
    fn default() -> motorcontrol::FollowerType {
        motorcontrol::FollowerType::PercentOutput
    }
}
impl Default for motorcontrol::FeedbackDevice {
    #[inline]
    fn default() -> motorcontrol::FeedbackDevice {
        motorcontrol::FeedbackDevice::QuadEncoder
    }
}
impl Default for motorcontrol::LimitSwitchSource {
    #[inline]
    fn default() -> motorcontrol::LimitSwitchSource {
        motorcontrol::LimitSwitchSource::FeedbackConnector
    }
}
impl Default for motorcontrol::LimitSwitchNormal {
    #[inline]
    fn default() -> motorcontrol::LimitSwitchNormal {
        motorcontrol::LimitSwitchNormal::NormallyOpen
    }
}
impl Default for motorcontrol::RemoteSensorSource {
    #[inline]
    fn default() -> motorcontrol::RemoteSensorSource {
        motorcontrol::RemoteSensorSource::Off
    }
}
impl Default for motorcontrol::VelocityMeasPeriod {
    #[inline]
    fn default() -> motorcontrol::VelocityMeasPeriod {
        motorcontrol::VelocityMeasPeriod::Period_100Ms
    }
}

use phoenix::motion;
use std::os::raw;
impl From<raw::c_int> for motion::SetValueMotionProfile {
    fn from(value: raw::c_int) -> motion::SetValueMotionProfile {
        match value {
            0 => motion::SetValueMotionProfile::Disable,
            1 => motion::SetValueMotionProfile::Enable,
            2 => motion::SetValueMotionProfile::Hold,
            _ => panic!("Invalid raw c_int to SetValueMotionProfile"),
        }
    }
}
// impl Default for motion::SetValueMotionProfile {
//     #[inline]
//     fn default() -> motion::SetValueMotionProfile {
//         motion::SetValueMotionProfile::Invalid
//     }
// }

// pigeon stuff
#[doc(hidden)]
pub enum _PigeonHandle {}
/// A handle representing a Pigeon IMU.
pub type PigeonHandle = *mut _PigeonHandle;
