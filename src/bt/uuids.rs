use uuid::{uuid, Uuid};

pub const SRV_CURRENT_TIME: Uuid = uuid!("00001805-0000-1000-8000-00805f9b34fb");

pub const CHR_CURRENT_TIME: Uuid = uuid!("00002a2b-0000-1000-8000-00805f9b34fb");

pub const CHR_BATTERY_LEVEL: Uuid = uuid!("00002a19-0000-1000-8000-00805f9b34fb");
pub const CHR_FIRMWARE_REVISION: Uuid = uuid!("00002a26-0000-1000-8000-00805f9b34fb");
pub const CHR_HEART_RATE: Uuid = uuid!("00002a37-0000-1000-8000-00805f9b34fb");

pub const CHR_FWUPD_CONTROL_POINT: Uuid = uuid!("00001531-1212-efde-1523-785feabcd123");
pub const CHR_FWUPD_PACKET: Uuid = uuid!("00001532-1212-efde-1523-785feabcd123");

pub const CHR_MP_EVENTS: Uuid = uuid!("00000001-78fc-48fe-8e23-433b3a1942d0");
pub const CHR_MP_STATUS: Uuid = uuid!("00000002-78fc-48fe-8e23-433b3a1942d0");
pub const CHR_MP_ARTIST: Uuid = uuid!("00000003-78fc-48fe-8e23-433b3a1942d0");
pub const CHR_MP_TRACK: Uuid = uuid!("00000004-78fc-48fe-8e23-433b3a1942d0");
pub const CHR_MP_ALBUM: Uuid = uuid!("00000005-78fc-48fe-8e23-433b3a1942d0");
