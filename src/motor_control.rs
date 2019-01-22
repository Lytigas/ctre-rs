//! Support for motor controllers (Talon SRX and Victor SPX).
use std::ffi::c_void;

pub use ctre_sys::ctre::phoenix::motorcontrol::{
    ControlFrame, ControlFrameEnhanced, ControlMode, DemandType, FeedbackDevice, FollowerType,
    LimitSwitchNormal, LimitSwitchSource, NeutralMode, RemoteFeedbackDevice,
    RemoteLimitSwitchSource, RemoteSensorSource, SensorTerm, StatusFrame, StatusFrameEnhanced,
    VelocityMeasPeriod,
};
use ctre_sys::*;
#[cfg(feature = "usage-reporting")]
use wpilib_sys::usage::report_usage;

use super::{
    ctre_sys::ctre::phoenix::motion::{MotionProfileStatus, TrajectoryPoint},
    ErrorCode, ParamEnum, Result,
};

pub mod config;

#[derive(Debug, Copy, Clone)]
pub struct Faults(i32);
impl Faults {
    pub fn under_voltage(self) -> bool {
        self.0 & 1 != 0
    }
    pub fn forward_limit_switch(self) -> bool {
        self.0 & (1 << 1) != 0
    }
    pub fn reverse_limit_switch(self) -> bool {
        self.0 & (1 << 2) != 0
    }
    pub fn forward_soft_limit(self) -> bool {
        self.0 & (1 << 3) != 0
    }
    pub fn reverse_soft_limit(self) -> bool {
        self.0 & (1 << 4) != 0
    }
    pub fn hardware_failure(self) -> bool {
        self.0 & (1 << 5) != 0
    }
    pub fn reset_during_en(self) -> bool {
        self.0 & (1 << 6) != 0
    }
    pub fn sensor_overflow(self) -> bool {
        self.0 & (1 << 7) != 0
    }
    pub fn sensor_out_of_phase(self) -> bool {
        self.0 & (1 << 8) != 0
    }
    pub fn hardware_esd_reset(self) -> bool {
        self.0 & (1 << 9) != 0
    }
    pub fn remote_loss_of_signal(self) -> bool {
        self.0 & (1 << 10) != 0
    }
    /// True iff any of the above flags are true.
    pub fn has_any_fault(self) -> bool {
        self.0 != 0
    }
}
impl_binary_fmt!(Faults);

#[derive(Debug, Copy, Clone)]
pub struct StickyFaults(i32);
impl StickyFaults {
    pub fn under_voltage(self) -> bool {
        self.0 & 1 != 0
    }
    pub fn forward_limit_switch(self) -> bool {
        self.0 & (1 << 1) != 0
    }
    pub fn reverse_limit_switch(self) -> bool {
        self.0 & (1 << 2) != 0
    }
    pub fn forward_soft_limit(self) -> bool {
        self.0 & (1 << 3) != 0
    }
    pub fn reverse_soft_limit(self) -> bool {
        self.0 & (1 << 4) != 0
    }
    pub fn reset_during_en(self) -> bool {
        self.0 & (1 << 5) != 0
    }
    pub fn sensor_overflow(self) -> bool {
        self.0 & (1 << 6) != 0
    }
    pub fn sensor_out_of_phase(self) -> bool {
        self.0 & (1 << 7) != 0
    }
    pub fn hardware_esd_reset(self) -> bool {
        self.0 & (1 << 8) != 0
    }
    pub fn remote_loss_of_signal(self) -> bool {
        self.0 & (1 << 9) != 0
    }
    /// True iff any of the above flags are true.
    pub fn has_any_fault(self) -> bool {
        self.0 != 0
    }
}
impl_binary_fmt!(StickyFaults);

