use std::ptr::NonNull;
use std::rc::Rc;

use libde265_sys::*;

use crate::{DeError, Image, Result};

/// Create a new decoder.
pub fn new_decoder() -> Result<(DecoderInput, DecoderOutput)> {
    let decoder_context_ptr = unsafe { de265_new_decoder() };
    if decoder_context_ptr.is_null() {
        return Err(DeError::ErrorLibraryInitializationFailed);
    }
    let context = Rc::new(DecoderContext {
        inner: decoder_context_ptr,
    });
    Ok((
        DecoderInput {
            context: context.clone(),
        },
        DecoderOutput { context },
    ))
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum ParamI32 {
    /// Dump SPS headers to a specified file-descriptor.
    DumpSpsHeaders = de265_param::DE265_DECODER_PARAM_DUMP_SPS_HEADERS as _,
    /// Dump VPS headers to a specified file-descriptor.
    DumpVpsHeaders = de265_param::DE265_DECODER_PARAM_DUMP_VPS_HEADERS as _,
    /// Dump PPS headers to a specified file-descriptor.
    DumpPpsHeaders = de265_param::DE265_DECODER_PARAM_DUMP_PPS_HEADERS as _,
    /// Dump Slice headers to a specified file-descriptor.
    DumpSliceHeaders = de265_param::DE265_DECODER_PARAM_DUMP_SLICE_HEADERS as _,
}

#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum ParamBool {
    /// Perform SEI hash check on decoded pictures.
    SeiCheckHash = de265_param::DE265_DECODER_PARAM_BOOL_SEI_CHECK_HASH as _,
    /// Do not output frames with decoding errors, default: `false` (output all images)
    SuppressFaultyPictures = de265_param::DE265_DECODER_PARAM_SUPPRESS_FAULTY_PICTURES as _,
    /// Disable deblocking
    DisableDeblocking = de265_param::DE265_DECODER_PARAM_DISABLE_DEBLOCKING as _,
    /// Disable SAO filter
    DisableSAO = de265_param::DE265_DECODER_PARAM_DISABLE_SAO as _,
}

/// Sorted such that a large ID includes all optimizations from lower IDs
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Acceleration {
    /// only fallback implementation
    Scalar = de265_acceleration::de265_acceleration_SCALAR as _,
    MMS = de265_acceleration::de265_acceleration_MMX as _,
    SSE = de265_acceleration::de265_acceleration_SSE as _,
    SSE2 = de265_acceleration::de265_acceleration_SSE2 as _,
    SSE4 = de265_acceleration::de265_acceleration_SSE4 as _,
    /// not implemented yet
    AVX = de265_acceleration::de265_acceleration_AVX as _,
    /// not implemented yet
    AVX2 = de265_acceleration::de265_acceleration_AVX2 as _,
    ARM = de265_acceleration::de265_acceleration_ARM as _,
    NEON = de265_acceleration::de265_acceleration_NEON as _,
    Auto = de265_acceleration::de265_acceleration_AUTO as _,
}

pub(crate) struct DecoderContext {
    pub(crate) inner: *mut de265_decoder_context,
}

impl Drop for DecoderContext {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { de265_free_decoder(self.inner) };
        }
    }
}

pub enum DecodeResult {
    /// The decoding process was finished.
    Done,
    /// The decoded picture buffer has some images in it.
    HasImagesInBuffer,
}

/// Instance of this type is used to push input data for the decoder.
pub struct DecoderInput {
    context: Rc<DecoderContext>,
}

impl DecoderInput {
    #[inline(always)]
    fn inner(&self) -> *mut de265_decoder_context {
        self.context.inner
    }

    /// Initialize background decoding threads.
    ///
    /// If this function is not called, all decoding is done in
    /// the main thread (no multi-threading).
    pub fn start_worker_threads(&mut self, num_threads: u32) -> Result<()> {
        let result = unsafe {
            de265_start_worker_threads(self.inner(), num_threads.min(i32::MAX as _) as _)
        };
        DeError::from_raw(result)
    }

