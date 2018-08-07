//! Enums, structs, and functions related to motor controllers.
#![allow(non_upper_case_globals)]

use std::os::raw::{c_char, c_int};
use ErrorCode;

pub enum _Handle {}
pub type Handle = *mut _Handle;

/* automatically generated by rust-bindgen, ish */

#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ControlFrame {
    C3_General = 262272,
    C4_Advanced = 262336,
    C6_MotProfAddTrajPoint = 262464,
}
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ControlFrameEnhanced {
    C3_General = 262272,
    C4_Advanced = 262336,
    C5_FeedbackOutputOverride = 262400,
    C6_MotProfAddTrajPoint = 262464,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ControlMode {
    PercentOutput = 0,
    Position = 1,
    Velocity = 2,
    Current = 3,
    Follower = 5,
    MotionProfile = 6,
    MotionMagic = 7,
    MotionProfileArc = 10,
    Disabled = 15,
}
#[repr(i32)]
/// How to interpret a demand value.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DemandType {
    /// Ignore the demand value and apply neutral/no-change.
    Neutral = 0,
    /**
     * When closed-looping, set the target of the aux PID loop to the demand value.
     *
     * When following, follow the processed output of the combined
     * primary/aux PID output.  The demand value is ignored.
     */
    AuxPID = 1,
    /// Simply add to the output
    ArbitraryFeedForward = 2,
}
impl Default for DemandType {
    #[inline]
    fn default() -> DemandType {
        DemandType::Neutral
    }
}