/// Base motor controller features for all CTRE CAN motor controllers.
///
/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait MotorController: private::Sealed {
    /// Constructor.
    /// * `device_number` - [0,62]
    fn new(device_number: i32) -> Self
    where
        Self: Sized;

    #[doc(hidden)]
    fn handle(&self) -> *mut c_void;
    fn get_base_id(&self) -> i32;
    fn get_device_id(&self) -> i32 {
        cci_get_only!(c_MotController_GetDeviceNumber(self.handle(), _: i32))
    }

    /**
     * * `mode` - Sets the appropriate output on the talon, depending on the mode.
     * * `demand0` - The output value to apply.
     *   such as advanced feed forward and/or auxiliary close-looping in firmware.
     *   * In PercentOutput, the output is between -1.0 and 1.0, with 0.0 as stopped.
     *   * In Current mode, output value is in amperes.
     *   * In Velocity mode, output value is in position change / 100ms.
     *   * In Position mode, output value is in encoder ticks or an analog value,
     *     depending on the sensor. See
     *   * In Follower mode, the output value is the integer device ID of the talon to duplicate.
     * * `demand1Type` - The demand type for demand1.
     * * `demand1` - Supplmental output value.  Units match the set mode.
     *
     * # Examples
     *
     * Arcade Drive Example:
     * ```
     * talonLeft.set(ControlMode::PercentOutput, joyForward, DemandType::ArbitraryFeedForward, +joyTurn);
     * talonRght.set(ControlMode::PercentOutput, joyForward, DemandType::ArbitraryFeedForward, -joyTurn);
     * ```
     *
     * Drive Straight Example:
     * Note: Selected Sensor Configuration is necessary for both PID0 and PID1.
     * ```
     * talonLeft.follow(talonRght, FollowerType::AuxOutput1);
     * talonRght.set(ControlMode::PercentOutput, joyForward, DemandType::AuxPID, desiredRobotHeading);
     * ```
     *
     * Drive Straight to a Distance Example:
     * Note: Other configurations (sensor selection, PID gains, etc.) need to be set.
     * ```
     * talonLeft.follow(talonRght, FollowerType::AuxOutput1);
     * talonRght.set(ControlMode::MotionMagic, targetDistance, DemandType::AuxPID, desiredRobotHeading);
     * ```
     */
    fn set(&mut self, mode: ControlMode, demand0: f64, demand1_type: DemandType, demand1: f64) {
        match mode {
            ControlMode::Follower => {
                // did caller specify device ID
                let work = if 0.0 <= demand0 && demand0 <= 62.0 {
                    ((self.get_base_id() as u32 >> 16) << 8) | (demand0 as u32)
                } else {
                    demand0 as u32
                };
                unsafe {
                    /* single precision guarantees 16bits of integral precision,
                     *  so float/double cast on work is safe */
                    c_MotController_Set_4(
                        self.handle(),
                        mode as _,
                        work as f64,
                        demand1,
                        demand1_type as _,
                    )
                }
            }
            ControlMode::Current => unsafe {
                // milliamps
                c_MotController_SetDemand(self.handle(), mode as _, (1000.0 * demand0) as _, 0)
            },
            ControlMode::PercentOutput
            //| ControlMode::TimedPercentOutput
            | ControlMode::Velocity
            | ControlMode::Position
            | ControlMode::MotionMagic
            //| ControlMode::MotionMagicArc
            | ControlMode::MotionProfile
            | ControlMode::MotionProfileArc => unsafe {
                c_MotController_Set_4(
                    self.handle(),
                    mode as _,
                    demand0,
                    demand1,
                    demand1_type as _,
                )
            },
            ControlMode::Disabled => unsafe {
                c_MotController_SetDemand(self.handle(), mode as _, 0, 0)
            },
        };
    }

    /// Neutral the motor output by setting control mode to disabled.
    fn neutral_output(&mut self) {
        self.set(ControlMode::Disabled, 0.0, DemandType::Neutral, 0.0)
    }
    /// Sets the mode of operation during neutral throttle output.
    fn set_neutral_mode(&mut self, neutral_mode: NeutralMode) {
        unsafe { c_MotController_SetNeutralMode(self.handle(), neutral_mode as _) }
    }

    /**
     * Sets the phase of the sensor. Use when controller forward/reverse output
     * doesn't correlate to appropriate forward/reverse reading of sensor.
     * Pick a value so that positive PercentOutput yields a positive change in sensor.
     * After setting this, user can freely call [`set_inverted`] with any value.
     *
     * * `phase_sensor` - Indicates whether to invert the phase of the sensor.
     *
     * [`set_inverted`]: #method.set_inverted
     */
    fn set_sensor_phase(&mut self, phase_sensor: bool) {
        unsafe { c_MotController_SetSensorPhase(self.handle(), phase_sensor) }
    }
    /**
     * Inverts the hbridge output of the motor controller.
     *
     * This does not impact sensor phase and should not be used to correct sensor polarity.
     *
     * This will invert the hbridge output but NOT the LEDs.
     * This ensures....
     *  - Green LEDs always represents positive request from robot-controller/closed-looping mode.
     *  - Green LEDs correlates to forward limit switch.
     *  - Green LEDs correlates to forward soft limit.
     */
    fn set_inverted(&mut self, invert: bool) {
        unsafe { c_MotController_SetInverted(self.handle(), invert) }
    }

    fn config_openloop_ramp(
        &mut self,
        seconds_from_neutral_to_full: f64,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigOpenLoopRamp(
                self.handle(),
                seconds_from_neutral_to_full,
                timeout_ms,
            )
        }
        .into()
    }
    fn config_closedloop_ramp(
        &mut self,
        seconds_from_neutral_to_full: f64,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigClosedLoopRamp(
                self.handle(),
                seconds_from_neutral_to_full,
                timeout_ms,
            )
        }
        .into()
    }

    fn config_peak_output_forward(&mut self, percent_out: f64, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigPeakOutputForward(self.handle(), percent_out, timeout_ms) }
            .into()
    }
    fn config_peak_output_reverse(&mut self, percent_out: f64, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigPeakOutputReverse(self.handle(), percent_out, timeout_ms) }
            .into()
    }

    fn config_nominal_output_forward(&mut self, percent_out: f64, timeout_ms: i32) -> Result<()> {
        unsafe {
            c_MotController_ConfigNominalOutputForward(self.handle(), percent_out, timeout_ms)
        }
        .into()
    }
    fn config_nominal_output_reverse(&mut self, percent_out: f64, timeout_ms: i32) -> Result<()> {
        unsafe {
            c_MotController_ConfigNominalOutputReverse(self.handle(), percent_out, timeout_ms)
        }
        .into()
    }

    fn config_neutral_deadband(&mut self, percent_deadband: f64, timeout_ms: i32) -> Result<()> {
        unsafe {
            c_MotController_ConfigNeutralDeadband(self.handle(), percent_deadband, timeout_ms)
        }
        .into()
    }

    /**
     * Configures the Voltage Compensation saturation voltage.
     *
     * * `voltage` - The max voltage to apply to the hbridge when voltage
     *   compensation is enabled.  For example, if 10 (volts) is specified
     *   and a TalonSRX is commanded to 0.5 (PercentOutput, closed-loop, etc)
     *   then the TalonSRX will attempt to apply a duty-cycle to produce 5V.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    fn config_voltage_comp_saturation(&mut self, voltage: f64, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigVoltageCompSaturation(self.handle(), voltage, timeout_ms) }
            .into()
    }
    /// Configures the voltage measurement filter.
    /// * `filter_window_samples` - Number of samples in the rolling average of voltage measurement.
    fn config_voltage_measurement_filter(
        &mut self,
        filter_window_samples: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigVoltageMeasurementFilter(
                self.handle(),
                filter_window_samples,
                timeout_ms,
            )
        }
        .into()
    }
    /// Enable voltage compensation.
    /// If enabled, voltage compensation works in all control modes.
    fn enable_voltage_compensation(&mut self, enable: bool) {
        unsafe { c_MotController_EnableVoltageCompensation(self.handle(), enable) }
    }

    fn get_bus_voltage(&self) -> Result<f64> {
        cci_get_call!(c_MotController_GetBusVoltage(self.handle(), _: f64))
    }
    /// Gets the output percentage of the motor controller, in the interval [-1,+1].
    fn get_motor_output_percent(&self) -> Result<f64> {
        cci_get_call!(c_MotController_GetMotorOutputPercent(self.handle(), _: f64))
    }
    fn get_motor_output_voltage(&self) -> Result<f64> {
        Ok(self.get_bus_voltage()? * self.get_motor_output_percent()?)
    }

    // output current moved to TalonSRX

    /// Gets the temperature of the motor controller in degrees Celsius.
    fn get_temperature(&self) -> Result<f64> {
        cci_get_call!(c_MotController_GetTemperature(self.handle(), _: f64))
    }

    /**
     * Select the remote feedback device for the motor controller.
     * Most CTRE CAN motor controllers will support remote sensors over CAN.
     *
     * * `feedback_device` - Remote Feedback Device to select.
     * * `pid_idx` - 0 for Primary closed-loop. 1 for auxiliary closed-loop.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    fn config_selected_feedback_sensor(
        &mut self,
        feedback_device: RemoteFeedbackDevice,
        pid_idx: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigSelectedFeedbackSensor(
                self.handle(),
                feedback_device as _,
                pid_idx,
                timeout_ms,
            )
        }
        .into()
    }
    /**
     * The Feedback Coefficient is a scalar applied to the value of the
     * feedback sensor.  Useful when you need to scale your sensor values
     * within the closed-loop calculations.  Default value is 1.
     *
     * Selected Feedback Sensor register in firmware is the decoded sensor value
     * multiplied by the Feedback Coefficient.
     *
     * * `coefficient` - Feedback Coefficient value.  Maximum value of 1.
     *   Resolution is 1/(2^16).  Cannot be 0.
     * * `pid_idx` - 0 for Primary closed-loop. 1 for auxiliary closed-loop.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    fn config_selected_feedback_coefficient(
        &mut self,
        coefficient: f64,
        pid_idx: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigSelectedFeedbackCoefficient(
                self.handle(),
                coefficient,
                pid_idx,
                timeout_ms,
            )
        }
        .into()
    }

    /**
     * Select what remote device and signal to assign to Remote Sensor 0 or Remote Sensor 1.
     * After binding a remote device and signal to Remote Sensor X, you may select Remote Sensor X
     * as a PID source for closed-loop features.
     */
    fn config_remote_feedback_filter(
        &mut self,
        device_id: i32,
        remote_sensor_source: RemoteSensorSource,
        remote_ordinal: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigRemoteFeedbackFilter(
                self.handle(),
                device_id,
                remote_sensor_source as _,
                remote_ordinal,
                timeout_ms,
            )
        }
        .into()
    }

    /**
     * Select what sensor term should be bound to switch feedback device.
     * Sensor Sum = Sensor Sum Term 0 - Sensor Sum Term 1
     * Sensor Difference = Sensor Diff Term 0 - Sensor Diff Term 1
     * The four terms are specified with this routine.  Then Sensor Sum/Difference
     * can be selected for closed-looping.
     */
    fn config_sensor_term(
        &mut self,
        sensor_term: SensorTerm,
        feedback_device: FeedbackDevice,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigSensorTerm(
                self.handle(),
                sensor_term as _,
                feedback_device as _,
                timeout_ms,
            )
        }
        .into()
    }

    /// Get the selected sensor position (in raw sensor units).
    fn get_selected_sensor_position(&self, pid_idx: i32) -> Result<i32> {
        cci_get_call!(c_MotController_GetSelectedSensorPosition(
            self.handle(),
            _: i32,
            pid_idx,
        ))
    }
    fn get_selected_sensor_velocity(&self, pid_idx: i32) -> Result<i32> {
        cci_get_call!(c_MotController_GetSelectedSensorVelocity(
            self.handle(),
            _: i32,
            pid_idx,
        ))
    }
    fn set_selected_sensor_position(
        &mut self,
        sensor_pos: i32,
        pid_idx: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_SetSelectedSensorPosition(
                self.handle(),
                sensor_pos,
                pid_idx,
                timeout_ms,
            )
        }
        .into()
    }

    fn set_control_frame_period(&mut self, frame: ControlFrame, period_ms: i32) -> Result<()> {
        unsafe { c_MotController_SetControlFramePeriod(self.handle(), frame as _, period_ms) }
            .into()
    }
    fn set_status_frame_period(
        &mut self,
        frame: StatusFrame,
        period_ms: u8,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_SetStatusFramePeriod(self.handle(), frame as _, period_ms, timeout_ms)
        }
        .into()
    }
    fn get_status_frame_period(&self, frame: StatusFrame, timeout_ms: i32) -> Result<i32> {
        cci_get_call!(
            c_MotController_GetStatusFramePeriod(self.handle(), frame as _, _: i32, timeout_ms)
        )
    }

    /**
     * Configures the forward limit switch for a remote source.
     * For example, a CAN motor controller may need to monitor the Limit-F pin
     * of another Talon or CANifier.
     *
     * * `type_` - Remote limit switch source.
     *   User can choose between a remote Talon SRX, CANifier, or deactivate the feature.
     * * `normal_open_or_close` - Setting for normally open, normally closed, or disabled.
     *   This setting matches the web-based configuration drop down.
     * * `device_id` - Device ID of remote source (Talon SRX or CANifier device ID).
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    fn config_forward_limit_switch_source(
        &mut self,
        type_: RemoteLimitSwitchSource,
        normal_open_or_close: LimitSwitchNormal,
        device_id: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigForwardLimitSwitchSource(
                self.handle(),
                type_ as _,
                normal_open_or_close as _,
                device_id,
                timeout_ms,
            )
        }
        .into()
    }
    /**
     * Configures the reverse limit switch for a remote source.
     * For example, a CAN motor controller may need to monitor the Limit-R pin
     * of another Talon or CANifier.
     *
     * * `type_` - Remote limit switch source.
     *   User can choose between a remote Talon SRX, CANifier, or deactivate the feature.
     * * `normal_open_or_close` - Setting for normally open, normally closed, or disabled.
     *   This setting matches the web-based configuration drop down.
     * * `device_id` - Device ID of remote source (Talon SRX or CANifier device ID).
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    fn config_reverse_limit_switch_source(
        &mut self,
        type_: RemoteLimitSwitchSource,
        normal_open_or_close: LimitSwitchNormal,
        device_id: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigReverseLimitSwitchSource(
                self.handle(),
                type_ as _,
                normal_open_or_close as _,
                device_id,
                timeout_ms,
            )
        }
        .into()
    }
    fn override_limit_switches_enable(&mut self, enable: bool) {
        unsafe { c_MotController_OverrideLimitSwitchesEnable(self.handle(), enable) }.into()
    }

    fn config_forward_soft_limit_threshold(
        &mut self,
        forward_sensor_limit: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigForwardSoftLimitThreshold(
                self.handle(),
                forward_sensor_limit,
                timeout_ms,
            )
        }
        .into()
    }
    fn config_reverse_soft_limit_threshold(
        &mut self,
        reverse_sensor_limit: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigReverseSoftLimitThreshold(
                self.handle(),
                reverse_sensor_limit,
                timeout_ms,
            )
        }
        .into()
    }
    fn config_forward_soft_limit_enable(&mut self, enable: bool, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigForwardSoftLimitEnable(self.handle(), enable, timeout_ms) }
            .into()
    }
    fn config_reverse_soft_limit_enable(&mut self, enable: bool, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigReverseSoftLimitEnable(self.handle(), enable, timeout_ms) }
            .into()
    }
    fn override_soft_limits_enable(&mut self, enable: bool) {
        unsafe { c_MotController_OverrideSoftLimitsEnable(self.handle(), enable) }
    }

    // current limiting is Talon-specific

    fn config_kp(&mut self, slot_idx: i32, value: f64, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_Config_kP(self.handle(), slot_idx, value, timeout_ms) }.into()
    }
    fn config_ki(&mut self, slot_idx: i32, value: f64, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_Config_kI(self.handle(), slot_idx, value, timeout_ms) }.into()
    }
    fn config_kd(&mut self, slot_idx: i32, value: f64, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_Config_kD(self.handle(), slot_idx, value, timeout_ms) }.into()
    }
    fn config_kf(&mut self, slot_idx: i32, value: f64, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_Config_kF(self.handle(), slot_idx, value, timeout_ms) }.into()
    }
    fn config_integral_zone(&mut self, slot_idx: i32, izone: i32, timeout_ms: i32) -> Result<()> {
        unsafe {
            c_MotController_Config_IntegralZone(
                self.handle(),
                slot_idx,
                izone as f64, // idek both C++ and Java do this too
                timeout_ms,
            )
        }
        .into()
    }
    fn config_allowable_closedloop_error(
        &mut self,
        slot_idx: i32,
        allowable_closed_loop_error: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigAllowableClosedloopError(
                self.handle(),
                slot_idx,
                allowable_closed_loop_error,
                timeout_ms,
            )
        }
        .into()
    }
    fn config_max_integral_accumulator(
        &mut self,
        slot_idx: i32,
        iaccum: f64,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigMaxIntegralAccumulator(
                self.handle(),
                slot_idx,
                iaccum,
                timeout_ms,
            )
        }
        .into()
    }
    fn config_closed_loop_peak_output(
        &mut self,
        slot_idx: i32,
        percent_out: f64,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigClosedLoopPeakOutput(
                self.handle(),
                slot_idx,
                percent_out,
                timeout_ms,
            )
        }
        .into()
    }
    fn config_closed_loop_period(
        &mut self,
        slot_idx: i32,
        loop_time_ms: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigClosedLoopPeriod(
                self.handle(),
                slot_idx,
                loop_time_ms,
                timeout_ms,
            )
        }
        .into()
    }
    fn config_aux_pid_polarity(&mut self, invert: bool, timeout_ms: i32) -> Result<()> {
        self.config_set_parameter(
            ParamEnum::ePIDLoopPolarity,
            invert as i8 as f64,
            0,
            1,
            timeout_ms,
        )
    }
    fn set_integral_accumulator(&mut self, iaccum: f64, pid_idx: i32, timeout_ms: i32) -> Result<()> {
        unsafe {
            c_MotController_SetIntegralAccumulator(self.handle(), iaccum, pid_idx, timeout_ms)
        }
        .into()
    }
    fn get_closed_loop_error(&self, pid_idx: i32) -> Result<i32> {
        cci_get_call!(c_MotController_GetClosedLoopError(self.handle(), _: i32, pid_idx))
    }
    fn get_integral_accumulator(&self, pid_idx: i32) -> Result<f64> {
        cci_get_call!(c_MotController_GetIntegralAccumulator(self.handle(), _: f64, pid_idx))
    }
    /// Gets the derivative of the closed-loop error.
    fn get_error_derivative(&self, pid_idx: i32) -> Result<f64> {
        cci_get_call!(c_MotController_GetErrorDerivative(self.handle(), _: f64, pid_idx))
    }
    /// Selects which profile slot to use for closed-loop control.
    fn select_profile_slot(&mut self, slot_idx: i32, pid_idx: i32) -> Result<()> {
        unsafe { c_MotController_SelectProfileSlot(self.handle(), slot_idx, pid_idx) }.into()
    }
    fn get_closed_loop_target(&self, pid_idx: i32) -> Result<i32> {
        cci_get_call!(c_MotController_GetClosedLoopTarget(self.handle(), _: i32, pid_idx))
    }

    /// Gets the active trajectory target position using MotionMagic/MotionProfile control modes.
    fn get_active_trajectory_position(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetActiveTrajectoryPosition(self.handle(), _: i32))
    }
    /// Gets the active trajectory target velocity using MotionMagic/MotionProfile control modes.
    fn get_active_trajectory_velocity(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetActiveTrajectoryVelocity(self.handle(), _: i32))
    }
    /// Gets the active trajectory target heading using MotionMagic/MotionProfile control modes.
    fn get_active_trajectory_heading(&self) -> Result<f64> {
        cci_get_call!(c_MotController_GetActiveTrajectoryHeading(self.handle(), _: f64))
    }

    /// Sets the Motion Magic Cruise Velocity.
    /// This is the peak target velocity that the motion magic curve generator can use.
    fn config_motion_cruise_velocity(
        &mut self,
        sensor_units_per_100ms: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigMotionCruiseVelocity(
                self.handle(),
                sensor_units_per_100ms,
                timeout_ms,
            )
        }
        .into()
    }
    /// Sets the Motion Magic Acceleration.
    /// This is the target acceleration that the motion magic curve generator can use.
    fn config_motion_acceleration(
        &mut self,
        sensor_units_per_100ms_per_sec: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigMotionAcceleration(
                self.handle(),
                sensor_units_per_100ms_per_sec,
                timeout_ms,
            )
        }
        .into()
    }

    /// Clear the buffered motion profile in both motor controller's RAM (bottom),
    /// and in the API (top).
    fn clear_motion_profile_trajectories(&mut self) -> Result<()> {
        unsafe { c_MotController_ClearMotionProfileTrajectories(self.handle()) }.into()
    }
    /**
     * Retrieve just the buffer count for the api-level (top) buffer.
     * This routine performs no CAN or data structure lookups, so its fast and ideal
     * if caller needs to quickly poll the progress of trajectory points being
     * emptied into motor controller's RAM. Otherwise just use [`get_motion_profile_status`].
     *
     * [`get_motion_profile_status`]: #method.get_motion_profile_status
     */
    fn get_motion_profile_top_level_buffer_count(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetMotionProfileTopLevelBufferCount(self.handle(), _: i32))
    }
    /// Push another trajectory point into the top level buffer (which is emptied
    /// into the motor controller's bottom buffer as room allows).
    fn push_motion_profile_trajectory(&mut self, traj_pt: &TrajectoryPoint) -> Result<()> {
        unsafe {
            c_MotController_PushMotionProfileTrajectory_2(
                self.handle(),
                traj_pt.position,
                traj_pt.velocity,
                traj_pt.auxiliaryPos,
                traj_pt.profileSlotSelect0 as _, // wtf CTRE???
                traj_pt.profileSlotSelect1 as _,
                traj_pt.isLastPoint,
                traj_pt.zeroPos,
                traj_pt.timeDur as _,
            )
        }
        .into()
    }
    /**
     * Retrieve just the buffer full for the api-level (top) buffer.
     * This routine performs no CAN or data structure lookups, so its fast and ideal
     * if caller needs to quickly poll. Otherwise just use [`get_motion_profile_status`].
     *
     * [`get_motion_profile_status`]: #method.get_motion_profile_status
     */
    fn is_motion_profile_top_level_buffer_full(&self) -> Result<bool> {
        cci_get_call!(c_MotController_IsMotionProfileTopLevelBufferFull(self.handle(), _: bool))
    }
    /**
     * This must be called periodically to funnel the trajectory points from the
     * API's top level buffer to the controller's bottom level buffer.  Recommendation
     * is to call this twice as fast as the execution rate of the motion profile.
     * So if MP is running with 20ms trajectory points, try calling this routine
     * every 10ms.  All motion profile functions are thread-safe through the use of
     * a mutex, so there is no harm in having the caller utilize threading.
     */
    fn process_motion_profile_buffer(&mut self) {
        unsafe { c_MotController_ProcessMotionProfileBuffer(self.handle()) };
    }
    /**
     * Retrieve all status information.
     * For best performance, Caller can snapshot all status information regarding the
     * motion profile executer.
     */
    fn get_motion_profile_status(&self, status_to_fill: &mut MotionProfileStatus) -> ErrorCode {
        let mut output_enable: ::std::os::raw::c_int = 0;
        let code = unsafe {
            c_MotController_GetMotionProfileStatus_2(
                self.handle(),
                &mut status_to_fill.topBufferRem,
                &mut status_to_fill.topBufferCnt,
                &mut status_to_fill.btmBufferCnt,
                &mut status_to_fill.hasUnderrun,
                &mut status_to_fill.isUnderrun,
                &mut status_to_fill.activePointValid,
                &mut status_to_fill.isLast,
                &mut status_to_fill.profileSlotSelect0,
                &mut output_enable,
                &mut status_to_fill.timeDurMs,
                &mut status_to_fill.profileSlotSelect1,
            )
        };
        status_to_fill.outputEnable = output_enable.into();
        code
    }
    /// Get all motion profile status information.  This returns a new MotionProfileStatus.
    /// See `get_motion_profile_status`.
    fn get_new_motion_profile_status(&self) -> Result<MotionProfileStatus> {
        let mut status_to_fill: MotionProfileStatus = Default::default();
        let code = self.get_motion_profile_status(&mut status_to_fill);
        match code {
            ErrorCode::OK => Ok(status_to_fill),
            _ => Err(code),
        }
    }
    /// Clear the "Has Underrun" flag.
    /// Typically this is called after application has confirmed an underrun had occured.
    fn clear_motion_profile_has_underrun(&mut self, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ClearMotionProfileHasUnderrun(self.handle(), timeout_ms) }.into()
    }
    /**
     * Calling application can opt to speed up the handshaking between the robot API
     * and the controller to increase the download rate of the controller's Motion Profile.
     * Ideally the period should be no more than half the period of a trajectory
     * point.
     */
    fn change_motion_control_frame_period(&mut self, period_ms: i32) -> Result<()> {
        unsafe { c_MotController_ChangeMotionControlFramePeriod(self.handle(), period_ms) }.into()
    }
    /**
     * When trajectory points are processed in the motion profile executer, the MPE determines
     * how long to apply the active trajectory point by summing `base_traj_duration_ms` with the
     * `time_dur` of the trajectory point (see [`TrajectoryPoint`]).
     *
     * This allows general selection of the execution rate of the points with 1ms resolution,
     * while allowing some degree of change from point to point.
     *
     * * `base_traj_duration_ms` - The base duration time of every trajectory point.
     *   This is summed with the trajectory points unique `time_dur`.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     *
     * [`TrajectoryPoint`]: ../motion/struct.TrajectoryPoint.html
     */
    fn config_motion_profile_trajectory_period(
        &mut self,
        base_traj_duration_ms: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigMotionProfileTrajectoryPeriod(
                self.handle(),
                base_traj_duration_ms,
                timeout_ms,
            )
        }
        .into()
    }

    /**
     * Gets the last error generated by this object.
     * Not all functions return an error code but can potentially report errors.
     * This function can be used to retrieve those error codes.
     */
    fn get_last_error(&self) -> Result<()> {
        unsafe { c_MotController_GetLastError(self.handle()) }.into()
    }

    fn get_faults(&self) -> Result<Faults> {
        Ok(Faults(
            cci_get_call!(c_MotController_GetFaults(self.handle(), _: i32))?,
        ))
    }
    fn get_sticky_faults(&self) -> Result<StickyFaults> {
        Ok(StickyFaults(
            cci_get_call!(c_MotController_GetStickyFaults(self.handle(), _: i32))?,
        ))
    }
    fn clear_sticky_faults(&mut self, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ClearStickyFaults(self.handle(), timeout_ms) }.into()
    }

    /**
     * Gets the firmware version of the device.
     *
     * @return Firmware version of device.  For example: version 1-dot-2 is 0x0102.
     */
    fn get_firmware_version(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetFirmwareVersion(self.handle(), _: i32))
    }
    /// Returns true if the device has reset since last call.
    fn has_reset_occurred(&self) -> Result<bool> {
        cci_get_call!(c_MotController_HasResetOccurred(self.handle(), _: bool))
    }

    /**
     * Sets the value of a custom parameter. This is for arbitrary use.
     *
     * Sometimes it is necessary to save calibration/limit/target
     * information in the device. Particularly if the
     * device is part of a subsystem that can be replaced.
     *
     * * `new_value` - Value for custom parameter.
     * * `param_index` - Index of custom parameter [0,1]
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    fn config_set_custom_param(
        &mut self,
        new_value: i32,
        param_index: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigSetCustomParam(self.handle(), new_value, param_index, timeout_ms)
        }
        .into()
    }
    /**
     * Gets the value of a custom parameter.
     *
     * * `param_index` - Index of custom parameter [0,1].
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    fn config_get_custom_param(&self, param_index: i32, timout_ms: i32) -> Result<i32> {
        cci_get_call!(
            c_MotController_ConfigGetCustomParam(self.handle(), _: i32, param_index, timout_ms)
        )
    }

    /**
     * Sets a parameter. Generally this is not used.
     * This can be utilized in
     * - Using new features without updating API installation.
     * - Errata workarounds to circumvent API implementation.
     * - Allows for rapid testing / unit testing of firmware.
     */
    fn config_set_parameter(
        &mut self,
        param: ParamEnum,
        value: f64,
        sub_value: u8,
        ordinal: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigSetParameter(
                self.handle(),
                param as _,
                value,
                sub_value as _,
                ordinal,
                timeout_ms,
            )
        }
        .into()
    }
    fn config_get_parameter(&self, param: ParamEnum, ordinal: i32, timeout_ms: i32) -> Result<f64> {
        cci_get_call!(c_MotController_ConfigGetParameter(
            self.handle(),
            param as _,
            _: f64,
            ordinal,
            timeout_ms,
        ))
    }

    fn config_pulse_width_period_filter_window_sz(&mut self, pulse_width_period_filter_window_sz: i32, timeout_ms: i32) -> Result<()> {
        unsafe {
            c_MotController_ConfigPulseWidthPeriod_FilterWindowSz(self.handle(), pulse_width_period_filter_window_sz, timeout_ms)
        }.into()
    }

    fn config_pulse_width_period_edges_per_rot(&mut self, pulse_width_period_edges_per_rot: i32, timeout_ms: i32) -> Result<()> {
        unsafe {
            c_MotController_ConfigPulseWidthPeriod_EdgesPerRot(self.handle(), pulse_width_period_edges_per_rot, timeout_ms)
        }.into()
    }

    fn config_feedback_not_continuous(&mut self, feedback_not_continuous: bool, timeout_ms: i32) -> Result<()> {
        unsafe {
            c_MotController_ConfigFeedbackNotContinuous(self.handle(), feedback_not_continuous, timeout_ms)
        }.into()
    }

    /// Sets the number of velocity samples used in the rolling average velocity measurement.
    fn config_velocity_measurement_window(
        &mut self,
        window_size: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigVelocityMeasurementWindow(self.handle(), window_size, timeout_ms)
        }
        .into()
    }

    /**
     * Enables clearing the position of the feedback sensor when the forward
     * limit switch is triggered
     *
     * @param clearPositionOnLimitF     Whether clearing is enabled, defaults false
     * @param timeoutMs
     *            Timeout value in ms. If nonzero, function will wait for
     *            config success and report an error if it times out.
     *            If zero, no blocking or checking is performed.
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn config_clear_position_on_limit_f(&mut self,  clear_position_on_limit_f: bool,  timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigClearPositionOnLimitF(self.handle(), clear_position_on_limit_f, timeout_ms) }.into()
    }

    /**
     * Disables going to neutral (brake/coast) when a remote sensor is no longer detected.
     *
     * @param remoteSensorClosedLoopDisableNeutralOnLOS     disable going to neutral
     *
     * @param timeoutMs
     *            Timeout value in ms. If nonzero, function will wait for
     *            config success and report an error if it times out.
     *            If zero, no blocking or checking is performed.
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn config_remote_sensor_closed_loop_disable_neutral_on_los(&mut self,  remote_sensor_closed_loop_disable_neutral_on_los: bool,  timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigRemoteSensorClosedLoopDisableNeutralOnLOS(self.handle(), remote_sensor_closed_loop_disable_neutral_on_los, timeout_ms) }.into()
    }

    /**
     * Configures all filter persistant settings.
     *
     * @param filter        Object with all of the filter persistant settings
     * @param ordinal       0 for remote sensor 0 and 1 for remote sensor 1.
     * @param timeoutMs
     *              Timeout value in ms. If nonzero, function will wait for
     *              config success and report an error if it times out.
     *              If zero, no blocking or checking is performed.
     *
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn configure_filter(&mut self, filter: &config::FilterConfiguration, ordinal: i32, timeout_ms: i32) -> Result<()> {
        self.config_remote_feedback_filter(filter.remoteSensorDeviceID, filter.remoteSensorSource, ordinal, timeout_ms)
    }

    /**
     * Enables clearing the position of the feedback sensor when the reverse
     * limit switch is triggered
     *
     * @param clearPositionOnLimitR     Whether clearing is enabled, defaults false
     * @param timeoutMs
     *            Timeout value in ms. If nonzero, function will wait for
     *            config success and report an error if it times out.
     *            If zero, no blocking or checking is performed.
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn config_clear_position_on_limit_r(&mut self,  clear_position_on_limit_r: bool,  timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigClearPositionOnLimitR(self.handle(), clear_position_on_limit_r, timeout_ms) }.into()
    }

    /**
     * Configures all base PID set persistant settings.
     *
     * @param pid           Object with all of the base PID set persistant settings
     * @param pidIdx        0 for Primary closed-loop. 1 for auxiliary closed-loop.
     * @param timeoutMs
     *              Timeout value in ms. If nonzero, function will wait for
     *              config success and report an error if it times out.
     *              If zero, no blocking or checking is performed.
     *
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn base_configure_pid(&mut self, pid: &config::BasePIDSetConfig, pid_idx: i32, timeout_ms: i32) -> Result<()> {
        self.config_selected_feedback_coefficient(pid.selectedFeedbackCoefficient, pid_idx, timeout_ms)
    }

    /**
     * Disables soft limits triggering (if enabled) when the sensor is no longer detected.
     *
     * @param softLimitDisableNeutralOnLOS    disable triggering
     *
     * @param timeoutMs
     *            Timeout value in ms. If nonzero, function will wait for
     *            config success and report an error if it times out.
     *            If zero, no blocking or checking is performed.
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn config_soft_limit_disable_neutral_on_los(&mut self, soft_limit_disable_neutral_on_los: bool, timeout_ms: i32) -> Result<()> {
        unsafe {c_MotController_ConfigSoftLimitDisableNeutralOnLOS(self.handle(), soft_limit_disable_neutral_on_los, timeout_ms)}.into()
    }

    /**
     * Configures the period of each velocity sample.
     * Every 1ms a position value is sampled, and the delta between that sample
     * and the position sampled kPeriod ms ago is inserted into a filter.
     * kPeriod is configured with this function.
     */
    fn config_velocity_measurement_period(
        &mut self,
        period: VelocityMeasPeriod,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigVelocityMeasurementPeriod(self.handle(), period as _, timeout_ms)
        }
        .into()
    }

    /**
     * Disables limit switches triggering (if enabled) when the sensor is no longer detected.
     *
     * @param limitSwitchDisableNeutralOnLOS    disable triggering
     *
     * @param timeoutMs
     *            Timeout value in ms. If nonzero, function will wait for
     *            config success and report an error if it times out.
     *            If zero, no blocking or checking is performed.
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn config_limit_switch_disable_neutral_on_los(&mut self, limit_switch_disable_neutral_on_los: bool, timeout_ms: i32) -> Result<()> {
    unsafe { c_MotController_ConfigLimitSwitchDisableNeutralOnLOS(self.handle(), limit_switch_disable_neutral_on_los, timeout_ms)
    }.into()}

    /**
     * Enables clearing the position of the feedback sensor when the quadrature index signal
     * is detected
     *
     * @param clearPositionOnQuadidx    Whether clearing is enabled, defaults false
     * @param timeoutMs
     *            Timeout value in ms. If nonzero, function will wait for
     *            config success and report an error if it times out.
     *            If zero, no blocking or checking is performed.
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn config_clear_position_on_quad_idx(&mut self, clear_position_on_quad_idx: bool,  timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigClearPositionOnQuadIdx(self.handle(), clear_position_on_quad_idx, timeout_ms)}.into()
    }

    /**
     * Set the control mode and output value so that this motor controller will
     * follow another motor controller.
     * Currently supports following Victor SPX and Talon SRX.
     *
     * * `master_to_follow` - Motor Controller object to follow.
     * * `follower_type` - Type of following control.
     *   Use AuxOutput1 to follow the master device's auxiliary output 1.
     *   Use PercentOutput for standard follower mode.
     */
    fn follow(&mut self, master_to_follow: &impl MotorController, follower_type: FollowerType) {
        let base_id = master_to_follow.get_base_id();
        let id24: i32 = ((base_id >> 0x10) << 8) | (base_id & 0xFF);

        match follower_type {
            FollowerType::PercentOutput => {
                self.set(ControlMode::Follower, id24 as f64, DemandType::Neutral, 0.0)
            }
            FollowerType::AuxOutput1 => {
                self.set(ControlMode::Follower, id24 as f64, DemandType::AuxPID, 0.0)
            }
        };
    }

    fn config_factory_default(&mut self, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigFactoryDefault(self.handle(), timeout_ms) }.into()
    }

    /**
     * Configures all slot persistant settings.
     *
     * @param slot        Object with all of the slot persistant settings
     * @param slotIdx     Parameter slot for the constant.
     * @param timeoutMs
     *              Timeout value in ms. If nonzero, function will wait for
     *              config success and report an error if it times out.
     *              If zero, no blocking or checking is performed.
     *
     * @return Error Code generated by function. 0 indicates no error.
     */
    fn configure_slot(&mut self, slot: &config::SlotConfiguration, slot_idx: i32, timeout_ms: i32) -> Result<()> {

        //------ General Close loop ----------//
        self.config_kp(slot_idx, slot.kP, timeout_ms)?;
        self.config_ki(slot_idx, slot.kI, timeout_ms)?;
        self.config_kd(slot_idx, slot.kD, timeout_ms)?;
        self.config_kf(slot_idx, slot.kF, timeout_ms)?;
        self.config_integral_zone(slot_idx, slot.integralZone, timeout_ms)?;
        self.config_allowable_closedloop_error(slot_idx, slot.allowableClosedloopError, timeout_ms)?;
        self.config_max_integral_accumulator(slot_idx, slot.maxIntegralAccumulator, timeout_ms)?;
        self.config_closed_loop_peak_output(slot_idx, slot.closedLoopPeakOutput, timeout_ms)?;
        self.config_closed_loop_period(slot_idx, slot.closedLoopPeriod, timeout_ms)?;

        Ok(())
    }

    fn base_config_all(&mut self, config: &config::BaseMotorConfig, timeout_ms: i32) -> Result<()> {
        self.config_factory_default(timeout_ms)?;

        //----- general output shaping ------------------//
        self.config_openloop_ramp(config.openloopRamp, timeout_ms)?;
        self.config_closedloop_ramp(config.closedloopRamp, timeout_ms)?;
        self.config_peak_output_forward(config.peakOutputForward, timeout_ms)?;
        self.config_peak_output_reverse(config.peakOutputReverse, timeout_ms)?;
        self.config_nominal_output_forward(config.nominalOutputForward, timeout_ms)?;
        self.config_nominal_output_reverse(config.nominalOutputReverse, timeout_ms)?;
        self.config_neutral_deadband(config.neutralDeadband, timeout_ms)?;

        //------ Voltage Compensation ----------//
        self.config_voltage_comp_saturation(config.voltageCompSaturation, timeout_ms)?;
        self.config_voltage_measurement_filter(config.voltageMeasurementFilter, timeout_ms)?;

        //----- velocity signal conditionaing ------//
        self.config_velocity_measurement_period(config.velocityMeasurementPeriod, timeout_ms)?;
        self.config_velocity_measurement_window(config.velocityMeasurementWindow, timeout_ms)?;

        //------ soft limit ----------//
        self.config_forward_soft_limit_threshold(config.forwardSoftLimitThreshold, timeout_ms)?;
        self.config_reverse_soft_limit_threshold(config.reverseSoftLimitThreshold, timeout_ms)?;
        self.config_forward_soft_limit_enable(config.forwardSoftLimitEnable, timeout_ms)?;
        self.config_reverse_soft_limit_enable(config.reverseSoftLimitEnable, timeout_ms)?;

        //------ limit switch ----------//
        /* not in base */

        //------ Current Lim ----------//
        /* not in base */

        //--------Slots---------------//

        self.configure_slot(&config.slot_0, 0, timeout_ms)?;
        self.configure_slot(&config.slot_1, 1, timeout_ms)?;
        self.configure_slot(&config.slot_2, 2, timeout_ms)?;
        self.configure_slot(&config.slot_3, 3, timeout_ms)?;

        //---------Auxilary Closed Loop Polarity-------------//

        self.config_aux_pid_polarity(config.auxPIDPolarity, timeout_ms)?;

        //----------Remote Feedback Filters----------//
        self.configure_filter(&config.filter_0, 0, timeout_ms)?;
        self.configure_filter(&config.filter_1, 1, timeout_ms)?;

        //------ Motion Profile Settings used in Motion Magic  ----------//
        self.config_motion_cruise_velocity(config.motionCruiseVelocity, timeout_ms)?;
        self.config_motion_acceleration(config.motionAcceleration, timeout_ms)?;

        //------ Motion Profile Buffer ----------//
        self.config_motion_profile_trajectory_period(
            config.motionProfileTrajectoryPeriod,
            timeout_ms,
        )?;

        //------ Custom Persistent Params ----------//
        self.config_set_custom_param(config.custom.customParam_0, 0, timeout_ms)?;
        self.config_set_custom_param(config.custom.customParam_1, 1, timeout_ms)?;

        self.config_feedback_not_continuous(config.feedbackNotContinuous, timeout_ms)?;
        self.config_remote_sensor_closed_loop_disable_neutral_on_los(
            config.remoteSensorClosedLoopDisableNeutralOnLOS,
            timeout_ms,
        )?;
        self.config_clear_position_on_limit_f(config.clearPositionOnLimitF, timeout_ms)?;
        self.config_clear_position_on_limit_r(config.clearPositionOnLimitR, timeout_ms)?;
        self.config_clear_position_on_quad_idx(config.clearPositionOnQuadIdx, timeout_ms)?;
        self.config_limit_switch_disable_neutral_on_los(
            config.limitSwitchDisableNeutralOnLOS,
            timeout_ms,
        )?;
        self.config_soft_limit_disable_neutral_on_los(
            config.softLimitDisableNeutralOnLOS,
            timeout_ms,
        )?;
        self.config_pulse_width_period_edges_per_rot(
            config.pulseWidthPeriod_EdgesPerRot,
            timeout_ms,
        )?;
        self.config_pulse_width_period_filter_window_sz(
            config.pulseWidthPeriod_FilterWindowSz,
            timeout_ms,
        )?;

        Ok(())
    }
}

