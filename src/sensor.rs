use crate::sensor_status::{
    AirStatus, EkfStatus, GpsPositionStatus, GpsVelStatus, ImuStatus, UtcTimeStatus,
};
use derive_more::From;
use messages_proc_macros_lib::common_derives;

#[common_derives]
pub struct AdcSensor {
    pub adc: u8, // ADC number
    pub positive_input: ads126x::register::PositiveInpMux,
    pub negative_input: ads126x::register::NegativeInpMux,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, From)]
pub enum Gps<'a> {
    #[serde(borrow)]
    NavPosLlh(ublox::NavPosLlhRef<'a>),
    #[serde(borrow)]
    NavStatus(ublox::NavStatusRef<'a>),
    #[serde(borrow)]
    NavDop(ublox::NavDopRef<'a>),
    #[serde(borrow)]
    NavPvt(ublox::NavPvtRef<'a>),
    #[serde(borrow)]
    NavSolution(ublox::NavSolutionRef<'a>),
    #[serde(borrow)]
    NavVelNed(ublox::NavVelNedRef<'a>),
    #[serde(borrow)]
    NavHpPosLlh(ublox::NavHpPosLlhRef<'a>),
    #[serde(borrow)]
    NavHpPosEcef(ublox::NavHpPosEcefRef<'a>),
    #[serde(borrow)]
    NavTimeUTC(ublox::NavTimeUTCRef<'a>),
    #[serde(borrow)]
    NavTimeLs(ublox::NavTimeLsRef<'a>),
    #[serde(borrow)]
    NavSat(ublox::NavSatRef<'a>),
    #[serde(borrow)]
    NavEoe(ublox::NavEoeRef<'a>),
    #[serde(borrow)]
    NavOdo(ublox::NavOdoRef<'a>),
    #[serde(borrow)]
    CfgOdo(ublox::CfgOdoRef<'a>),
    #[serde(borrow)]
    MgaAck(ublox::MgaAckRef<'a>),
    #[serde(borrow)]
    MgaGpsIono(ublox::MgaGpsIonoRef<'a>),
    #[serde(borrow)]
    MgaGpsEph(ublox::MgaGpsEphRef<'a>),
    #[serde(borrow)]
    MgaGloEph(ublox::MgaGloEphRef<'a>),
    #[serde(borrow)]
    AlpSrv(ublox::AlpSrvRef<'a>),
    #[serde(borrow)]
    AckAck(ublox::AckAckRef<'a>),
    #[serde(borrow)]
    AckNak(ublox::AckNakRef<'a>),
    #[serde(borrow)]
    CfgItfm(ublox::CfgItfmRef<'a>),
    #[serde(borrow)]
    CfgPrtI2c(ublox::CfgPrtI2cRef<'a>),
    #[serde(borrow)]
    CfgPrtSpi(ublox::CfgPrtSpiRef<'a>),
    #[serde(borrow)]
    CfgPrtUart(ublox::CfgPrtUartRef<'a>),
    #[serde(borrow)]
    CfgNav5(ublox::CfgNav5Ref<'a>),
    #[serde(borrow)]
    CfgAnt(ublox::CfgAntRef<'a>),
    #[serde(borrow)]
    CfgTmode2(ublox::CfgTmode2Ref<'a>),
    #[serde(borrow)]
    CfgTmode3(ublox::CfgTmode3Ref<'a>),
    #[serde(borrow)]
    CfgTp5(ublox::CfgTp5Ref<'a>),
    #[serde(borrow)]
    InfError(ublox::InfErrorRef<'a>),
    #[serde(borrow)]
    InfWarning(ublox::InfWarningRef<'a>),
    #[serde(borrow)]
    InfNotice(ublox::InfNoticeRef<'a>),
    #[serde(borrow)]
    InfTest(ublox::InfTestRef<'a>),
    #[serde(borrow)]
    InfDebug(ublox::InfDebugRef<'a>),
    #[serde(borrow)]
    RxmRawx(ublox::RxmRawxRef<'a>),
    #[serde(borrow)]
    TimTp(ublox::TimTpRef<'a>),
    #[serde(borrow)]
    TimTm2(ublox::TimTm2Ref<'a>),
    #[serde(borrow)]
    MonVer(ublox::MonVerRef<'a>),
    #[serde(borrow)]
    MonGnss(ublox::MonGnssRef<'a>),
    #[serde(borrow)]
    MonHw(ublox::MonHwRef<'a>),
    #[serde(borrow)]
    RxmRtcm(ublox::RxmRtcmRef<'a>),
    #[serde(borrow)]
    EsfMeas(ublox::EsfMeasRef<'a>),
    #[serde(borrow)]
    EsfIns(ublox::EsfInsRef<'a>),
    #[serde(borrow)]
    HnrAtt(ublox::HnrAttRef<'a>),
    #[serde(borrow)]
    HnrIns(ublox::HnrInsRef<'a>),
    #[serde(borrow)]
    HnrPvt(ublox::HnrPvtRef<'a>),
    #[serde(borrow)]
    NavAtt(ublox::NavAttRef<'a>),
    #[serde(borrow)]
    NavClock(ublox::NavClockRef<'a>),
    #[serde(borrow)]
    NavVelECEF(ublox::NavVelECEFRef<'a>),
    #[serde(borrow)]
    MgaGpsEPH(ublox::MgaGpsEPHRef<'a>),
    #[serde(borrow)]
    RxmSfrbx(ublox::RxmSfrbxRef<'a>),
    #[serde(borrow)]
    EsfRaw(ublox::EsfRawRef<'a>),
    #[serde(borrow)]
    TimSvin(ublox::TimSvinRef<'a>),
    #[serde(borrow)]
    SecUniqId(ublox::SecUniqIdRef<'a>),
}

#[common_derives]
#[derive(From)]
pub enum SbgData {
    UtcTime(UtcTime),
    Air(Air),
    EkfQuat(EkfQuat),
    EkfNav(EkfNav),
    Imu(Imu),
    GpsVel(GpsVel),
    GpsPos(GpsPos),
}

#[common_derives]
pub struct GpsPos {
    #[doc = "< Latitude in degrees, positive north."]
    pub latitude: Option<f64>,
    #[doc = "< Longitude in degrees, positive east."]
    pub longitude: Option<f64>,
    #[doc = "< GPS time of week in ms."]
    pub time_of_week: Option<u32>,
    #[doc = "< Altitude difference between the geoid and the Ellipsoid in meters (Height above Ellipsoid = altitude + undulation)."]
    pub undulation: Option<f32>,
    #[doc = "< Altitude above Mean Sea Level in meters."]
    pub altitude: Option<f64>,
    #[doc = "< Time in us since the sensor power up."]
    pub time_stamp: u32,
    #[doc = "< GPS position status, type and bitmask."]
    pub status: GpsPositionStatus,
    #[doc = "< 1 sigma latitude accuracy in meters."]
    pub latitude_accuracy: Option<f32>,
    #[doc = "< 1 sigma longitude accuracy in meters."]
    pub longitude_accuracy: Option<f32>,
    #[doc = "< 1 sigma altitude accuracy in meters."]
    pub altitude_accuracy: Option<f32>,
    #[doc = "< Number of space vehicles used to compute the solution (since version 1.4)."]
    pub num_sv_used: Option<u8>,
    #[doc = "< Base station id for differential corrections (0-4095). Set to 0xFFFF if differential corrections are not used (since version 1.4)."]
    pub base_station_id: Option<u16>,
    #[doc = "< Differential correction age in 0.01 seconds. Set to 0XFFFF if differential corrections are not used (since version 1.4)."]
    pub differential_age: Option<u16>,
}

#[common_derives]
pub struct UtcTime {
    #[doc = "< Time in us since the sensor power up."]
    pub time_stamp: u32,
    #[doc = "< UTC time and clock status information"]
    pub status: UtcTimeStatus,
    #[doc = "< Year for example: 2013."]
    pub year: Option<u16>,
    #[doc = "< Month in year [1 .. 12]."]
    pub month: Option<i8>,
    #[doc = "< Day in month [1 .. 31]."]
    pub day: Option<i8>,
    #[doc = "< Hour in day [0 .. 23]."]
    pub hour: Option<i8>,
    #[doc = "< Minute in hour [0 .. 59]."]
    pub minute: Option<i8>,
    #[doc = "< Second in minute [0 .. 60]. (60 is used only when a leap second is added)"]
    pub second: Option<i8>,
    #[doc = "< Nanosecond of current second in ns."]
    pub nano_second: Option<i32>,
    #[doc = "< GPS time of week in ms."]
    pub gps_time_of_week: Option<u32>,
}

#[common_derives]
pub struct Air {
    #[doc = "< Time in us since the sensor power up."]
    pub time_stamp: u32,
    #[doc = "< Airdata sensor status bitmask."]
    pub status: AirStatus,
    #[doc = "< Raw absolute pressure measured by the barometer sensor in Pascals."]
    pub pressure_abs: Option<f32>,
    #[doc = "< Altitude computed from barometric altimeter in meters and positive upward."]
    pub altitude: Option<f32>,
    #[doc = "< Raw differential pressure measured by the pitot tube in Pascal."]
    pub pressure_diff: Option<f32>,
    #[doc = "< True airspeed measured by a pitot tube in m.s^-1 and positive forward."]
    pub true_airspeed: Option<f32>,
    #[doc = "< Outside air temperature in °C that could be used to compute true airspeed from differential pressure."]
    pub air_temperature: Option<f32>,
}

#[common_derives]
pub struct EkfQuat {
    #[doc = "< Time in us since the sensor power up."]
    pub time_stamp: u32,
    #[doc = "< Orientation quaternion stored in W, X, Y, Z form."]
    pub quaternion: Option<[f32; 4usize]>,
    #[doc = "< Roll, Pitch and Yaw angles 1 sigma standard deviation in rad."]
    pub euler_std_dev: Option<[f32; 3usize]>,
    #[doc = "< EKF solution status bitmask and enum."]
    pub status: EkfStatus,
}

#[common_derives]
pub struct EkfNav {
    #[doc = "< EKF solution status bitmask and enum."]
    pub status: EkfStatus,
    #[doc = "< North, East, Down velocity 1 sigma standard deviation in m.s^-1."]
    pub velocity_std_dev: Option<[f32; 3usize]>,
    #[doc = "< Latitude, longitude and altitude 1 sigma standard deviation in meters."]
    pub position_std_dev: Option<[f32; 3usize]>,
    #[doc = "< Time in us since the sensor power up."]
    pub time_stamp: u32,
    #[doc = "< North, East, Down velocity in m.s^-1."]
    pub velocity: Option<[f32; 3usize]>,
    #[doc = "< Latitude, Longitude in degrees positive North and East.\nAltitude above Mean Sea Level in meters."]
    pub position: Option<[f64; 3usize]>,
    #[doc = "< Altitude difference between the geoid and the Ellipsoid in meters (Height above Ellipsoid = altitude + undulation)."]
    pub undulation: Option<f32>,
}

#[common_derives]
pub struct Imu {
    #[doc = "< Time in us since the sensor power up."]
    pub time_stamp: u32,
    #[doc = "< IMU status bitmask."]
    pub status: ImuStatus,
    #[doc = "< X, Y, Z accelerometers in m.s^-2."]
    pub accelerometers: Option<[f32; 3usize]>,
    #[doc = "< X, Y, Z gyroscopes in rad.s^-1."]
    pub gyroscopes: Option<[f32; 3usize]>,
    #[doc = "< Internal temperature in °C."]
    pub temperature: Option<f32>,
    #[doc = "< X, Y, Z delta velocity in m.s^-2."]
    pub delta_velocity: Option<[f32; 3usize]>,
    #[doc = "< X, Y, Z delta angle in rad.s^-1."]
    pub delta_angle: Option<[f32; 3usize]>,
}

#[common_derives]
pub struct GpsVel {
    #[doc = "< GPS time of week in ms."]
    pub time_of_week: Option<u32>,
    #[doc = "< Time in us since the sensor power up."]
    pub time_stamp: u32,
    #[doc = "< GPS velocity status, type and bitmask."]
    pub status: GpsVelStatus,
    #[doc = "< GPS North, East, Down velocity in m.s^-1."]
    pub velocity: Option<[f32; 3usize]>,
    #[doc = "< Track ground course in degrees."]
    pub course: Option<f32>,
    #[doc = "< Course accuracy in degrees."]
    pub course_acc: Option<f32>,
    #[doc = "< GPS North, East, Down velocity 1 sigma accuracy in m.s^-1."]
    pub velocity_acc: Option<[f32; 3usize]>,
}

#[common_derives]
pub struct RecoverySensing {
    pub drogue_current: u16,
    pub main_current: u16,
    pub drogue_voltage: u16,
    pub main_voltage: u16,
}