impl FeedbackDevice {
    pub const CTRE_MagEncoder_Absolute: FeedbackDevice = FeedbackDevice::PulseWidthEncodedPosition;
    pub const CTRE_MagEncoder_Relative: FeedbackDevice = FeedbackDevice::QuadEncoder;
}
#[repr(i32)]
/// Motor controller with gadgeteer connector.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FeedbackDevice {
    None = -1,
    QuadEncoder = 0,
    Analog = 2,
    Tachometer = 4,
    PulseWidthEncodedPosition = 8,
    SensorSum = 9,
    SensorDifference = 10,
    RemoteSensor0 = 11,
    RemoteSensor1 = 12,
    SoftwareEmulatedSensor = 15,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RemoteFeedbackDevice {
    None = -1,
    SensorSum = 9,
    SensorDifference = 10,
    RemoteSensor0 = 11,
    RemoteSensor1 = 12,
    SoftwareEmulatedSensor = 15,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FollowerType {
    PercentOutput = 0,
    AuxOutput1 = 1,
}
impl Default for FollowerType {
    #[inline]
    fn default() -> FollowerType {
        FollowerType::PercentOutput
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LimitSwitchSource {
    FeedbackConnector = 0,
    RemoteTalonSRX = 1,
    RemoteCANifier = 2,
    Deactivated = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RemoteLimitSwitchSource {
    RemoteTalonSRX = 1,
    RemoteCANifier = 2,
    Deactivated = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LimitSwitchNormal {
    NormallyOpen = 0,
    NormallyClosed = 1,
    Disabled = 2,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NeutralMode {
    /// Use the NeutralMode that is set by the jumper wire on the CAN device
    EEPROMSetting = 0,
    /// Stop the motor's rotation by applying a force.
    Coast = 1,
    /// Stop the motor's rotation by applying a force.
    Brake = 2,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RemoteSensorSource {
    Off = 0,
    TalonSRX_SelectedSensor = 1,
    Pigeon_Yaw = 2,
    Pigeon_Pitch = 3,
    Pigeon_Roll = 4,
    CANifier_Quadrature = 5,
    CANifier_PWMInput0 = 6,
    CANifier_PWMInput1 = 7,
    CANifier_PWMInput2 = 8,
    CANifier_PWMInput3 = 9,
    GadgeteerPigeon_Yaw = 10,
    GadgeteerPigeon_Pitch = 11,
    GadgeteerPigeon_Roll = 12,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SensorTerm {
    Sum0 = 0,
    Sum1 = 1,
    Diff0 = 2,
    Diff1 = 3,
}

#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatusFrameEnhanced {
    S1_General = 5120,
    S2_Feedback0 = 5184,
    S4_AinTempVbat = 5312,
    S6_Misc = 5440,
    S7_CommStatus = 5504,
    S9_MotProfBuffer = 5632,
    S10_Targets = 5696,
    S12_Feedback1 = 5824,
    S13_Base_PIDF0 = 5888,
    S14_Turn_PIDF1 = 5952,
    S15_FirmareApiStatus = 6016,
    S3_Quadrature = 5248,
    S8_PulseWidth = 5568,
    S11_UartGadgeteer = 5760,
}
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatusFrame {
    S1_General = 5120,
    S2_Feedback0 = 5184,
    S4_AinTempVbat = 5312,
    S6_Misc = 5440,
    S7_CommStatus = 5504,
    S9_MotProfBuffer = 5632,
    S10_Targets = 5696,
    S12_Feedback1 = 5824,
    S13_Base_PIDF0 = 5888,
    S14_Turn_PIDF1 = 5952,
    S15_FirmareApiStatus = 6016,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VelocityMeasPeriod {
    P1Ms = 1,
    P2Ms = 2,
    P5Ms = 5,
    P10Ms = 10,
    P20Ms = 20,
    P25Ms = 25,
    P50Ms = 50,
    P100Ms = 100,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SetValueMotionProfile {
    Invalid = -1,
    Disable = 0,
    Enable = 1,
    Hold = 2,
}
impl From<c_int> for SetValueMotionProfile {
    fn from(value: c_int) -> SetValueMotionProfile {
        match value {
            0 => SetValueMotionProfile::Disable,
            1 => SetValueMotionProfile::Enable,
            2 => SetValueMotionProfile::Hold,
            _ => SetValueMotionProfile::Invalid,
        }
    }
}
impl Default for SetValueMotionProfile {
    #[inline]
    fn default() -> SetValueMotionProfile {
        SetValueMotionProfile::Invalid
    }
}
#[repr(i32)]
/// Duration to apply to a particular trajectory pt.
/// This time unit is ADDED to the existing base time set by
/// ConfigMotionProfileTrajectoryPeriod().
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TrajectoryDuration {
    T0ms = 0,
    T5ms = 5,
    T10ms = 10,
    T20ms = 20,
    T30ms = 30,
    T40ms = 40,
    T50ms = 50,
    T100ms = 100,
}
impl Default for TrajectoryDuration {
    #[inline]
    fn default() -> TrajectoryDuration {
        TrajectoryDuration::T0ms
    }
}
/// Motion Profile Trajectory Point
/// This is simply a data transfer object.
#[repr(C)]
#[derive(Default, Debug)]
pub struct TrajectoryPoint {
    /// The position to servo to.
    pub position: f64,
    /// The velocity to feed-forward.
    pub velocity: f64,
    /// Not used.  Use auxiliary_pos instead.
    heading_deg: f64,
    /// The position for auxiliary PID to target.
    pub auxiliary_pos: f64,
    /// Which slot to get PIDF gains.
    /// PID is used for position servo.
    /// F is used as the Kv constant for velocity feed-forward.
    /// Typically this is hard-coded
    /// to a particular slot, but you are free to gain schedule if need be.
    /// gain schedule if need be.
    /// Choose from [0,3].
    pub profile_slot_select_0: u32,
    /// Which slot to get PIDF gains for auxiliary PID.
    /// This only has impact during MotionProfileArc Control mode.
    /// Choose from [0,1].
    pub profile_slot_select_1: u32,
    /// Set to true to signal Talon that this is the final point, so do not
    /// attempt to pop another trajectory point from out of the Talon buffer.
    /// Instead continue processing this way point.  Typically the velocity
    /// member variable should be zero so that the motor doesn't spin indefinitely.
    pub is_last_point: bool,
    /// Set to true to signal Talon to zero the selected sensor.
    /// When generating MPs, one simple method is to make the first target position zero,
    /// and the final target position the target distance from the current position.
    /// Then when you fire the MP, the current position gets set to zero.
    /// If this is the intent, you can set zeroPos on the first trajectory point.
    ///
    /// Otherwise you can leave this false for all points, and offset the positions
    /// of all trajectory points so they are correct.
    pub zero_pos: bool,
    /// Duration to apply this trajectory pt.
    /// This time unit is ADDED to the existing base time set by
    /// ConfigMotionProfileTrajectoryPeriod().
    pub time_dur: TrajectoryDuration,
}
#[test]
fn bindgen_test_layout_TrajectoryPoint() {
    assert_eq!(
        ::std::mem::size_of::<TrajectoryPoint>(),
        48usize,
        concat!("Size of: ", stringify!(TrajectoryPoint))
    );
    assert_eq!(
        ::std::mem::align_of::<TrajectoryPoint>(),
        8usize,
        concat!("Alignment of ", stringify!(TrajectoryPoint))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TrajectoryPoint>())).position as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(position)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TrajectoryPoint>())).velocity as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(velocity)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TrajectoryPoint>())).heading_deg as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(heading_deg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TrajectoryPoint>())).auxiliary_pos as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(auxiliary_pos)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TrajectoryPoint>())).profile_slot_select_0 as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(profile_slot_select_0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TrajectoryPoint>())).profile_slot_select_1 as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(profile_slot_select1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TrajectoryPoint>())).is_last_point as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(is_last_point)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TrajectoryPoint>())).zero_pos as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(zero_pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TrajectoryPoint>())).time_dur as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(TrajectoryPoint),
            "::",
            stringify!(time_dur)
        )
    );
}
/// Motion Profile Status
/// This is simply a data transer object.
#[repr(C)]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct MotionProfileStatus {
    /// The available empty slots in the trajectory buffer.
    ///
    /// The robot API holds a "top buffer" of trajectory points, so your applicaion
    /// can dump several points at once.  The API will then stream them into the Talon's
    /// low-level buffer, allowing the Talon to act on them.
    pub top_buffer_rem: c_int,
    /// The number of points in the top trajectory buffer.
    pub top_buffer_cnt: c_int,
    /// The number of points in the low level Talon buffer.
    pub btm_buffer_cnt: c_int,
    /// Set if `is_underrun` ever gets set.
    /// Only is cleared by clearMotionProfileHasUnderrun() to ensure
    /// robot logic can react or instrument it.
    /// @see clearMotionProfileHasUnderrun()
    pub has_underrun: bool,
    /// This is set if Talon needs to shift a point from its buffer into
    /// the active trajectory point however the buffer is empty. This gets cleared
    /// automatically when is resolved.
    pub is_underrun: bool,
    /// True if the active trajectory point has not empty, false otherwise.
    /// The members in activePoint are only valid if this signal is set.
    pub active_point_valid: bool,
    pub is_last: bool,
    /// Selected slot for PID Loop 0
    pub profile_slot_select_0: c_int,
    /// Selected slot for PID Loop 0
    pub profile_slot_select_1: c_int,
    /// The current output mode of the motion profile executer (disabled, enabled, or hold).
    /// When changing the set() value in MP mode, it's important to check this signal to
    /// confirm the change takes effect before interacting with the top buffer.
    pub output_enable: SetValueMotionProfile,
    /// The applied duration of the active trajectory point
    pub time_dur_ms: c_int,
}
#[test]
fn bindgen_test_layout_MotionProfileStatus() {
    assert_eq!(
        ::std::mem::size_of::<MotionProfileStatus>(),
        32usize,
        concat!("Size of: ", stringify!(MotionProfileStatus))
    );
    assert_eq!(
        ::std::mem::align_of::<MotionProfileStatus>(),
        4usize,
        concat!("Alignment of ", stringify!(MotionProfileStatus))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MotionProfileStatus>())).top_buffer_rem as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(top_buffer_rem)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MotionProfileStatus>())).top_buffer_cnt as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(top_buffer_cnt)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MotionProfileStatus>())).btm_buffer_cnt as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(btm_buffer_cnt)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MotionProfileStatus>())).has_underrun as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(has_underrun)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MotionProfileStatus>())).is_underrun as *const _ as usize },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(is_underrun)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MotionProfileStatus>())).active_point_valid as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(active_point_valid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MotionProfileStatus>())).is_last as *const _ as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(is_last)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MotionProfileStatus>())).profile_slot_select_0 as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(profile_slot_select_0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MotionProfileStatus>())).profile_slot_select_1 as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(profile_slot_select_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MotionProfileStatus>())).output_enable as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(output_enable)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MotionProfileStatus>())).time_dur_ms as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(MotionProfileStatus),
            "::",
            stringify!(time_dur_ms)
        )
    );
}

