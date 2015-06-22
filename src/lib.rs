#![feature(plugin, slice_position_elem)]
#![plugin(fourcc)]

//! See [Toupcam](struct.Toupcam.html).

extern crate libc;
#[macro_use]
extern crate bitflags;

use std::str;
use std::ffi::CStr;
use std::ptr::{null, null_mut};
use libc::{c_void, c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_double};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[must_use]
#[allow(overflowing_literals, non_camel_case_types, dead_code)]
enum HRESULT {
    S_OK         = 0x00000000, /* Operation successful */
    S_FALSE      = 0x00000001, /* Operation successful */
    E_FAIL       = 0x80004005 as i32, /* Unspecified failure */
    E_INVALIDARG = 0x80070057 as i32, /* One or more arguments are not valid */
    E_NOTIMPL    = 0x80004001 as i32, /* Not supported or not implemented */
    E_POINTER    = 0x80004003 as i32, /* Pointer that is not valid */
    E_UNEXPECTED = 0x8000FFFF as i32, /* Unexpected failure */
}

bitflags! {
    flags Flags: u32 {
        const FLAG_CMOS               = 0x00000001,  /* cmos sensor */
        const FLAG_CCD_PROGRESSIVE    = 0x00000002,  /* progressive ccd sensor */
        const FLAG_CCD_INTERLACED     = 0x00000004,  /* interlaced ccd sensor */
        const FLAG_ROI_HARDWARE       = 0x00000008,  /* support hardware ROI */
        const FLAG_MONO               = 0x00000010,  /* monochromatic */
        const FLAG_BINSKIP_SUPPORTED  = 0x00000020,  /* support bin/skip mode, see Toupcam_put_Mode and Toupcam_get_Mode */
        const FLAG_USB30              = 0x00000040,  /* USB 3.0 */
        const FLAG_COOLED             = 0x00000080,  /* Cooled */
        const FLAG_USB30_OVER_USB20   = 0x00000100,  /* usb3.0 camera connected to usb2.0 port */
        const FLAG_ST4                = 0x00000200,  /* ST4 */
        const FLAG_GETTEMPERATURE     = 0x00000400,  /* support to get the temperature of sensor */
        const FLAG_PUTTEMPERATURE     = 0x00000800,  /* support to put the temperature of sensor */
        const FLAG_BITDEPTH10         = 0x00001000,  /* Maximum Bit Depth = 10 */
        const FLAG_BITDEPTH12         = 0x00002000,  /* Maximum Bit Depth = 12 */
        const FLAG_BITDEPTH14         = 0x00004000,  /* Maximum Bit Depth = 14 */
        const FLAG_BITDEPTH16         = 0x00008000,  /* Maximum Bit Depth = 16 */
        const FLAG_FAN                = 0x00010000,  /* cooling fan */
        const FLAG_COOLERONOFF        = 0x00020000,  /* cooler can be turn on or off */
        const FLAG_ISP                = 0x00040000,  /* image signal processing supported */
        const FLAG_TRIGGER            = 0x00080000,  /* support the trigger mode */
    }
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Event {
    Exposure        = 0x0001, /* exposure time changed */
    TempTint        = 0x0002, /* white balance changed, Temp/Tint mode */
    Chrome          = 0x0003, /* reversed, do not use it */
    Image           = 0x0004, /* live image arrived, use Toupcam_PullImage to get this image */
    StillImage      = 0x0005, /* snap (still) frame arrived, use Toupcam_PullStillImage to get this frame */
    WBGain          = 0x0006, /* white balance changed, RGB Gain mode */
    Error           = 0x0080, /* something error happens */
    Disconnected    = 0x0081  /* camera disconnected */
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(dead_code)]
enum Option {
    NoFrameTimeout  = 0x01,    /* iValue: 1 = enable; 0 = disable. default: enable */
    ThreadPriority  = 0x02,    /* set the priority of the internal thread which grab data from the usb device. iValue: 0 = THREAD_PRIORITY_NORMAL; 1 = THREAD_PRIORITY_ABOVE_NORMAL; 2 = THREAD_PRIORITY_HIGHEST; default: 0; see: msdn SetThreadPriority */
    ProcessMode     = 0x03,    /* 0 = better image quality, more cpu usage. this is the default value
                                         1 = lower image quality, less cpu usage */
    Raw             = 0x04,    /* raw mode, read the sensor data. This can be set only BEFORE Toupcam_StartXXX() */
    Histogram       = 0x05,    /* 0 = only one, 1 = continue mode */
    BitDepth        = 0x06,    /* 0 = 8bits mode, 1 = 16bits mode */
    Fan             = 0x07,    /* 0 = turn off the cooling fan, 1 = turn on the cooling fan */
    Cooler          = 0x08,    /* 0 = turn off cooler, 1 = turn on cooler */
    Linear          = 0x09,    /* 0 = turn off tone linear, 1 = turn on tone linear */
    Curve           = 0x0a,    /* 0 = turn off tone curve, 1 = turn on tone curve */
    Trigger         = 0x0b,    /* 0 = continuous mode, 1 = trigger mode, default value = 0 */
    RGB48           = 0x0c     /* enable RGB48 format when bitdepth > 8 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Resolution {
    pub width           : u32,
    pub height          : u32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Image {
    pub resolution      : Resolution,
    pub bits            : u32,
    pub data            : Vec<u8>,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Layout {
        GBRG            = fourcc!("GBRG"),
        RGGB            = fourcc!("RGGB"),
        BGGR            = fourcc!("BGGR"),
        GRBG            = fourcc!("GRBG"),
        YUYV            = fourcc!("YUYV"),
        YYYY            = fourcc!("YYYY"),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Format {
    pub fourcc          : Layout,
    pub bit_depth       : u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Rect {
    pub left            : u32,
    pub top             : u32,
    pub right           : u32,
    pub bottom          : u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Range<T> {
    pub minimum         : T,
    pub maximum         : T,
    pub default         : T,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Flicker {
        AC60Hz          = 0,
        AC50Hz          = 1,
        DC              = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum SamplingMode {
        Bin             = 0,
        Skip            = 1,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TempTint  {
    pub temperature     : u32,
    pub tint            : u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct WhiteBalanceTempTint  {
    pub temperature     : u32,
    pub tint            : u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct WhiteBalanceRGB {
    pub red             : i32,
    pub green           : i32,
    pub blue            : i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct LevelRanges {
    pub red             : (u16, u16),
    pub green           : (u16, u16),
    pub blue            : (u16, u16),
    pub gray            : (u16, u16),
}

#[repr(u16)]
pub enum LEDState {
        Off             = 0,
        On              = 1,
        Flashing        = 2,
}

#[repr(C)]
struct ModelInternal {
        name            : *const c_char,
        flags           : u32,
        maxspeed        : c_uint,
        preview         : c_uint,
        still           : c_uint,
        res             : [Resolution; 16],
}

#[derive(Clone, Debug)]
pub struct Model
{
    pub name            : String,
    pub flags           : Flags,
    pub maximum_speed   : u32,
    pub preview_resolutions : Vec<Resolution>,
    pub still_resolutions   : Vec<Resolution>,
}

#[repr(C)]
struct InstanceInternal {
        displayname     : [c_char; 64],
        id              : [c_char; 64],
        model           : *const ModelInternal,
}

#[derive(Clone, Debug)]
pub struct Instance
{
    pub display_name    : String, /* display name */
    pub unique_id       : String, /* unique and opaque id of a connected camera */
    pub model           : Model,
}

#[repr(C)]
struct Handle;

/* FFI functions */

#[link(name = "toupcam")]
#[allow(dead_code)]
extern {
    fn Toupcam_Version() -> *const c_char;

    fn Toupcam_HotPlug(pHotPlugCallback: extern fn(*mut c_void), pCallbackCtx: *mut c_void);

    fn Toupcam_Enum(pti: *mut [InstanceInternal; 16]) -> c_uint;
    fn Toupcam_Open(id: *const c_char) -> *mut Handle;
    fn Toupcam_Close(h: *mut Handle);

    fn Toupcam_get_SerialNumber(h: *mut Handle, sn: *mut [c_char; 32]) -> HRESULT;
    fn Toupcam_get_ProductionDate(h: *mut Handle, pdate: *mut [c_char; 10]) -> HRESULT;

    fn Toupcam_StartPullModeWithCallback(h: *mut Handle,
                                         pEventCallback: extern fn(Event, *mut c_void),
                                         pCallbackCtx: *mut c_void) -> HRESULT;
    fn Toupcam_PullImage(h: *mut Handle, pImageData: *mut u8, bits: c_int,
                         pnWidth: *mut c_uint, pnHeight: *mut c_uint) -> HRESULT;
    fn Toupcam_PullStillImage(h: *mut Handle, pImageData: *mut u8, bits: c_int,
                              pnWidth: *mut c_uint, pnHeight: *mut c_uint) -> HRESULT;
    fn Toupcam_Stop(h: *mut Handle) -> HRESULT;
    fn Toupcam_Pause(h: *mut Handle, bPause: c_int) -> HRESULT;

    fn Toupcam_get_FwVersion(h: *mut Handle, fwver: *mut [c_char; 16]) -> HRESULT;
    fn Toupcam_get_HwVersion(h: *mut Handle, hwver: *mut [c_char; 16]) -> HRESULT;
    fn Toupcam_get_MaxSpeed(h: *mut Handle) -> HRESULT;
    fn Toupcam_get_MaxBitDepth(h: *mut Handle) -> HRESULT;

    fn Toupcam_get_RawFormat(h: *mut Handle,
                             nFourCC: *mut Layout, bitdepth: *mut c_uint) -> HRESULT;

    fn Toupcam_get_ResolutionNumber(h: *mut Handle) -> HRESULT;
    fn Toupcam_get_Resolution(h: *mut Handle, nResolutionIndex: c_uint,
                              pWidth: *mut c_int, pHeight: *mut c_int) -> HRESULT;
    fn Toupcam_get_ResolutionRatio(h: *mut Handle, nResolutionIndex: c_uint,
                                   pNumerator: *mut c_int, pDenominator: *mut c_int) -> HRESULT;

    fn Toupcam_put_Size(h: *mut Handle, nWidth: c_int, nHeight: c_int) -> HRESULT;
    fn Toupcam_get_Size(h: *mut Handle, nWidth: *mut c_int, nHeight: *mut c_int) -> HRESULT;
    fn Toupcam_put_eSize(h: *mut Handle, nResolutionIndex: c_uint) -> HRESULT;
    fn Toupcam_get_eSize(h: *mut Handle, nResolutionIndex: *mut c_uint) -> HRESULT;

    fn Toupcam_Trigger(h: *mut Handle) -> HRESULT;

    fn Toupcam_get_StillResolutionNumber(h: *mut Handle) -> HRESULT;
    fn Toupcam_get_StillResolution(h: *mut Handle, nIndex: c_uint,
                                   pWidth: *mut c_int, pHeight: *mut c_int) -> HRESULT;

    fn Toupcam_Snap(h: *mut Handle, nResolutionIndex: c_uint) -> HRESULT;

    fn Toupcam_put_RealTime(h: *mut Handle, bEnable: c_int) -> HRESULT;
    fn Toupcam_get_RealTime(h: *mut Handle, bEnable: *mut c_int) -> HRESULT;

    fn Toupcam_get_Temperature(h: *mut Handle, pTemperature: *mut c_short) -> HRESULT;
    fn Toupcam_put_Temperature(h: *mut Handle, nTemperature: c_short) -> HRESULT;

    fn Toupcam_get_Roi(h: *mut Handle, pxOffset: *mut c_uint, pyOffset: *mut c_uint,
                       pxWidth: *mut c_uint, pyHeight: *mut c_uint) -> HRESULT;
    fn Toupcam_put_Roi(h: *mut Handle, xOffset: c_uint, yOffset: c_uint,
                       xWidth: c_uint, yHeight: c_uint) -> HRESULT;

    fn Toupcam_get_AutoExpoEnable(h: *mut Handle, bAutoExposure: *mut c_int) -> HRESULT;
    fn Toupcam_put_AutoExpoEnable(h: *mut Handle, bAutoExposure: c_int) -> HRESULT;
    fn Toupcam_get_AutoExpoTarget(h: *mut Handle, Target: *mut c_ushort) -> HRESULT;
    fn Toupcam_put_AutoExpoTarget(h: *mut Handle, Target: c_ushort) -> HRESULT;
    fn Toupcam_put_MaxAutoExpoTimeAGain(h: *mut Handle,
                                        maxTime: c_uint, maxAGain: c_ushort) -> HRESULT;

    fn Toupcam_get_ExpoTime(h: *mut Handle, Time: *mut c_uint) -> HRESULT;
    fn Toupcam_put_ExpoTime(h: *mut Handle, Time: c_uint) -> HRESULT;
    fn Toupcam_get_ExpTimeRange(h: *mut Handle,
                                nMin: *mut c_uint, nMax: *mut c_uint,
                                nDef: *mut c_uint) -> HRESULT;

    fn Toupcam_get_ExpoAGain(h: *mut Handle, AGain: *mut c_ushort) -> HRESULT;
    fn Toupcam_put_ExpoAGain(h: *mut Handle, AGain: c_ushort) -> HRESULT;
    fn Toupcam_get_ExpoAGainRange(h: *mut Handle,
                                  nMin: *mut c_ushort, nMax: *mut c_ushort,
                                  nDef: *mut c_ushort) -> HRESULT;

    fn Toupcam_put_AEAuxRect(h: *mut Handle, pAuxRect: *const Rect) -> HRESULT;
    fn Toupcam_get_AEAuxRect(h: *mut Handle, pAuxRect: *mut Rect) -> HRESULT;

    fn Toupcam_put_Hue(h: *mut Handle, Hue: c_int) -> HRESULT;
    fn Toupcam_get_Hue(h: *mut Handle, Hue: *mut c_int) -> HRESULT;
    fn Toupcam_put_Saturation(h: *mut Handle, Saturation: c_int) -> HRESULT;
    fn Toupcam_get_Saturation(h: *mut Handle, Saturation: *mut c_int) -> HRESULT;
    fn Toupcam_put_Brightness(h: *mut Handle, Brightness: c_int) -> HRESULT;
    fn Toupcam_get_Brightness(h: *mut Handle, Brightness: *mut c_int) -> HRESULT;
    fn Toupcam_get_Contrast(h: *mut Handle, Contrast: *mut c_int) -> HRESULT;
    fn Toupcam_put_Contrast(h: *mut Handle, Contrast: c_int) -> HRESULT;
    fn Toupcam_get_Gamma(h: *mut Handle, Gamma: *mut c_int) -> HRESULT;
    fn Toupcam_put_Gamma(h: *mut Handle, Gamma: c_int) -> HRESULT;

    fn Toupcam_get_Chrome(h: *mut Handle, bChrome: *mut c_int) -> HRESULT;
    fn Toupcam_put_Chrome(h: *mut Handle, bChrome: c_int) -> HRESULT;

    fn Toupcam_get_MonoMode(h: *mut Handle) -> HRESULT;

    fn Toupcam_get_VFlip(h: *mut Handle, bVFlip: *mut c_int) -> HRESULT;
    fn Toupcam_put_VFlip(h: *mut Handle, bVFlip: c_int) -> HRESULT;
    fn Toupcam_get_HFlip(h: *mut Handle, bHFlip: *mut c_int) -> HRESULT;
    fn Toupcam_put_HFlip(h: *mut Handle, bHFlip: c_int) -> HRESULT;

    fn Toupcam_get_Negative(h: *mut Handle, bNegative: *mut c_int) -> HRESULT;
    fn Toupcam_put_Negative(h: *mut Handle, bNegative: c_int) -> HRESULT;

    fn Toupcam_put_Speed(h: *mut Handle, nSpeed: c_ushort) -> HRESULT;
    fn Toupcam_get_Speed(h: *mut Handle, pSpeed: *mut c_ushort) -> HRESULT;

    fn Toupcam_put_HZ(h: *mut Handle, nHZ: Flicker) -> HRESULT;
    fn Toupcam_get_HZ(h: *mut Handle, nHZ: *mut Flicker) -> HRESULT;

    fn Toupcam_put_Mode(h: *mut Handle, bSkip: SamplingMode) -> HRESULT;
    fn Toupcam_get_Mode(h: *mut Handle, bSkip: *mut SamplingMode) -> HRESULT;

    fn Toupcam_put_TempTint(h: *mut Handle, nTemp: c_int, nTint: c_int) -> HRESULT;
    fn Toupcam_get_TempTint(h: *mut Handle, nTemp: *mut c_int, nTint: *mut c_int) -> HRESULT;
    fn Toupcam_AwbOnePush(h: *mut Handle,
                          fnTTProc: std::option::Option<extern fn(c_int, c_int, *mut c_void)>,
                          pTTCtx: *mut c_void) -> HRESULT;

    fn Toupcam_put_WhiteBalanceGain(h: *mut Handle, aGain: *const [c_int; 3]) -> HRESULT;
    fn Toupcam_get_WhiteBalanceGain(h: *mut Handle, aGain: *mut [c_int; 3]) -> HRESULT;
    fn Toupcam_AwbInit(h: *mut Handle,
                       fnWBProc: std::option::Option<extern fn(*const [c_int; 3], *mut c_void)>,
                       pWBCtx: *mut c_void) -> HRESULT;

    fn Toupcam_put_AWBAuxRect(h: *mut Handle, pAuxRect: *const Rect) -> HRESULT;
    fn Toupcam_get_AWBAuxRect(h: *mut Handle, pAuxRect: *mut Rect) -> HRESULT;

    fn Toupcam_put_LevelRange(h: *mut Handle,
                              aLow: *const [c_ushort; 4], aHigh: *const [c_ushort; 4]) -> HRESULT;
    fn Toupcam_get_LevelRange(h: *mut Handle,
                              aLow: *mut [c_ushort; 4], aHigh: *mut [c_ushort; 4]) -> HRESULT;
    fn Toupcam_LevelRangeAuto(h: *mut Handle) -> HRESULT;

    fn Toupcam_put_ExpoCallback(h: *mut Handle,
                                fnExpoProc: std::option::Option<extern fn(*mut c_void)>,
                                pExpoCtx: *mut c_void) -> HRESULT;
    fn Toupcam_put_ChromeCallback(h: *mut Handle,
                                  fnChromeProc: std::option::Option<extern fn(*mut c_void)>,
                                  pChromeCtx: *mut c_void) -> HRESULT;

    fn Toupcam_put_LEDState(h: *mut Handle, iLed: c_ushort,
                            iState: LEDState, iPeriod: c_ushort) -> HRESULT;

    fn Toupcam_write_EEPROM(h: *mut Handle, addr: c_uint,
                            pData: *const u8, nDataLen: c_uint) -> HRESULT;
    fn Toupcam_read_EEPROM(h: *mut Handle, addr: c_uint,
                           pBuffer: *mut u8, nBufferLen: c_uint) -> HRESULT;

    fn Toupcam_put_Option(h: *mut Handle, iOption: Option, iValue: c_uint) -> HRESULT;
    fn Toupcam_get_Option(h: *mut Handle, iOption: Option, iValue: *mut c_uint) -> HRESULT;

    fn Toupcam_GetHistogram(h: *mut Handle,
                            fnHistogramProc: extern fn(*const [c_double; 256],
                                                       *const [c_double; 256],
                                                       *const [c_double; 256],
                                                       *const [c_double; 256],
                                                       *mut c_void),
                            pHistogramCtx: *mut c_void) -> HRESULT;

    fn Toupcam_calc_ClarityFactor(pImageData: *const u8, bits: c_int,
                                  nImgWidth: c_uint, nImgHeight: c_uint) -> c_double;
    fn Toupcam_deBayer(nBayer: c_uint, nW: c_int, nH: c_int,
                       input: *const u8, output: *mut u8, nBitDepth: c_uchar);
}

/* Helper functions */

fn accept(result: HRESULT) {
    match result {
        HRESULT::S_OK | HRESULT::S_FALSE => (),
        _ => panic!("toupcam: {:?}", result)
    }
}

fn accept_u32(result: HRESULT) -> u32 {
    accept(result);
    result as u32
}

unsafe fn unmarshal_static_string(buf: *const c_char) -> &'static str {
    str::from_utf8(CStr::from_ptr(buf).to_bytes()).unwrap()
}