/// An interface for getting and setting raw sensor values.
pub trait SensorCollection: MotorController {
    fn get_analog_in(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetAnalogIn(self.handle(), _: i32))
    }
    fn set_analog_position(&mut self, new_position: i32, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_SetAnalogPosition(self.handle(), new_position, timeout_ms) }.into()
    }
    fn get_analog_in_raw(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetAnalogInRaw(self.handle(), _: i32))
    }
    fn get_analog_in_vel(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetAnalogInVel(self.handle(), _: i32))
    }
    fn get_quadrature_position(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetQuadraturePosition(self.handle(), _: i32))
    }
    fn set_quadrature_position(&mut self, new_position: i32, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_SetQuadraturePosition(self.handle(), new_position, timeout_ms) }
            .into()
    }
    fn get_quadrature_velocity(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetQuadratureVelocity(self.handle(), _: i32))
    }
    fn get_pulse_width_position(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetPulseWidthPosition(self.handle(), _: i32))
    }
    fn set_pulse_width_position(&mut self, new_position: i32, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_SetPulseWidthPosition(self.handle(), new_position, timeout_ms) }
            .into()
    }
    fn get_pulse_width_velocity(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetPulseWidthVelocity(self.handle(), _: i32))
    }
    fn get_pulse_width_rise_to_fall_us(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetPulseWidthRiseToFallUs(self.handle(), _: i32))
    }
    fn get_pulse_width_rise_to_rise_us(&self) -> Result<i32> {
        cci_get_call!(c_MotController_GetPulseWidthRiseToRiseUs(self.handle(), _: i32))
    }
    fn get_pin_state_quad_a(&self) -> Result<bool> {
        Ok(cci_get_call!(c_MotController_GetPinStateQuadA(self.handle(), _: i32))? != 0)
    }
    fn get_pin_state_quad_b(&self) -> Result<bool> {
        Ok(cci_get_call!(c_MotController_GetPinStateQuadB(self.handle(), _: i32))? != 0)
    }
    fn get_pin_state_quad_idx(&self) -> Result<bool> {
        Ok(cci_get_call!(c_MotController_GetPinStateQuadIdx(self.handle(), _: i32))? != 0)
    }
    fn is_fwd_limit_switch_closed(&self) -> Result<bool> {
        Ok(cci_get_call!(c_MotController_IsFwdLimitSwitchClosed(self.handle(), _: i32))? != 0)
    }
    fn is_rev_limit_switch_closed(&self) -> Result<bool> {
        Ok(cci_get_call!(c_MotController_IsRevLimitSwitchClosed(self.handle(), _: i32))? != 0)
    }
}

