use crate::Addresses;
use crate::frame::components::{FrameControl, MacAddress};

#[derive(Clone, Debug)]
pub struct NdpAnnouncement {
    pub frame_control: FrameControl,
    pub duration: [u8; 2],
    pub receiver: MacAddress,
    pub transmitter: MacAddress,
    pub sounding_token: [u8; 1],
    pub sta_info: [u8; 2],
}

impl Addresses for NdpAnnouncement {
    fn src(&self) -> Option<&MacAddress> {
        Some(&self.transmitter)
    }

    fn dest(&self) -> &MacAddress {
        &self.receiver
    }

    fn bssid(&self) -> Option<&MacAddress> {
        None
    }
}