unsafe fn unmarshal_string(buf: *const c_char) -> String {
    str::from_utf8(CStr::from_ptr(buf).to_bytes()).unwrap().to_owned()
}

unsafe fn unmarshal_strary(ary: &[c_char]) -> String {
    unmarshal_string(&ary[0])
}

macro_rules! property {
    (bool, $reader:ident, $writer:ident, $raw_reader:ident, $raw_writer:ident) =>
    (
        pub fn $reader(&self) -> bool {
            unsafe {
                let mut value = 0;
                accept($raw_reader(self.handle, &mut value));
                value == 1
            }
        }

        pub fn $writer(&self, value: bool) {
            unsafe { accept($raw_writer(self.handle, value as c_int)) }
        }
    );
    (i16, $reader:ident, $writer:ident, $raw_reader:ident, $raw_writer:ident) =>
    (
        pub fn $reader(&self) -> i16 {
            unsafe {
                let mut value = 0;
                accept($raw_reader(self.handle, &mut value));
                value as i16
            }
        }

        pub fn $writer(&self, value: i16) {
            unsafe { accept($raw_writer(self.handle, value as c_short)) }
        }
    );
    (u16, $reader:ident, $writer:ident, $raw_reader:ident, $raw_writer:ident) =>
    (
        pub fn $reader(&self) -> u16 {
            unsafe {
                let mut value = 0;
                accept($raw_reader(self.handle, &mut value));
                value as u16
            }
        }

        pub fn $writer(&self, value: u16) {
            unsafe { accept($raw_writer(self.handle, value as c_ushort)) }
        }
    );
    (i32, $reader:ident, $writer:ident, $raw_reader:ident, $raw_writer:ident) =>
    (
        pub fn $reader(&self) -> i32 {
            unsafe {
                let mut value = 0;
                accept($raw_reader(self.handle, &mut value));
                value as i32
            }
        }

        pub fn $writer(&self, value: i32) {
            unsafe { accept($raw_writer(self.handle, value as c_int)) }
        }
    );    (u32, $reader:ident, $writer:ident, $raw_reader:ident, $raw_writer:ident) =>
    (
        pub fn $reader(&self) -> u32 {
            unsafe {
                let mut value = 0;
                accept($raw_reader(self.handle, &mut value));
                value as u32
            }
        }

        pub fn $writer(&self, value: u32) {
            unsafe { accept($raw_writer(self.handle, value as c_uint)) }
        }
    );
    (Rect, $reader:ident, $writer:ident, $raw_reader:ident, $raw_writer:ident) =>
    (
        pub fn $reader(&self) -> Rect {
            unsafe {
                let mut value = std::mem::zeroed();
                accept($raw_reader(self.handle, &mut value));
                value
            }
        }

        pub fn $writer(&self, value: Rect) {
            unsafe { accept($raw_writer(self.handle, &value)) }
        }
    );
    (bool option, $reader:ident, $writer:ident, $option:expr) =>
    (
        pub fn $reader(&self) -> bool {
            unsafe {
                let mut value = 0;
                accept(Toupcam_get_Option(self.handle, $option, &mut value));
                value == 1
            }
        }

        pub fn $writer(&self, value: bool) {
            unsafe { accept(Toupcam_put_Option(self.handle, $option, value as c_uint)) }
        }
    )
}