    /// Push more data into the decoder.
    ///
    /// Tha data must be a raw h265 bytestream with startcodes.
    /// The PTS (presentation time stamp) is assigned to all NALs whose
    /// start-code 0x000001 is contained in the data.
    /// The bytestream must contain all stuffing-bytes.
    /// This function only pushes data into the decoder, nothing will be decoded.
    pub fn push_data(&mut self, data: &[u8], pts: i64, user_data: usize) -> Result<()> {
        let result = unsafe {
            de265_push_data(
                self.inner(),
                data.as_ptr() as _,
                data.len() as _,
                pts,
                user_data as _,
            )
        };
        DeError::from_raw(result)
    }

    /// Indicate that the `push_data` method has just received data until the end of a NAL.
    /// The remaining pending input data is put into a NAL package and forwarded to the decoder.
    pub fn push_end_of_nal(&mut self) {
        unsafe { de265_push_end_of_NAL(self.inner()) };
    }

    /// Indicate that the `push_data` method has just received data until the end of a frame.
    ///
    /// All data pending at the decoder input will be pushed into the decoder,
    /// and the decoded picture is pushed to the output queue.
    pub fn push_end_of_frame(&mut self) {
        unsafe { de265_push_end_of_frame(self.inner()) };
    }

    /// Push a complete NAL unit without startcode into the decoder.
    ///
    /// The data must still contain all stuffing-bytes.
    /// This function only pushes data into the decoder, nothing will be decoded.
    pub fn push_nal(&mut self, data: &[u8], pts: i64, user_data: usize) -> Result<()> {
        let result = unsafe {
            de265_push_NAL(
                self.inner(),
                data.as_ptr() as _,
                data.len() as _,
                pts,
                user_data as _,
            )
        };
        DeError::from_raw(result)
    }

    /// Indicate the end-of-stream.
    ///
    /// All data pending at the decoder input will be pushed into the decoder,
    /// and the decoded picture queue will be completely emptied.
    pub fn flush_data(&mut self) -> Result<()> {
        let result = unsafe { de265_flush_data(self.inner()) };
        DeError::from_raw(result)
    }

    /// Return the number of bytes pending at the decoder input.
    ///
    /// Can be used to avoid overflowing the decoder with too much data.
    pub fn number_of_input_bytes_pending(&self) -> usize {
        let value = unsafe { de265_get_number_of_input_bytes_pending(self.inner()) };
        value.max(0) as _
    }

    /// Return the number of NAL units pending at the decoder input.
    ///
    /// Can be used to avoid overflowing the decoder with too much data.
    pub fn number_of_nal_units_pending(&self) -> usize {
        let value = unsafe { de265_get_number_of_NAL_units_pending(self.inner()) };
        value.max(0) as _
    }

    /// Do some decoding.
    ///
    /// Returns status whether it did perform some decoding or why it could not do so.
    ///
    /// The result can be one of the following values:
    /// - [`DecodeResult::Done`] - decoding was finished;
    /// - [`DecodeResult::HasImagesInBuffer`] - the decoded picture buffer contains some images.
    ///
    /// There are a few errors that indicate that this method should be called again
    /// (possibly after resolving the indicated problem).
    /// - [`DeError::ErrorImageBufferFull`] - the decoded picture buffer is full,
    ///   extract some images before continuing;
    /// - [`DeError::ErrorWaitingForInputData`] - insert more data
    ///   before continuing.
    pub fn decode(&mut self) -> Result<DecodeResult> {
        let mut more = 0;
        let result = unsafe { de265_decode(self.inner(), &mut more) };
        DeError::from_raw(result).map(|_| {
            if more > 0 {
                DecodeResult::HasImagesInBuffer
            } else {
                DecodeResult::Done
            }
        })
    }

    /// Push more data into the decoder.
    ///
    /// The data must be raw h265 bytestream.
    /// All complete images in the data will be decoded, hence, do not push
    /// too much data at once to prevent image buffer overflows.
    /// The end of a picture can only be detected when the succeeding start-code
    /// is read from the data.
    /// If you want to flush the data and force decoding of the data so far
    /// (e.g. at the end of a file), call `decode_data()` with an empty slice as
    /// the `data` argument.
    #[deprecated(note = "you should use `push_data` or `push_nal` and `decode` methods instead.")]
    pub fn decode_data(&mut self, data: &[u8]) -> Result<()> {
        let result =
            unsafe { de265_decode_data(self.inner(), data.as_ptr() as _, data.len() as _) };
        DeError::from_raw(result)
    }

