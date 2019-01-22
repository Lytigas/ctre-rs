#![allow(non_snake_case)]

use ctre_sys::ctre::phoenix::motorcontrol::{
    FeedbackDevice, LimitSwitchNormal, LimitSwitchSource, RemoteSensorSource, VelocityMeasPeriod,
};

#[derive(Clone, Debug)]
pub struct CustomParamConfig {
    pub customParam_0: i32,
    pub customParam_1: i32,
}
impl Default for CustomParamConfig {
    fn default() -> Self {
        Self {
            customParam_0: 0,
            customParam_1: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BaseMotorConfig {
    pub custom: CustomParamConfig,
    pub openloopRamp: f64,
    pub closedloopRamp: f64,
    pub peakOutputForward: f64,
    pub peakOutputReverse: f64,
    pub nominalOutputForward: f64,
    pub nominalOutputReverse: f64,
    pub neutralDeadband: f64,
    pub voltageCompSaturation: f64,
    pub voltageMeasurementFilter: i32,
    pub velocityMeasurementPeriod: VelocityMeasPeriod,
    pub velocityMeasurementWindow: i32,
    pub forwardLimitSwitchDeviceID: i32, //Limit Switch device id isn't used unless device is a remote
    pub reverseLimitSwitchDeviceID: i32,
    pub forwardLimitSwitchNormal: LimitSwitchNormal,
    pub reverseLimitSwitchNormal: LimitSwitchNormal,
    pub forwardSoftLimitThreshold: i32,
    pub reverseSoftLimitThreshold: i32,
    pub forwardSoftLimitEnable: bool,
    pub reverseSoftLimitEnable: bool,
    pub slot_0: SlotConfiguration,
    pub slot_1: SlotConfiguration,
    pub slot_2: SlotConfiguration,
    pub slot_3: SlotConfiguration,
    pub auxPIDPolarity: bool,
    pub filter_0: FilterConfiguration,
    pub filter_1: FilterConfiguration,
    pub motionCruiseVelocity: i32,
    pub motionAcceleration: i32,
    pub motionProfileTrajectoryPeriod: i32,
    pub feedbackNotContinuous: bool,
    pub remoteSensorClosedLoopDisableNeutralOnLOS: bool,
    pub clearPositionOnLimitF: bool,
    pub clearPositionOnLimitR: bool,
    pub clearPositionOnQuadIdx: bool,
    pub limitSwitchDisableNeutralOnLOS: bool,
    pub softLimitDisableNeutralOnLOS: bool,
    pub pulseWidthPeriod_EdgesPerRot: i32,
    pub pulseWidthPeriod_FilterWindowSz: i32,
}
impl Default for BaseMotorConfig {
    fn default() -> Self {
        Self {
            custom: Default::default(),
            openloopRamp: 0.0,
            closedloopRamp: 0.0,
            peakOutputForward: 1.0,
            peakOutputReverse: -1.0,
            nominalOutputForward: 0.0,
            nominalOutputReverse: 0.0,
            neutralDeadband: 0.04,
            voltageCompSaturation: 0.0,
            voltageMeasurementFilter: 32,
            velocityMeasurementPeriod: VelocityMeasPeriod::Period_100Ms,
            velocityMeasurementWindow: 64,
            forwardLimitSwitchDeviceID: 0,
            reverseLimitSwitchDeviceID: 0,
            forwardLimitSwitchNormal: LimitSwitchNormal::NormallyOpen,
            reverseLimitSwitchNormal: LimitSwitchNormal::NormallyOpen,
            forwardSoftLimitThreshold: 0,
            reverseSoftLimitThreshold: 0,
            forwardSoftLimitEnable: false,
            reverseSoftLimitEnable: false,
            slot_0: Default::default(),
            slot_1: Default::default(),
            slot_2: Default::default(),
            slot_3: Default::default(),
            auxPIDPolarity: false,
            filter_0: Default::default(),
            filter_1: Default::default(),
            motionCruiseVelocity: 0,
            motionAcceleration: 0,
            motionProfileTrajectoryPeriod: 0,
            feedbackNotContinuous: false,
            remoteSensorClosedLoopDisableNeutralOnLOS: false,
            clearPositionOnLimitF: false,
            clearPositionOnLimitR: false,
            clearPositionOnQuadIdx: false,
            limitSwitchDisableNeutralOnLOS: false,
            softLimitDisableNeutralOnLOS: false,
            pulseWidthPeriod_EdgesPerRot: 1,
            pulseWidthPeriod_FilterWindowSz: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TalonSRXConfig {
    pub base: BaseMotorConfig,
    pub primaryPID: TalonSRXPIDSetConfiguration,
    pub auxilaryPID: TalonSRXPIDSetConfiguration,
    pub forwardLimitSwitchSource: LimitSwitchSource,
    pub reverseLimitSwitchSource: LimitSwitchSource,
    pub sum_0: FeedbackDevice,
    pub sum_1: FeedbackDevice,
    pub diff_0: FeedbackDevice,
    pub diff_1: FeedbackDevice,
    pub peakCurrentLimit: i32,
    pub peakCurrentDuration: i32,
    pub continuousCurrentLimit: i32,
}
impl Default for TalonSRXConfig {
    fn default() -> Self {
        Self {
            base: Default::default(),
            primaryPID: Default::default(),
            auxilaryPID: Default::default(),
            forwardLimitSwitchSource: LimitSwitchSource::FeedbackConnector,
            reverseLimitSwitchSource: LimitSwitchSource::FeedbackConnector,
            sum_0: FeedbackDevice::QuadEncoder,
            sum_1: FeedbackDevice::QuadEncoder,
            diff_0: FeedbackDevice::QuadEncoder,
            diff_1: FeedbackDevice::QuadEncoder,
            peakCurrentLimit: 1,
            peakCurrentDuration: 1,
            continuousCurrentLimit: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BasePIDSetConfig {
    pub selectedFeedbackCoefficient: f64,
}
impl Default for BasePIDSetConfig {
    fn default() -> Self {
        Self {
            selectedFeedbackCoefficient: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FilterConfiguration {
    pub remoteSensorDeviceID: i32,
    pub remoteSensorSource: RemoteSensorSource,
}
impl Default for FilterConfiguration {
    fn default() -> Self {
        Self {
            remoteSensorDeviceID: 0,
            remoteSensorSource: RemoteSensorSource::Off,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TalonSRXPIDSetConfiguration {
    pub base: BasePIDSetConfig,
    pub selectedFeedbackSensor: FeedbackDevice,
}
impl Default for TalonSRXPIDSetConfiguration {
    fn default() -> Self {
        Self {
            base: Default::default(),
            selectedFeedbackSensor: FeedbackDevice::QuadEncoder,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SlotConfiguration {
    pub kP: f64,
    pub kI: f64,
    pub kD: f64,
    pub kF: f64,
    pub integralZone: i32,
    pub allowableClosedloopError: i32,
    pub maxIntegralAccumulator: f64,
    pub closedLoopPeakOutput: f64,
    pub closedLoopPeriod: i32,
}
impl Default for SlotConfiguration {
    fn default() -> Self {
        Self {
            kP: 0.0,
            kI: 0.0,
            kD: 0.0,
            kF: 0.0,
            integralZone: 0,
            allowableClosedloopError: 0,
            maxIntegralAccumulator: 0.0,
            closedLoopPeakOutput: 1.0,
            closedLoopPeriod: 1,
        }
    }
}