/* API wrapper */

pub struct Toupcam<'a> {
    handle  : *mut Handle,
    handler : std::marker::PhantomData<&'a FnMut(Event)>
}

impl<'a> Toupcam<'a> {
    pub fn version() -> &'static str {
        unsafe { unmarshal_static_string(Toupcam_Version()) }
    }

    pub fn set_hotplug_handler<F>(f: &'static F) where F: Fn() {
        extern fn wrapper<F>(closure: *mut c_void) where F: Fn() {
            unsafe { (*(closure as *mut F))() }
        }

        unsafe {
            Toupcam_HotPlug(wrapper::<F>, &f as *const _ as *mut c_void)
        }
    }

    pub fn enumerate() -> Vec<Instance> {
        let mut instances = Vec::new();
        unsafe {
            let mut i_instances: [InstanceInternal; 16] = std::mem::zeroed();
            for i in 0..Toupcam_Enum(&mut i_instances) {
                let i_inst = &i_instances[i as usize];
                let i_model = i_inst.model;
                instances.push(Instance {
                    display_name: unmarshal_strary(&(*i_inst).displayname),
                    unique_id: unmarshal_strary(&(*i_inst).id),
                    model: Model {
                        name: unmarshal_string((*i_model).name),
                        flags: Flags::from_bits((*i_model).flags).unwrap(),
                        maximum_speed: (*i_model).maxspeed,
                        preview_resolutions:
                            (*i_model).res[..(*i_model).preview as usize].to_owned(),
                        still_resolutions:
                            (*i_model).res[..(*i_model).still as usize].to_owned(),
                    },
                })
            }
        }
        instances
    }

    pub fn open(unique_id: std::option::Option<&str>) -> Toupcam {
        let id =
            match unique_id {
                None => null(),
                Some(str) => str.as_ptr()  as *const c_char
            };
        let handle = unsafe { Toupcam_Open(id) };
        if handle.is_null() {
            panic!("toupcam: camera {:?} not found", unique_id)
        }
        Toupcam {
            handle: handle,
            handler: std::marker::PhantomData
        }
    }

    pub fn serial_number(&self) -> String {
        unsafe {
            let mut ret: [c_char; 32] = std::mem::zeroed();
            accept(Toupcam_get_SerialNumber(self.handle, &mut ret));
            unmarshal_string(&ret[0])
        }
    }

    pub fn production_date(&self) -> String {
        unsafe {
            let mut ret: [c_char; 10] = std::mem::zeroed();
            accept(Toupcam_get_ProductionDate(self.handle, &mut ret));
            unmarshal_string(&ret[0])
        }
    }

    pub fn start<F>(&self, mut body: F) where F: FnMut(&Receiver<Event>) {
        extern fn wrapper(event: Event, sender: *mut c_void) {
            unsafe { (*(sender as *const SyncSender<Event>)).send(event).unwrap() }
        }

        unsafe {
            let (tx, rx) = sync_channel(10); // at least 3, for initial exposure events
            accept(Toupcam_StartPullModeWithCallback(
                        self.handle, wrapper, &tx as *const _ as *mut c_void));
            body(&rx);
            accept(Toupcam_Stop(self.handle))
        }
    }

    fn buffer_size(&self, bits: u32, width: u32, height: u32) -> usize {
        #[allow(non_snake_case)]
        fn DIBWIDTHBYTES(bits: u32) -> u32 { ((bits + 31) & !31) / 8 }
        (if self.is_raw_capture_enabled() {
            if self.is_rgb48_format_enabled() {
                width * height * 2
            } else {
                width * height
            }
        } else {
            match bits {
                24 => DIBWIDTHBYTES(24 * width) * height,
                32 => width * height * 4,
                48 => DIBWIDTHBYTES(48 * width) * height,
                8  => DIBWIDTHBYTES(8 * width) * height,
                _  => unreachable!()
            }
        }) as usize
    }

    pub fn pull_image(&self, bits: u32) -> Image {
        unsafe {
            let (mut width, mut height) = std::mem::zeroed();
            accept(Toupcam_PullImage(self.handle, null_mut(), bits as i32,
                                     &mut width, &mut height));
            let mut data = vec![0; self.buffer_size(bits, width, height)];
            accept(Toupcam_PullImage(self.handle, data.as_mut_ptr(), bits as i32,
                                     null_mut(), null_mut()));
            Image {
                resolution: Resolution { width: width, height: height },
                bits: bits,
                data: data
            }
        }
    }

    pub fn pull_still_image(&self, bits: u32) -> Image {
        unsafe {
            let (mut width, mut height) = std::mem::zeroed();
            accept(Toupcam_PullStillImage(self.handle, null_mut(), bits as i32,
                                          &mut width, &mut height));
            let mut data = vec![0; self.buffer_size(bits, width, height)];
            accept(Toupcam_PullStillImage(self.handle, data.as_mut_ptr(), bits as i32,
                                          null_mut(), null_mut()));
            Image {
                resolution: Resolution { width: width, height: height },
                bits: bits,
                data: data
            }
        }
    }

    pub fn pause(&self, do_pause: bool) {
        unsafe {
            accept(Toupcam_Pause(self.handle, if do_pause { 1 } else { 0 }));
        }
    }

    pub fn firmware_version(&self) -> String {
        unsafe {
            let mut ret: [c_char; 16] = std::mem::zeroed();
            accept(Toupcam_get_FwVersion(self.handle, &mut ret));
            unmarshal_string(&ret[0])
        }
    }

    pub fn hardware_version(&self) -> String {
        unsafe {
            let mut ret: [c_char; 16] = std::mem::zeroed();
            accept(Toupcam_get_HwVersion(self.handle, &mut ret));
            unmarshal_string(&ret[0])
        }
    }

    pub fn maximum_bit_depth(&self) -> u32 {
        unsafe {
            accept_u32(Toupcam_get_MaxBitDepth(self.handle))
        }
    }

    pub fn raw_format(&self) -> Format {
        unsafe {
            let (mut fourcc, mut bit_depth) = std::mem::zeroed();
            accept(Toupcam_get_RawFormat(self.handle, &mut fourcc, &mut bit_depth));
            Format { fourcc: fourcc, bit_depth: bit_depth }
        }
    }

    pub fn preview_resolutions(&self) -> Vec<Resolution> {
        let mut resolutions = Vec::new();
        unsafe {
            for i in 0..accept_u32(Toupcam_get_ResolutionNumber(self.handle)) {
                let (mut w, mut h) = (0, 0);
                accept(Toupcam_get_Resolution(self.handle, i, &mut w, &mut h));
                resolutions.push(Resolution { width: w as u32, height: h as u32 })
            }
        }
        resolutions
    }

    pub fn preview_size(&self) -> Resolution {
        unsafe {
            let (mut w, mut h) = (0, 0);
            accept(Toupcam_get_Size(self.handle, &mut w, &mut h));
            Resolution { width: w as u32, height: h as u32 }
        }
    }

    pub fn set_preview_size(&self, res: Resolution) {
        unsafe {
            accept(Toupcam_put_Size(self.handle, res.width as c_int, res.height as c_int))
        }
    }

    pub fn preview_size_index(&self) -> usize {
        unsafe {
            let mut index = 0;
            accept(Toupcam_get_eSize(self.handle, &mut index));
            index as usize
        }
    }

    pub fn set_preview_size_index(&self, index: usize) {
        unsafe {
            accept(Toupcam_put_eSize(self.handle, index as u32))
        }
    }

    pub fn still_resolutions(&self) -> Vec<Resolution> {
        let mut resolutions = Vec::new();
        unsafe {
            for i in 0..accept_u32(Toupcam_get_StillResolutionNumber(self.handle)) {
                let (mut w, mut h) = (0, 0);
                accept(Toupcam_get_StillResolution(self.handle, i, &mut w, &mut h));
                resolutions.push(Resolution { width: w as u32, height: h as u32 })
            }
        }
        resolutions
    }

    pub fn snap(&self, res: Resolution) {
        let index = self.still_resolutions().position_elem(&res).unwrap();
        self.snap_index(index)
    }

    pub fn snap_index(&self, index: usize) {
        unsafe {
            accept(Toupcam_Snap(self.handle, index as u32))
        }
    }

    property!(bool, is_real_time, set_real_time,
                    Toupcam_get_RealTime, Toupcam_put_RealTime);

    property!(i16,  sensor_temperature, set_sensor_temperature,
                    Toupcam_get_Temperature, Toupcam_put_Temperature);

    pub fn rectangle_of_interest(&self) -> Rect {
        unsafe {
            let (mut left, mut top, mut width, mut height) = (0, 0, 0, 0);
            accept(Toupcam_get_Roi(self.handle, &mut left, &mut top, &mut width, &mut height));
            Rect { left: left, top: top, right: left + width, bottom: top + height }
        }
    }

    pub fn set_rectangle_of_interest(&self, value: Rect) {
        unsafe {
            accept(Toupcam_put_Roi(self.handle, value.left, value.top,
                                   value.right - value.left, value.bottom - value.top))
        }
    }

    property!(bool, is_automatic_exposure, set_automatic_exposure,
                    Toupcam_get_AutoExpoEnable, Toupcam_put_AutoExpoEnable);

    property!(u16,  automatic_exposure_target, set_automatic_exposure_target,
                    Toupcam_get_AutoExpoTarget, Toupcam_put_AutoExpoTarget);

    pub fn set_maximum_exposure_time_and_gain(&self, max_time: u32, max_gain: u16) {
        unsafe {
            accept(Toupcam_put_MaxAutoExpoTimeAGain(self.handle, max_time, max_gain))
        }
    }

    /* in microseconds */
    property!(u32,  exposure_time, set_exposure_time,
                    Toupcam_get_ExpoTime, Toupcam_put_ExpoTime);

    pub fn exposure_time_range(&self) -> Range<u32> {
        unsafe {
            let (mut min, mut max, mut def) = (0, 0, 0);
            accept(Toupcam_get_ExpTimeRange(self.handle, &mut min, &mut max, &mut def));
            Range { minimum: min, maximum: max, default: def }
        }
    }

    /* in percents */
    property!(u16,  exposure_gain, set_exposure_gain,
                    Toupcam_get_ExpoAGain, Toupcam_put_ExpoAGain);

    pub fn exposure_gain_range(&self) -> Range<u16> {
        unsafe {
            let (mut min, mut max, mut def) = (0, 0, 0);
            accept(Toupcam_get_ExpoAGainRange(self.handle, &mut min, &mut max, &mut def));
            Range { minimum: min, maximum: max, default: def }
        }
    }

    property!(Rect, automatic_exposure_area, set_automatic_exposure_area,
                    Toupcam_get_AEAuxRect, Toupcam_put_AEAuxRect);

    property!(i32,  hue, set_hue,
                    Toupcam_get_Hue, Toupcam_put_Hue);
    property!(i32,  saturation, set_saturation,
                    Toupcam_get_Saturation, Toupcam_put_Saturation);
    property!(i32,  brightness, set_brightness,
                    Toupcam_get_Brightness, Toupcam_put_Brightness);
    property!(i32,  contrast, set_contrast,
                    Toupcam_get_Contrast, Toupcam_put_Contrast);
    property!(i32,  gamma, set_gamma,
                    Toupcam_get_Gamma, Toupcam_put_Gamma);

    property!(bool, is_monochromatic, set_monochromatic,
                    Toupcam_get_Chrome, Toupcam_put_Chrome);

    property!(bool, is_flipped_vertically, set_flipped_vertically,
                    Toupcam_get_VFlip, Toupcam_put_VFlip);
    property!(bool, is_flipped_horizontally, set_flipped_horizontally,
                    Toupcam_get_HFlip, Toupcam_put_HFlip);

    property!(bool, is_negated, set_negated,
                    Toupcam_get_Negative, Toupcam_put_Negative);

    property!(u16,  speed, get_speed,
                    Toupcam_get_Speed, Toupcam_put_Speed);

    pub fn maximum_speed(&self) -> u32 {
        unsafe {
            accept_u32(Toupcam_get_MaxSpeed(self.handle))
        }
    }

    pub fn flicker_compensation(&self) -> Flicker {
        unsafe {
            let mut hz: Flicker = std::mem::zeroed();
            accept(Toupcam_get_HZ(self.handle, &mut hz));
            hz
        }
    }

    pub fn set_flicker_compensation(&self, value: Flicker) {
        unsafe {
            accept(Toupcam_put_HZ(self.handle, value));
        }
    }

    pub fn sampling_mode(&self) -> SamplingMode {
        unsafe {
            let mut mode: SamplingMode = std::mem::zeroed();
            accept(Toupcam_get_Mode(self.handle, &mut mode));
            mode
        }
    }

    pub fn set_sampling_mode(&self, value: SamplingMode) {
        unsafe {
            accept(Toupcam_put_Mode(self.handle, value));
        }
    }

    pub fn white_balance_temp_tint(&self) -> WhiteBalanceTempTint {
        unsafe {
            let (mut temp, mut tint) = (0, 0);
            accept(Toupcam_get_TempTint(self.handle, &mut temp, &mut tint));
            WhiteBalanceTempTint { temperature: temp as u32, tint: tint as u32 }
        }
    }

    pub fn set_white_balance_temp_tint(&self, value: WhiteBalanceTempTint) {
        unsafe {
            accept(Toupcam_put_TempTint(self.handle, value.temperature as i32, value.tint as i32));
        }
    }

    pub fn automatic_white_balance_oneshot(&self) {
        unsafe {
            accept(Toupcam_AwbOnePush(self.handle, None, null_mut()))
        }
    }

    pub fn white_balance_rgb(&self) -> WhiteBalanceRGB {
        unsafe {
            let mut gain: [c_int; 3] = std::mem::zeroed();
            accept(Toupcam_get_WhiteBalanceGain(self.handle, &mut gain));
            WhiteBalanceRGB { red: gain[0], green: gain[1], blue: gain[2] }
        }
    }

    pub fn set_white_balance_rgb(&self, value: WhiteBalanceRGB) {
        unsafe {
            let gain = [value.red, value.green, value.blue];
            accept(Toupcam_put_WhiteBalanceGain(self.handle, &gain));
        }
    }

    pub fn automatic_white_balance_continuous(&self) {
        unsafe {
            accept(Toupcam_AwbInit(self.handle, None, null_mut()))
        }
    }

    property!(Rect, automatic_white_balance_area, set_automatic_white_balance_area,
                    Toupcam_get_AWBAuxRect, Toupcam_put_AWBAuxRect);

    pub fn level_ranges(&self) -> LevelRanges {
        unsafe {
            let mut low:  [c_ushort; 4] = std::mem::zeroed();
            let mut high: [c_ushort; 4] = std::mem::zeroed();
            accept(Toupcam_get_LevelRange(self.handle, &mut low, &mut high));
            LevelRanges { red:  (low[0], high[0]), green: (low[1], high[1]),
                          blue: (low[2], high[2]), gray:  (low[3], high[3]) }
        }
    }

    pub fn set_level_ranges(&self, value: LevelRanges) {
        unsafe {
            let low  = [value.red.0, value.green.0, value.blue.0, value.gray.0];
            let high = [value.red.1, value.green.1, value.blue.1, value.gray.1];
            accept(Toupcam_put_LevelRange(self.handle, &low, &high));
        }
    }

    pub fn automatic_level_ranges(&self) {
        unsafe {
            accept(Toupcam_LevelRangeAuto(self.handle));
        }
    }

    pub fn set_led_state(&self, led_number: u16, state: LEDState, period: u16) {
        unsafe {
            accept(Toupcam_put_LEDState(self.handle, led_number, state, period))
        }
    }

    pub unsafe fn read_eeprom(&self, address: u32, data: &mut [u8]) {
        accept(Toupcam_read_EEPROM(self.handle, address, &mut data[0], data.len() as u32));
    }

    pub unsafe fn write_eeprom(&self, address: u32, data: &[u8]) {
        accept(Toupcam_write_EEPROM(self.handle, address, &data[0], data.len() as u32));
    }

    property!(bool option, is_noframe_timeout_enabled, set_noframe_timeout_enabled,
                           Option::NoFrameTimeout);
    property!(bool option, is_high_quality_enabled, set_high_quality_enabled,
                           Option::ProcessMode);
    property!(bool option, is_raw_capture_enabled, set_raw_capture_enabled,
                           Option::Raw);
    property!(bool option, is_continuous_histogram_enabled, set_continuous_histogram_enabled,
                           Option::Histogram);
    property!(bool option, is_16_bit_depth_enabled, set_16_bit_depth_enabled,
                           Option::BitDepth);
    property!(bool option, is_fan_enabled, set_fan_enabled,
                           Option::Fan);
    property!(bool option, is_cooler_enabled, set_cooler_enabled,
                           Option::Cooler);
    property!(bool option, is_linear_tone_enabled, set_linear_tone_enabled,
                           Option::Linear);
    property!(bool option, is_curve_tone_enabled, set_curve_tone_enabled,
                           Option::Curve);
    property!(bool option, is_trigger_mode_enabled, set_trigger_mode_enabled,
                           Option::Trigger);
    property!(bool option, is_rgb48_format_enabled, set_rgb48_format_enabled,
                           Option::RGB48);

    // TODO: histogram.
    // Unclear what the lifetime of the callback should be, or when it is called.
}

