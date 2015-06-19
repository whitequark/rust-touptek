#![allow(dead_code, non_camel_case_types)]

#[macro_use]
extern crate bitflags;
extern crate libc;

use libc::c_char;
use libc::c_int;
use libc::c_uint;
use std::ffi::CStr;
use std::str;

#[derive(Debug)]
enum HRESULT {
    S_OK         = 0x00000000, /* Operation successful */
    S_FALSE      = 0x00000001, /* Operation successful */
    E_FAIL       = 0x80004005, /* Unspecified failure */
    E_INVALIDARG = 0x80070057, /* One or more arguments are not valid */
    E_NOTIMPL    = 0x80004001, /* Not supported or not implemented */
    E_POINTER    = 0x80004003, /* Pointer that is not valid */
    E_UNEXPECTED = 0x8000FFFF, /* Unexpected failure */
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
pub enum Event {
    EXPOSURE        = 0x0001, /* exposure time changed */
    TEMPTINT        = 0x0002, /* white balance changed, Temp/Tint mode */
    CHROME          = 0x0003, /* reversed, do not use it */
    IMAGE           = 0x0004, /* live image arrived, use Toupcam_PullImage to get this image */
    STILLIMAGE      = 0x0005, /* snap (still) frame arrived, use Toupcam_PullStillImage to get this frame */
    WBGAIN          = 0x0006, /* white balance changed, RGB Gain mode */
    ERROR           = 0x0080, /* something error happens */
    DISCONNECTED    = 0x0081  /* camera disconnected */
}

#[repr(u32)]
pub enum Option
{
    NOFRAME_TIMEOUT = 0x01,    /* iValue: 1 = enable; 0 = disable. default: enable */
    THREAD_PRIORITY = 0x02,    /* set the priority of the internal thread which grab data from the usb device. iValue: 0 = THREAD_PRIORITY_NORMAL; 1 = THREAD_PRIORITY_ABOVE_NORMAL; 2 = THREAD_PRIORITY_HIGHEST; default: 0; see: msdn SetThreadPriority */
    PROCESSMODE     = 0x03,    /* 0 = better image quality, more cpu usage. this is the default value
                                         1 = lower image quality, less cpu usage */
    RAW             = 0x04,    /* raw mode, read the sensor data. This can be set only BEFORE Toupcam_StartXXX() */
    HISTOGRAM       = 0x05,    /* 0 = only one, 1 = continue mode */
    BITDEPTH        = 0x06,    /* 0 = 8bits mode, 1 = 16bits mode */
    FAN             = 0x07,    /* 0 = turn off the cooling fan, 1 = turn on the cooling fan */
    COOLER          = 0x08,    /* 0 = turn off cooler, 1 = turn on cooler */
    LINEAR          = 0x09,    /* 0 = turn off tone linear, 1 = turn on tone linear */
    CURVE           = 0x0a,    /* 0 = turn off tone curve, 1 = turn on tone curve */
    TRIGGER         = 0x0b,    /* 0 = continuous mode, 1 = trigger mode, default value = 0 */
    RGB48           = 0x0c     /* enable RGB48 format when bitdepth > 8 */
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Rect {
    left            : u32,
    top             : u32,
    right           : u32,
    bottom          : u32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Resolution
{
    width           : u32,
    height          : u32,
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
    name            : String,
    flags           : Flags,
    maximum_speed   : u32,
    preview_resolutions : Vec<Resolution>,
    still_resolutions   : Vec<Resolution>,
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
    display_name    : String, /* display name */
    unique_id       : String, /* unique and opaque id of a connected camera */
    model           : Model,
}

#[repr(C)]
struct Handle;

/* FFI functions */

#[link(name = "toupcam")]
extern {
    fn Toupcam_Version() -> *const c_char;
    fn Toupcam_Enum(pti: *mut [InstanceInternal; 16]) -> c_uint;
    fn Toupcam_Open(id: *const c_char) -> *mut Handle;
    fn Toupcam_Close(h: *mut Handle);
}

/* Helper functions */

fn ensure(result: HRESULT) {
    match result {
        HRESULT::S_OK | HRESULT::S_FALSE => (),
        _ => panic!("toupcam: {:?}", result)
    }
}

unsafe fn unmarshal_static_string(buf: *const c_char) -> &'static str {
    str::from_utf8(CStr::from_ptr(buf).to_bytes()).unwrap()
}

unsafe fn unmarshal_string(buf: *const c_char) -> String {
    str::from_utf8(CStr::from_ptr(buf).to_bytes()).unwrap().to_owned()
}

/* API wrapper */

struct Toupcam {
    handle : *mut Handle
}

impl Toupcam {
    pub fn version() -> &'static str {
        unsafe { unmarshal_static_string(Toupcam_Version()) }
    }

    pub fn enumerate() -> Vec<Instance> {
        let mut instances = Vec::new();
        unsafe {
            let mut i_instances: [InstanceInternal; 16] = std::mem::zeroed();
            for i in 0..Toupcam_Enum(&mut i_instances) {
                let i_inst = &i_instances[i as usize];
                let i_model = i_inst.model;
                instances.push(Instance {
                    display_name:
                        unmarshal_string(&(*i_inst).displayname as *const i8),
                    unique_id:
                        unmarshal_string(&(*i_inst).id as *const i8),
                    model: Model {
                        name: unmarshal_string((*i_model).name),
                        flags: Flags::from_bits((*i_model).flags).unwrap(),
                        maximum_speed: (*i_model).maxspeed,
                        preview_resolutions:
                            (*i_model).res[..(*i_model).preview as usize].to_owned(),
                        still_resolutions:
                            (*i_model).res[(*i_model).preview as usize..
                                           ((*i_model).preview + (*i_model).still) as usize].
                                to_owned(),
                    },
                })
            }
        }
        instances
    }

    pub fn open(unique_id: std::option::Option<&str>) -> Toupcam {
        let id =
            match unique_id {
                None => 0 as *const c_char,
                Some(str) => str.as_ptr()  as *const c_char
            };
        let handle = unsafe { Toupcam_Open(id) };
        if handle.is_null() {
            panic!("toupcam: camera {:?} not found", unique_id)
        }
        Toupcam { handle: handle }
    }
}

impl Drop for Toupcam {
    fn drop(&mut self) {
        unsafe {
            Toupcam_Close(self.handle)
        }
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
}
