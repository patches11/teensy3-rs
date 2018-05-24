//! Safe bindings for OctoWS2811 Teensy library

use bindings;

/// LED Color Order
#[derive(Debug, PartialEq)]
pub enum ColorOrder {
    Ws2811Rgb, // The WS2811 datasheet documents this way
    Ws2811Rbg,
    Ws2811Grb, // Most LED strips are wired this way
    Ws2811Gbr,
    Ws2811Brg,
    Ws2811Bgr,
}

/// LED Rate & Type
#[derive(Debug, PartialEq)]
pub enum TypeRate {
    Ws2811r800kHz, // Nearly all WS2811 are 800 kHz
    Ws2811r400kHz, // Adafruit's Flora Pixels
    Ws2813r800kHz, // WS2813 are close to 800 kHz but has 300 us frame set delay
}

#[derive(Copy, Clone)]
pub struct OctoWS2811 {
    c_obj: bindings::OctoWS2811,
    pub max: u32
}

impl OctoWS2811 {

    /// Create OctoWS2811 object. You can only create a single object
    pub fn new(leds_per_string: u32, display_memory: *mut [u32], drawing_memory: *mut [u32], color_order: ColorOrder, type_rate: TypeRate) -> OctoWS2811 {

        let color: u32 = match color_order {
            ColorOrder::Ws2811Rgb => bindings::WS2811_RGB,
            ColorOrder::Ws2811Rbg => bindings::WS2811_RBG,
            ColorOrder::Ws2811Grb => bindings::WS2811_GRB,
            ColorOrder::Ws2811Gbr => bindings::WS2811_GBR,
            ColorOrder::Ws2811Brg => bindings::WS2811_BRG,
            ColorOrder::Ws2811Bgr => bindings::WS2811_BGR,
        };

        let type_r: u32 = match type_rate {
            TypeRate::Ws2811r800kHz => bindings::WS2811_800kHz,
            TypeRate::Ws2811r400kHz => bindings::WS2811_400kHz,
            TypeRate::Ws2813r800kHz => bindings::WS2813_800kHz,
        };

        let config: u32 = color | type_r;

        OctoWS2811 {
            c_obj: unsafe {
                bindings::OctoWS2811::new(
                    leds_per_string,
                    display_memory as *mut bindings::c_types::c_void,
                    drawing_memory as *mut bindings::c_types::c_void,
                    config as u8
                )
            },
            max: leds_per_string * 8
        }
    }

    /// Initialize the library. This must be called before using setPixel or show.
    pub fn begin(&mut self) {
        unsafe {
            self.c_obj.begin()
        }
    }

    /// Set a pixel. Red, green and blue are 0 to 255.
    pub fn set_pixel(&mut self, led: u32, red: u8, green: u8, blue: u8) {
        unsafe {
            self.c_obj.setPixel1(led, red, green, blue)
        }
    }

    /// Start an update of the LEDs. This function returns within 2 microseconds. The display update continues, taking 30 microseconds for for each LED, plus 50 microseconds to reset the WS2811.
    /// If called while a previous update is running, this function waits for previous update to complete and then starts a new update.
    pub fn show(&mut self) {
        unsafe {
            self.c_obj.show()
        }
    }

    /// Check if a previous show() is still running. Returns true if the WS2811 LEDs are busy being updated, or false if no update is in progress.
    pub fn busy(&mut self) -> bool {
        unsafe {
            if self.c_obj.busy() == 1 {
                true
            } else {
                false
            }
        }
    }

    /// Read a pixel's color. The return is a 24 bit color number.
    pub fn get_pixel(&mut self, led: u32) -> i32 {
        unsafe {
            self.c_obj.getPixel(led)
        }
    }

    /*
    leds.setPixel(num, color);
    Set a pixel. Color is a 24 bit color in RGB order (the same as html).
    */

}

