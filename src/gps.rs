use messages_proc_macros_lib::common_derives;

// integrate ublox with the messaging library
#[common_derives]
pub struct NavPosLlh {
    pub height_msl: f64,
    pub longitude: f64,
    pub latitude: f64,
}

impl<'a> From<ublox::NavPosLlhRef<'a>> for NavPosLlh {
    fn from(nav_pos_llh: ublox::NavPosLlhRef<'a>) -> Self {
        NavPosLlh {
            height_msl: nav_pos_llh.height_msl(),
            longitude: nav_pos_llh.lat_degrees(),
            latitude: nav_pos_llh.lon_degrees(),
        }
    }
}

#[common_derives]
pub struct NavTimeUTC {
    /// Nanoseconds of second, range -1e9 .. 1e9
    pub nanos: i32,
    /// Year, range 1999..2099
    pub year: u16,
    /// Month, range 1..12
    pub month: u8,
    /// Day of Month, range 1..31
    pub day: u8,
    /// Hour of Day, range 0..23
    pub hour: u8,
    /// Minute of Hour, range 0..59
    pub min: u8,
    /// Seconds of Minute, range 0..59
    pub sec: u8,
}

impl<'a> From<ublox::NavTimeUTCRef<'a>> for NavTimeUTC {
    fn from(nav_time_utc: ublox::NavTimeUTCRef<'a>) -> Self {
        NavTimeUTC {
            nanos: nav_time_utc.nanos(),
            year: nav_time_utc.year(),
            month: nav_time_utc.month(),
            day: nav_time_utc.day(),
            hour: nav_time_utc.hour(),
            min: nav_time_utc.min(),
            sec: nav_time_utc.sec(),
        }
    }
}

#[common_derives]
pub enum Nav {
    NavPosLlh(NavPosLlh),
    NavTimeUTC(NavTimeUTC),
    Other,
}

impl<'a> From<ublox::PacketRef<'a>> for Nav {
    fn from(packet_ref: ublox::PacketRef<'a>) -> Self {
        match packet_ref {
            ublox::PacketRef::NavPosLlh(nav_pos_llh) => Nav::NavPosLlh(nav_pos_llh.into()),
            ublox::PacketRef::NavTimeUTC(nav_time_utc) => Nav::NavTimeUTC(nav_time_utc.into()),
            _ => Nav::Other,
        }
    }
}