    /// Clear decoder state. Call this when skipping in the stream.
    pub fn reset(&mut self) {
        unsafe { de265_reset(self.inner()) };
    }

    pub fn get_warning(&self) -> Result<()> {
        let result = unsafe { de265_get_warning(self.inner()) };
        DeError::from_raw(result)
    }

    /// Returns the maximum layer ID in the stream.
    ///
    /// Note that the maximum layer ID can change throughout the stream.
    pub fn highest_tid(&self) -> u32 {
        unsafe { de265_get_highest_TID(self.inner()).max(0) as _ }
    }

    /// Returns an ID of the currently decoded temporal substream.
    pub fn current_tid(&self) -> u32 {
        unsafe { de265_get_current_TID(self.inner()).max(0) as _ }
    }

    /// Limits decoding to a maximum temporal layer (TID).
    pub fn set_limit_tid(&mut self, max_tid: u32) {
        unsafe { de265_set_limit_TID(self.inner(), max_tid.min(i32::MAX as _) as _) };
    }

    /// It is used for a fine-grained selection of the frame-rate.
    ///
    /// A percentage of 100% will decode all frames in all temporal layers. A lower percentage
    /// will drop approximately as many frames. Note that this is only accurate if the frames
    /// are distributed evenly among the layers. Otherwise, the mapping is non-linear.
    ///
    /// The TID limit has a higher precedence than the framerate ratio. Hence, setting a higher
    /// framerate ratio will decode at TID limit without dropping.
    pub fn set_framerate_ratio(&mut self, percent: u8) {
        unsafe { de265_set_framerate_ratio(self.inner(), percent as _) };
    }

    /// Increase or decrease the output frame-rate to some
    /// discrete preferable value. Currently, these are non-dropped decoding at various
    /// TID layers.
    ///
    /// The `more_vs_less` argument can be one of [-1, 0, 1].
    ///
    /// Returns the corresponding framerate ratio.
    pub fn change_framerate(&mut self, more_vs_less: i8) -> u32 {
        unsafe {
            de265_change_framerate(self.inner(), more_vs_less.clamp(-1, 1) as i32).max(0) as _
        }
    }

    /// Set an integer decoding parameter.
    pub fn set_parameter_i32(&mut self, param: ParamI32, val: i32) {
        unsafe {
            de265_set_parameter_int(self.inner(), param as de265_param::Type, val);
        }
    }

    /// Set a bool decoding parameter.
    pub fn set_parameter_bool(&mut self, param: ParamBool, val: bool) {
        unsafe {
            de265_set_parameter_bool(
                self.inner(),
                param as de265_param::Type,
                if val { 1 } else { 0 },
            );
        }
    }

    /// Set acceleration method, default: [`Acceleration::Auto`]
    pub fn set_acceleration(&mut self, val: Acceleration) {
        unsafe {
            de265_set_parameter_int(
                self.inner(),
                de265_param::DE265_DECODER_PARAM_ACCELERATION_CODE,
                val as i32,
            );
        }
    }

    /// Get a bool decoding parameter.
    pub fn get_parameter_bool(&self, param: ParamBool) -> bool {
        unsafe { de265_get_parameter_bool(self.inner(), param as de265_param::Type) != 0 }
    }
}

/// Instance of this type is used to receive decoded pictures.
pub struct DecoderOutput {
    context: Rc<DecoderContext>,
}

impl DecoderOutput {
    #[inline(always)]
    pub(crate) fn inner(&self) -> *mut de265_decoder_context {
        self.context.inner
    }

    /// Return the next decoded picture if there is any.
    pub fn next_picture(&mut self) -> Option<Image<'_>> {
        let image_ptr = unsafe { de265_peek_next_picture(self.inner()) };
        NonNull::new(image_ptr as _).map(|p| Image::new(self.context.as_ref(), p))
    }
}