impl<'a> Drop for Toupcam<'a> {
    fn drop(&mut self) {
        unsafe {
            Toupcam_Close(self.handle)
        }
    }
}

pub fn clarity_factor(image: &Image) -> f64 {
    unsafe {
        Toupcam_calc_ClarityFactor(&image.data[0], image.bits as i32,
                                   image.resolution.width, image.resolution.height)
    }
}

#[test]
fn without_hardware() {
    println!("version: {}", Toupcam::version());
    println!("cameras: {:?}", Toupcam::enumerate());
}

#[test]
fn with_hardware() {
    let cam = Toupcam::open(None);
    println!("serial: {:?}", cam.serial_number());
    println!("production date: {:?}", cam.production_date());
    println!("preview size: {:?}", cam.preview_size());
    println!("raw format: {:?}", cam.raw_format());
    println!("flicker compensation: {:?}", cam.flicker_compensation());
    println!("automatic exposure: {:?} target: {:?} time: {:?} gain: {:?} area: {:?}",
             cam.is_automatic_exposure(), cam.automatic_exposure_target(), cam.exposure_time(),
             cam.exposure_gain(), cam.automatic_exposure_area());
    println!("white balance temp/tint: {:?} area: {:?}",
             cam.white_balance_temp_tint(), cam.automatic_white_balance_area());
    println!("hue: {:?} saturation: {:?} brightness: {:?} contrast: {:?} gamma: {:?}",
             cam.hue(), cam.saturation(), cam.brightness(), cam.contrast(), cam.gamma());
    println!("horizontal flip: {:?} vertical flip: {:?} negated: {:?}",
             cam.is_flipped_horizontally(), cam.is_flipped_vertically(), cam.is_negated());
    println!("level ranges: {:?}", cam.level_ranges());
    cam.start(|eventrx| {
        cam.snap_index(cam.preview_size_index());

        for _ in 0..10 {
            let event = eventrx.recv().unwrap();
            println!("event: {:?}", event);
            match event {
                Event::Image => {
                    let mut image = cam.pull_image(8);
                    println!("clarity: {:?}", clarity_factor(&image));
                    image.data.truncate(100);
                    println!("captured: {:?}", image);
                },
                Event::StillImage => {
                    let mut image = cam.pull_still_image(8);
                    image.data.truncate(100);
                    println!("captured: {:?}", image);
                },
                _ => ()
            }
        }
    });
}
