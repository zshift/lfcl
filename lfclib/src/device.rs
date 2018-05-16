use consts;
use hidapi::{HidResult, HidApi, HidDevice};

pub struct LuxaforDeviceDescriptor {
    pub vendor_id  : u16,
    pub product_id : u16
}

pub struct LuxaforContext {
    hid_api : HidApi
}

pub struct LuxaforDevice<'a> {
    hid_device : HidDevice<'a>
}

impl LuxaforContext {
    pub fn new() -> HidResult<LuxaforContext> {
        Ok(LuxaforContext {
            hid_api: HidApi::new()?
        })
    }

    pub fn open_device(&self, device_descriptor: LuxaforDeviceDescriptor) -> HidResult<LuxaforDevice> {
        LuxaforDevice::new(self.hid_api.open(device_descriptor.vendor_id, device_descriptor.product_id)?)
    }
}

impl<'a> LuxaforDevice<'a> {
    pub fn new(device: HidDevice) -> HidResult<LuxaforDevice> {
        Ok(LuxaforDevice {
            hid_device: device
        })
    }

    // solid
    pub fn solid(self, r: u8, g: u8, b: u8, led: u8) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::STATIC, led, r, g, b])
    }

    // last number is the timing of the fade. bigger == longer. 
    pub fn fade(self, r: u8, g: u8, b: u8, led: u8) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::FADE, led, r, g, b, 255])
    }

    // 1-7 works, but 0 and 10 don't. Not sure how the other patterns are indexed yet. 
    pub fn pattern(self) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::PATTERN, 7])
    }

    // not sure if there are more options, (eg strobe timing) but it works
    pub fn strobe(self, r: u8, g: u8, b: u8, led: u8) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::STROBE, led, r, g, b])
    }

    // not working yet
    pub fn wave(self, r: u8, g: u8, b: u8, led: u8) -> HidResult<usize> {
        self.hid_device.write(&[consts::mode::WAVE, led, r, g, b])
    }
}