extern "C" {
    pub fn c_MotController_Create1(baseArbId: c_int) -> Handle;

    pub fn c_MotController_GetDeviceNumber(handle: Handle, deviceNumber: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetDescription(
        handle: Handle,
        toFill: *mut c_char,
        toFillByteSz: c_int,
        numBytesFilled: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_SetDemand(
        handle: Handle,
        mode: c_int,
        demand0: c_int,
        demand1: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_Set_4(
        handle: Handle,
        mode: c_int,
        demand0: f64,
        demand1: f64,
        demand1Type: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_SetNeutralMode(handle: Handle, neutralMode: c_int);

    pub fn c_MotController_SetSensorPhase(handle: Handle, PhaseSensor: bool);

    pub fn c_MotController_SetInverted(handle: Handle, invert: bool);

    pub fn c_MotController_ConfigOpenLoopRamp(
        handle: Handle,
        secondsFromNeutralToFull: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigClosedLoopRamp(
        handle: Handle,
        secondsFromNeutralToFull: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigPeakOutputForward(
        handle: Handle,
        percentOut: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigPeakOutputReverse(
        handle: Handle,
        percentOut: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigNominalOutputForward(
        handle: Handle,
        percentOut: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigNominalOutputReverse(
        handle: Handle,
        percentOut: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigNeutralDeadband(
        handle: Handle,
        percentDeadband: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigVoltageCompSaturation(
        handle: Handle,
        voltage: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigVoltageMeasurementFilter(
        handle: Handle,
        filterWindowSamples: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_EnableVoltageCompensation(handle: Handle, enable: bool);

    pub fn c_MotController_GetBusVoltage(handle: Handle, voltage: *mut f64) -> ErrorCode;

    pub fn c_MotController_GetMotorOutputPercent(
        handle: Handle,
        percentOutput: *mut f64,
    ) -> ErrorCode;

    pub fn c_MotController_GetOutputCurrent(handle: Handle, current: *mut f64) -> ErrorCode;

    pub fn c_MotController_GetTemperature(handle: Handle, temperature: *mut f64) -> ErrorCode;

    pub fn c_MotController_ConfigSelectedFeedbackSensor(
        handle: Handle,
        feedbackDevice: c_int,
        pidIdx: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigSelectedFeedbackCoefficient(
        handle: Handle,
        coefficient: f64,
        pidIdx: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigRemoteFeedbackFilter(
        handle: Handle,
        deviceID: c_int,
        remoteSensorSource: c_int,
        remoteOrdinal: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigSensorTerm(
        handle: Handle,
        sensorTerm: c_int,
        feedbackDevice: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetSelectedSensorPosition(
        handle: Handle,
        param: *mut c_int,
        pidIdx: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetSelectedSensorVelocity(
        handle: Handle,
        param: *mut c_int,
        pidIdx: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_SetSelectedSensorPosition(
        handle: Handle,
        sensorPos: c_int,
        pidIdx: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_SetControlFramePeriod(
        handle: Handle,
        frame: c_int,
        periodMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_SetStatusFramePeriod(
        handle: Handle,
        frame: c_int,
        periodMs: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetStatusFramePeriod(
        handle: Handle,
        frame: c_int,
        periodMs: *mut c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigVelocityMeasurementPeriod(
        handle: Handle,
        period: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigVelocityMeasurementWindow(
        handle: Handle,
        windowSize: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigForwardLimitSwitchSource(
        handle: Handle,
        type_: c_int,
        normalOpenOrClose: c_int,
        deviceID: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigReverseLimitSwitchSource(
        handle: Handle,
        type_: c_int,
        normalOpenOrClose: c_int,
        deviceID: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_OverrideLimitSwitchesEnable(handle: Handle, enable: bool);

    pub fn c_MotController_ConfigForwardSoftLimitThreshold(
        handle: Handle,
        forwardSensorLimit: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigReverseSoftLimitThreshold(
        handle: Handle,
        reverseSensorLimit: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigForwardSoftLimitEnable(
        handle: Handle,
        enable: bool,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigReverseSoftLimitEnable(
        handle: Handle,
        enable: bool,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_OverrideSoftLimitsEnable(handle: Handle, enable: bool);

    pub fn c_MotController_Config_kP(
        handle: Handle,
        slotIdx: c_int,
        value: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_Config_kI(
        handle: Handle,
        slotIdx: c_int,
        value: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_Config_kD(
        handle: Handle,
        slotIdx: c_int,
        value: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_Config_kF(
        handle: Handle,
        slotIdx: c_int,
        value: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_Config_IntegralZone(
        handle: Handle,
        slotIdx: c_int,
        izone: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigAllowableClosedloopError(
        handle: Handle,
        slotIdx: c_int,
        allowableClosedLoopError: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigMaxIntegralAccumulator(
        handle: Handle,
        slotIdx: c_int,
        iaccum: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigClosedLoopPeakOutput(
        handle: Handle,
        slotIdx: c_int,
        percentOut: f64,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigClosedLoopPeriod(
        handle: Handle,
        slotIdx: c_int,
        loopTimeMs: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_SetIntegralAccumulator(
        handle: Handle,
        iaccum: f64,
        pidIdx: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetClosedLoopError(
        handle: Handle,
        closedLoopError: *mut c_int,
        pidIdx: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetIntegralAccumulator(
        handle: Handle,
        iaccum: *mut f64,
        pidIdx: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetErrorDerivative(
        handle: Handle,
        derror: *mut f64,
        pidIdx: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_SelectProfileSlot(
        handle: Handle,
        slotIdx: c_int,
        pidIdx: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetActiveTrajectoryPosition(
        handle: Handle,
        param: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetActiveTrajectoryVelocity(
        handle: Handle,
        param: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetActiveTrajectoryHeading(handle: Handle, param: *mut f64)
        -> ErrorCode;

    pub fn c_MotController_GetActiveTrajectoryAll(
        handle: Handle,
        vel: *mut c_int,
        pos: *mut c_int,
        heading: *mut f64,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigMotionCruiseVelocity(
        handle: Handle,
        sensorUnitsPer100ms: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigMotionAcceleration(
        handle: Handle,
        sensorUnitsPer100msPerSec: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ClearMotionProfileTrajectories(handle: Handle) -> ErrorCode;

    pub fn c_MotController_GetMotionProfileTopLevelBufferCount(
        handle: Handle,
        value: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_PushMotionProfileTrajectory(
        handle: Handle,
        position: f64,
        velocity: f64,
        headingDeg: f64,
        profileSlotSelect: c_int,
        isLastPoint: bool,
        zeroPos: bool,
    ) -> ErrorCode;

    pub fn c_MotController_PushMotionProfileTrajectory_2(
        handle: Handle,
        position: f64,
        velocity: f64,
        headingDeg: f64,
        profileSlotSelect0: c_int,
        profileSlotSelect1: c_int,
        isLastPoint: bool,
        zeroPos: bool,
        durationMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_IsMotionProfileTopLevelBufferFull(
        handle: Handle,
        value: *mut bool,
    ) -> ErrorCode;

    pub fn c_MotController_ProcessMotionProfileBuffer(handle: Handle) -> ErrorCode;

    pub fn c_MotController_GetMotionProfileStatus(
        handle: Handle,
        topBufferRem: *mut c_int,
        topBufferCnt: *mut c_int,
        btmBufferCnt: *mut c_int,
        hasUnderrun: *mut bool,
        isUnderrun: *mut bool,
        activePointValid: *mut bool,
        isLast: *mut bool,
        profileSlotSelect: *mut c_int,
        outputEnable: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetMotionProfileStatus_2(
        handle: Handle,
        topBufferRem: *mut c_int,
        topBufferCnt: *mut c_int,
        btmBufferCnt: *mut c_int,
        hasUnderrun: *mut bool,
        isUnderrun: *mut bool,
        activePointValid: *mut bool,
        isLast: *mut bool,
        profileSlotSelect: *mut c_int,
        outputEnable: *mut c_int,
        timeDurMs: *mut c_int,
        profileSlotSelect1: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ClearMotionProfileHasUnderrun(
        handle: Handle,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ChangeMotionControlFramePeriod(
        handle: Handle,
        periodMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigMotionProfileTrajectoryPeriod(
        handle: Handle,
        durationMs: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetLastError(handle: Handle) -> ErrorCode;

    pub fn c_MotController_GetFirmwareVersion(handle: Handle, arg1: *mut c_int) -> ErrorCode;

    pub fn c_MotController_HasResetOccurred(handle: Handle, arg1: *mut bool) -> ErrorCode;

    pub fn c_MotController_ConfigSetCustomParam(
        handle: Handle,
        newValue: c_int,
        paramIndex: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigGetCustomParam(
        handle: Handle,
        readValue: *mut c_int,
        paramIndex: c_int,
        timoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigSetParameter(
        handle: Handle,
        param: c_int,
        value: f64,
        subValue: c_int,
        ordinal: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigGetParameter(
        handle: Handle,
        param: c_int,
        value: *mut f64,
        ordinal: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigPeakCurrentLimit(
        handle: Handle,
        amps: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigPeakCurrentDuration(
        handle: Handle,
        milliseconds: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_ConfigContinuousCurrentLimit(
        handle: Handle,
        amps: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_EnableCurrentLimit(handle: Handle, enable: bool) -> ErrorCode;

    pub fn c_MotController_SetLastError(handle: Handle, error: c_int) -> ErrorCode;

    pub fn c_MotController_GetAnalogIn(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_SetAnalogPosition(
        handle: Handle,
        newPosition: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetAnalogInRaw(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetAnalogInVel(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetQuadraturePosition(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_SetQuadraturePosition(
        handle: Handle,
        newPosition: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetQuadratureVelocity(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetPulseWidthPosition(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_SetPulseWidthPosition(
        handle: Handle,
        newPosition: c_int,
        timeoutMs: c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetPulseWidthVelocity(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetPulseWidthRiseToFallUs(
        handle: Handle,
        param: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetPulseWidthRiseToRiseUs(
        handle: Handle,
        param: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetPinStateQuadA(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetPinStateQuadB(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetPinStateQuadIdx(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_IsFwdLimitSwitchClosed(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_IsRevLimitSwitchClosed(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetFaults(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_GetStickyFaults(handle: Handle, param: *mut c_int) -> ErrorCode;

    pub fn c_MotController_ClearStickyFaults(handle: Handle, timeoutMs: c_int) -> ErrorCode;

    pub fn c_MotController_SelectDemandType(handle: Handle, enable: bool) -> ErrorCode;

    pub fn c_MotController_SetMPEOutput(handle: Handle, MpeOutput: c_int) -> ErrorCode;

    pub fn c_MotController_EnableHeadingHold(handle: Handle, enable: bool) -> ErrorCode;

    pub fn c_MotController_GetAnalogInAll(
        handle: Handle,
        withOv: *mut c_int,
        raw: *mut c_int,
        vel: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetQuadratureSensor(
        handle: Handle,
        pos: *mut c_int,
        vel: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetPulseWidthAll(
        handle: Handle,
        pos: *mut c_int,
        vel: *mut c_int,
        riseToRiseUs: *mut c_int,
        riseToFallUs: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetQuadPinStates(
        handle: Handle,
        quadA: *mut c_int,
        quadB: *mut c_int,
        quadIdx: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetLimitSwitchState(
        handle: Handle,
        isFwdClosed: *mut c_int,
        isRevClosed: *mut c_int,
    ) -> ErrorCode;

    pub fn c_MotController_GetClosedLoopTarget(
        handle: Handle,
        value: *mut c_int,
        pidIdx: c_int,
    ) -> ErrorCode;
}