/// CTRE Talon SRX Motor Controller when used on CAN Bus.
#[derive(Debug)]
pub struct TalonSRX {
    handle: *mut c_void,
    arb_id: i32,
}

impl MotorController for TalonSRX {
    fn new(device_number: i32) -> TalonSRX {
        let arb_id = device_number | 0x02040000;
        let handle = unsafe { c_MotController_Create1(arb_id) };
        // kResourceType_CANTalonSRX
        #[cfg(feature = "usage-reporting")]
        report_usage(52, device_number as u32 + 1);
        TalonSRX { handle, arb_id }
    }

    #[doc(hidden)]
    fn handle(&self) -> *mut c_void {
        self.handle
    }
    fn get_base_id(&self) -> i32 {
        self.arb_id
    }
}

impl TalonSRX {
    /// Gets the output current of the motor controller in amps.
    pub fn get_output_current(&self) -> Result<f64> {
        cci_get_call!(c_MotController_GetOutputCurrent(self.handle, _: f64))
    }

    /**
     * Select the feedback device for the motor controller.
     *
     * * `feedback_device` - Feedback Device to select.
     * * `pid_idx` - 0 for Primary closed-loop. 1 for auxiliary closed-loop.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    pub fn config_selected_feedback_sensor(
        &mut self,
        feedback_device: FeedbackDevice,
        pid_idx: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigSelectedFeedbackSensor(
                self.handle,
                feedback_device as _,
                pid_idx,
                timeout_ms,
            )
        }
        .into()
    }

    // XXX: not provided by CTRE's APIs
    /*
    pub fn set_control_frame_period(
        &self,
        frame: ControlFrameEnhanced,
        period_ms: i32,
    ) -> Result<()> {
        unsafe { c_MotController_SetControlFramePeriod(self.handle, frame as _, period_ms) }
    }
    */
    pub fn set_status_frame_period(
        &mut self,
        frame: StatusFrameEnhanced,
        period_ms: u8,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_SetStatusFramePeriod(self.handle, frame as _, period_ms, timeout_ms)
        }
        .into()
    }
    pub fn get_status_frame_period(
        &self,
        frame: StatusFrameEnhanced,
        timeout_ms: i32,
    ) -> Result<i32> {
        cci_get_call!(
            c_MotController_GetStatusFramePeriod(self.handle, frame as _, _: i32, timeout_ms)
        )
    }

    /**
     * Configures the forward limit switch for a local/remote source.
     *
     * For example, a CAN motor controller may need to monitor the Limit-F pin
     * of another Talon, CANifier, or local Gadgeteer feedback connector.
     *
     * If the sensor is remote, a device ID of zero is assumed.
     * If that's not desired, use the four parameter version of this function.
     *
     * * `type_` - Limit switch source.
     *   User can choose between the feedback connector, remote Talon SRX, CANifier, or deactivate the feature.
     * * `normal_open_or_close` - Setting for normally open, normally closed, or disabled.
     *   This setting matches the web-based configuration drop down.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    pub fn config_forward_limit_switch_source(
        &mut self,
        type_: LimitSwitchSource,
        normal_open_or_close: LimitSwitchNormal,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigForwardLimitSwitchSource(
                self.handle,
                type_ as _,
                normal_open_or_close as _,
                0,
                timeout_ms,
            )
        }
        .into()
    }
    /**
     * Configures the reverse limit switch for a local/remote source.
     *
     * For example, a CAN motor controller may need to monitor the Limit-R pin
     * of another Talon, CANifier, or local Gadgeteer feedback connector.
     *
     * If the sensor is remote, a device ID of zero is assumed.
     * If that's not desired, use the four parameter version of this function.
     *
     * * `type_` - Limit switch source.
     *   User can choose between the feedback connector, remote Talon SRX, CANifier, or deactivate the feature.
     * * `normal_open_or_close` - Setting for normally open, normally closed, or disabled.
     *   This setting matches the web-based configuration drop down.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    pub fn config_reverse_limit_switch_source(
        &mut self,
        type_: LimitSwitchSource,
        normal_open_or_close: LimitSwitchNormal,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe {
            c_MotController_ConfigReverseLimitSwitchSource(
                self.handle,
                type_ as _,
                normal_open_or_close as _,
                0,
                timeout_ms,
            )
        }
        .into()
    }

    /**
     * Configure the peak allowable current (when current limit is enabled).
     *
     * Current limit is activated when current exceeds the peak limit for longer
     * than the peak duration. Then software will limit to the continuous limit.
     * This ensures current limiting while allowing for momentary excess current
     * events.
     *
     * For simpler current-limiting (single threshold) use
     * [`config_continuous_current_limit`] and set the peak to zero:
     * `config_peak_current_limit(0)`.
     *
     * * `amps` - Amperes to limit.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     *
     * [`config_continuous_current_limit`]: #method.config_continuous_current_limit
     */
    pub fn config_peak_current_limit(&mut self, amps: i32, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigPeakCurrentLimit(self.handle, amps, timeout_ms) }.into()
    }
    /**
     * Configure the peak allowable duration (when current limit is enabled).
     *
     * Current limit is activated when current exceeds the peak limit for longer
     * than the peak duration. Then software will limit to the continuous limit.
     * This ensures current limiting while allowing for momentary excess current
     * events.
     *
     * For simpler current-limiting (single threshold) use
     * [`config_continuous_current_limit`] and set the peak to zero:
     * `config_peak_current_limit(0)`.
     *
     * * `milliseconds` - How long to allow current-draw past peak limit.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     *
     * [`config_continuous_current_limit`]: #method.config_continuous_current_limit
     */
    pub fn config_peak_current_duration(
        &mut self,
        milliseconds: i32,
        timeout_ms: i32,
    ) -> Result<()> {
        unsafe { c_MotController_ConfigPeakCurrentLimit(self.handle, milliseconds, timeout_ms) }
            .into()
    }
    /**
     * Configure the continuous allowable current-draw (when current limit is enabled).
     *
     * Current limit is activated when current exceeds the peak limit for longer
     * than the peak duration. Then software will limit to the continuous limit.
     * This ensures current limiting while allowing for momentary excess current
     * events.
     *
     * For simpler current-limiting (single threshold) use
     * `config_continuous_current_limit()` and set the peak to zero:
     * `config_peak_current_limit(0)`.
     *
     * * `amps` - Amperes to limit.
     * * `timeout_ms` - Timeout value in ms.
     *   If nonzero, function will wait for config success and report an error if it times out.
     *   If zero, no blocking or checking is performed.
     */
    pub fn config_continuous_current_limit(&mut self, amps: i32, timeout_ms: i32) -> Result<()> {
        unsafe { c_MotController_ConfigContinuousCurrentLimit(self.handle, amps, timeout_ms) }
            .into_res()
    }
    pub fn enable_current_limit(&mut self, enable: bool) {
        unsafe { c_MotController_EnableCurrentLimit(self.handle, enable) };
    }

    /**
     * Gets all PID set persistant settings.
     *
     * @param pid               Object with all of the PID set persistant settings
     * @param pidIdx            0 for Primary closed-loop. 1 for auxiliary closed-loop.
     * @param timeoutMs
     *              Timeout value in ms. If nonzero, function will wait for
     *              config success and report an error if it times out.
     *              If zero, no blocking or checking is performed.
     */
    pub fn configure_pid(&mut self, pid: &config::TalonSRXPIDSetConfiguration,  pid_idx: i32, timeout_ms: i32) -> Result<()> {


        //------ sensor selection ----------//

        self.base_configure_pid(&pid.base, pid_idx, timeout_ms)?;
        self.config_selected_feedback_sensor(pid.selectedFeedbackSensor, pid_idx, timeout_ms)?;

        Ok(())
    }

    pub fn config_all(&mut self, config: &config::TalonSRXConfig, timeout_ms: i32) -> Result<()> {
        self.base_config_all(&config.base, timeout_ms)?;

        //------ limit switch ----------//
        unsafe {c_MotController_ConfigForwardLimitSwitchSource(self.handle, config.forwardLimitSwitchSource as _,
                config.base.forwardLimitSwitchNormal as _, config.base.forwardLimitSwitchDeviceID, timeout_ms) }.into_res()?;
        unsafe {c_MotController_ConfigReverseLimitSwitchSource(self.handle, config.reverseLimitSwitchSource as _,
                config.base.reverseLimitSwitchNormal as _, config.base.reverseLimitSwitchDeviceID, timeout_ms) }.into_res()?;


        //--------PIDs---------------//

        self.configure_pid(&config.primaryPID, 0, timeout_ms)?;
        self.configure_pid(&config.auxilaryPID, 1, timeout_ms)?;
        self.config_sensor_term(SensorTerm::Sum0, config.sum_0, timeout_ms)?;
        self.config_sensor_term(SensorTerm::Sum1, config.sum_1, timeout_ms)?;
        self.config_sensor_term(SensorTerm::Diff0, config.diff_0, timeout_ms)?;
        self.config_sensor_term(SensorTerm::Diff1, config.diff_1, timeout_ms)?;

        //--------Current Limiting-----//
        self.config_peak_current_limit(config.peakCurrentLimit, timeout_ms)?;
        self.config_peak_current_duration(config.peakCurrentDuration, timeout_ms)?;
        self.config_continuous_current_limit(config.continuousCurrentLimit, timeout_ms)?;


        Ok(())
    }
}

impl SensorCollection for TalonSRX {}

/// VEX Victor SPX Motor Controller when used on CAN Bus.
#[derive(Debug)]
pub struct VictorSPX {
    handle: *mut c_void,
    arb_id: i32,
}

impl MotorController for VictorSPX {
    fn new(device_number: i32) -> VictorSPX {
        let arb_id = device_number | 0x01040000;
        let handle = unsafe { c_MotController_Create1(arb_id) };
        // kResourceType_CTRE_future1
        #[cfg(feature = "usage-reporting")]
        report_usage(65, device_number as u32 + 1);
        VictorSPX { handle, arb_id }
    }

    #[doc(hidden)]
    fn handle(&self) -> *mut c_void {
        self.handle
    }
    fn get_base_id(&self) -> i32 {
        self.arb_id
    }
}

// Prevent users from implementing the MotorController trait.
mod private {
    pub trait Sealed {}
    impl Sealed for super::TalonSRX {}
    impl Sealed for super::VictorSPX {}
}
