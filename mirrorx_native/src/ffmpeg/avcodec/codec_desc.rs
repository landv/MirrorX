use super::super::avutil::*;
use super::*;
use std::os::raw::c_char;

#[repr(C)]
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_: AVMediaType,
    pub name: *const c_char,
    pub long_name: *const c_char,
    pub props: i32,
    pub mime_types: *const *const c_char,
    pub profiles: *const *const AVProfile,
}
