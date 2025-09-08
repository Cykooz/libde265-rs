use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::ptr::NonNull;

use libde265_sys::*;

use crate::DecoderContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Channel {
    Y,
    Cb,
    Cr,
}

impl Channel {
    pub(crate) fn index(&self) -> c_int {
        match self {
            Channel::Y => 0,
            Channel::Cb => 1,
            Channel::Cr => 2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ChromaFormat {
    Mono,
    C420,
    C422,
    C444,
}

#[derive(Debug, Copy, Clone)]
pub struct NalHeader {
    pub unit_type: u8,
    pub unit_name: &'static CStr,
    pub layer_id: u8,
    pub temporal_id: u8,
}

pub struct Image<'a> {
    inner: *const de265_image,
    decoder: &'a DecoderContext,
}

impl<'a> Drop for Image<'a> {
    fn drop(&mut self) {
        unsafe { de265_release_next_picture(self.decoder.inner) };
    }
}

impl<'a> Image<'a> {
    pub(crate) fn new(decoder: &'a DecoderContext, ptr: NonNull<de265_image>) -> Self {
        Self {
            inner: ptr.as_ptr(),
            decoder,
        }
    }

    pub fn chroma_format(&self) -> ChromaFormat {
        match unsafe { de265_get_chroma_format(self.inner) } {
            de265_chroma::de265_chroma_mono => ChromaFormat::Mono,
            de265_chroma::de265_chroma_420 => ChromaFormat::C420,
            de265_chroma::de265_chroma_422 => ChromaFormat::C422,
            de265_chroma::de265_chroma_444 => ChromaFormat::C444,
            _ => unreachable!(),
        }
    }

    pub fn width(&self, channel: Channel) -> u32 {
        let value = unsafe { de265_get_image_width(self.inner, channel.index()) };
        value.max(0) as u32
    }

    pub fn height(&self, channel: Channel) -> u32 {
        let value = unsafe { de265_get_image_height(self.inner, channel.index()) };
        value.max(0) as u32
    }

    pub fn bits_per_pixel(&self, channel: Channel) -> u32 {
        let value = unsafe { de265_get_bits_per_pixel(self.inner, channel.index()) };
        value.max(0) as u32
    }

    /// Returns the plane data and bytes per line (stride).
    pub fn plane(&self, channel: Channel) -> (&[u8], usize) {
        let mut stride: c_int = 0;
        let buf = unsafe { de265_get_image_plane(self.inner, channel.index(), &mut stride) };
        if buf.is_null() {
            return (&[], 0);
        }
        let stride = stride.max(0) as usize;
        let size = stride * self.height(channel) as usize;
        (unsafe { std::slice::from_raw_parts(buf, size) }, stride)
    }

    pub fn plane_user_data(&self, channel: Channel) -> *mut c_void {
        unsafe { de265_get_image_plane_user_data(self.inner, channel.index()) }
    }

    pub fn user_data(&self) -> usize {
        let ptr = unsafe { de265_get_image_user_data(self.inner) };
        ptr as usize
    }

    /// The presentation time stamp in microseconds.
    pub fn pts(&self) -> i64 {
        let value = unsafe { de265_get_image_PTS(self.inner) };
        value as i64
    }

    /// Get NAL-header information of this frame.
    pub fn nal_header(&self) -> NalHeader {
        let mut unit_type: c_int = 0;
        let mut unit_name: *const c_char = ptr::null();
        let mut layer_id: c_int = 0;
        let mut temporal_id: c_int = 0;

        unsafe {
            de265_get_image_NAL_header(
                self.inner,
                &mut unit_type,
                &mut unit_name,
                &mut layer_id,
                &mut temporal_id,
            )
        }

        let unit_name = if unit_name.is_null() {
            c""
        } else {
            unsafe { CStr::from_ptr(unit_name) }
        };

        NalHeader {
            unit_type: c_int_to_u8(unit_type),
            unit_name,
            layer_id: c_int_to_u8(layer_id),
            temporal_id: c_int_to_u8(temporal_id),
        }
    }

    pub fn full_range(&self) -> bool {
        let value = unsafe { de265_get_image_full_range_flag(self.inner) };
        value != 0
    }

    pub fn colour_primaries(&self) -> u8 {
        let value = unsafe { de265_get_image_colour_primaries(self.inner) };
        c_int_to_u8(value)
    }

    pub fn transfer_characteristics(&self) -> u8 {
        let value = unsafe { de265_get_image_transfer_characteristics(self.inner) };
        c_int_to_u8(value)
    }

    pub fn matrix_coefficients(&self) -> u8 {
        let value = unsafe { de265_get_image_matrix_coefficients(self.inner) };
        c_int_to_u8(value)
    }
}

#[inline(always)]
fn c_int_to_u8(value: c_int) -> u8 {
    debug_assert!((0..=255).contains(&value));
    value.max(0).min(u8::MAX as _) as u8
}
