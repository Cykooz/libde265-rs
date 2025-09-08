use libde265_sys::de265_error as de;
use thiserror::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Error)]
#[non_exhaustive]
pub enum DeError {
    #[error("Error: No such file")]
    ErrorNoSuchFile,
    #[error("Error: Coefficient out of image bounds")]
    ErrorCoefficientOutOfImageBounds,
    #[error("Error: Checksum mismatch")]
    ErrorChecksumMismatch,
    #[error("Error: CTB outside image area")]
    ErrorCtbOutsideImageArea,
    #[error("Error: Out of memory")]
    ErrorOutOfMemory,
    #[error("Error: Coded parameter out of range")]
    ErrorCodedParameterOutOfRange,
    #[error("Error: Image buffer full")]
    ErrorImageBufferFull,
    #[error("Error: Cannot start threadpool")]
    ErrorCannotStartThreadpool,
    #[error("Error: Library initialization failed")]
    ErrorLibraryInitializationFailed,
    #[error("Error: Library not initialized")]
    ErrorLibraryNotInitialized,
    #[error("Error: Waiting for input data")]
    ErrorWaitingForInputData,
    #[error("Error: Cannot process SEI")]
    ErrorCannotProcessSei,
    #[error("Error: Parameter parsing error")]
    ErrorParameterParsing,
    #[error("Error: No initial slice header")]
    ErrorNoInitialSliceHeader,
    #[error("Error: Premature end of slice")]
    ErrorPrematureEndOfSlice,
    #[error("Error: Unspecified decoding error")]
    ErrorUnspecifiedDecodingError,
    #[error("Error: Not implemented yet")]
    ErrorNotImplementedYet,
    #[error("Warning: No WPP - cannot use multithreading")]
    WarningNoWppCannotUseMultithreading,
    #[error("Warning: Warning buffer full")]
    WarningWarningBufferFull,
    #[error("Warning: Premature end of slice segment")]
    WarningPrematureEndOfSliceSegment,
    #[error("Warning: Incorrect entry point offset")]
    WarningIncorrectEntryPointOffset,
    #[error("Warning: CTB outside image area")]
    WarningCtbOutsideImageArea,
    #[error("Warning: SPS header invalid")]
    WarningSpsHeaderInvalid,
    #[error("Warning: PPS header invalid")]
    WarningPpsHeaderInvalid,
    #[error("Warning: Slice header invalid")]
    WarningSliceHeaderInvalid,
    #[error("Warning: Incorrect motion vector scaling")]
    WarningIncorrectMotionVectorScaling,
    #[error("Warning: Non-existing PPS referenced")]
    WarningNonexistingPpsReferenced,
    #[error("Warning: Non-existing SPS referenced")]
    WarningNonexistingSpsReferenced,
    #[error("Warning: Both prediction flags zero")]
    WarningBothPredFlagsZero,
    #[error("Warning: Non-existing reference picture accessed")]
    WarningNonexistingReferencePictureAccessed,
    #[error("Warning: Number of MVP not equal to number of MVQ")]
    WarningNumMvpNotEqualToNumMvq,
    #[error("Warning: Number of short term reference picture sets out of range")]
    WarningNumberOfShortTermRefPicSetsOutOfRange,
    #[error("Warning: Short term reference picture set out of range")]
    WarningShortTermRefPicSetOutOfRange,
    #[error("Warning: Faulty reference picture list")]
    WarningFaultyReferencePictureList,
    #[error("Warning: EOSS bit not set")]
    WarningEossBitNotSet,
    #[error("Warning: Maximum number of reference pictures exceeded")]
    WarningMaxNumRefPicsExceeded,
    #[error("Warning: Invalid chroma format")]
    WarningInvalidChromaFormat,
    #[error("Warning: Slice segment address invalid")]
    WarningSliceSegmentAddressInvalid,
    #[error("Warning: Dependent slice with address zero")]
    WarningDependentSliceWithAddressZero,
    #[error("Warning: Number of threads limited to maximum")]
    WarningNumberOfThreadsLimitedToMaximum,
    #[error("Warning: Non-existing LT reference candidate in slice header")]
    WarningNonExistingLtReferenceCandidateInSliceHeader,
    #[error("Warning: Cannot apply SAO - out of memory")]
    WarningCannotApplySaoOutOfMemory,
    #[error("Warning: SPS missing - cannot decode SEI")]
    WarningSpsMissingCannotDecodeSei,
    #[error("Warning: Collocated motion vector outside image area")]
    WarningCollocatedMotionVectorOutsideImageArea,
    #[error("Warning: PCM bit depth too large")]
    WarningPcmBitDepthTooLarge,
    #[error("Warning: Reference image bit depth does not match")]
    WarningReferenceImageBitDepthDoesNotMatch,
    #[error("Warning: Reference image size does not match SPS")]
    WarningReferenceImageSizeDoesNotMatchSps,
    #[error("Warning: Chroma of current image does not match SPS")]
    WarningChromaOfCurrentImageDoesNotMatchSps,
    #[error("Warning: Bit depth of current image does not match SPS")]
    WarningBitDepthOfCurrentImageDoesNotMatchSps,
    #[error("Warning: Reference image chroma format does not match")]
    WarningReferenceImageChromaFormatDoesNotMatch,
    #[error("Warning: Invalid slice header index access")]
    WarningInvalidSliceHeaderIndexAccess,
    #[error("Unknown result code: {0}")]
    Unknown(u32),
}

pub type Result<T> = std::result::Result<T, DeError>;

impl DeError {
    pub fn from_raw(raw: de::Type) -> Result<()> {
        let error = match raw {
            de::DE265_OK => return Ok(()),
            de::DE265_ERROR_NO_SUCH_FILE => Self::ErrorNoSuchFile,
            de::DE265_ERROR_COEFFICIENT_OUT_OF_IMAGE_BOUNDS => {
                Self::ErrorCoefficientOutOfImageBounds
            }
            de::DE265_ERROR_CHECKSUM_MISMATCH => Self::ErrorChecksumMismatch,
            de::DE265_ERROR_CTB_OUTSIDE_IMAGE_AREA => Self::ErrorCtbOutsideImageArea,
            de::DE265_ERROR_OUT_OF_MEMORY => Self::ErrorOutOfMemory,
            de::DE265_ERROR_CODED_PARAMETER_OUT_OF_RANGE => Self::ErrorCodedParameterOutOfRange,
            de::DE265_ERROR_IMAGE_BUFFER_FULL => Self::ErrorImageBufferFull,
            de::DE265_ERROR_CANNOT_START_THREADPOOL => Self::ErrorCannotStartThreadpool,
            de::DE265_ERROR_LIBRARY_INITIALIZATION_FAILED => Self::ErrorLibraryInitializationFailed,
            de::DE265_ERROR_LIBRARY_NOT_INITIALIZED => Self::ErrorLibraryNotInitialized,
            de::DE265_ERROR_WAITING_FOR_INPUT_DATA => Self::ErrorWaitingForInputData,
            de::DE265_ERROR_CANNOT_PROCESS_SEI => Self::ErrorCannotProcessSei,
            de::DE265_ERROR_PARAMETER_PARSING => Self::ErrorParameterParsing,
            de::DE265_ERROR_NO_INITIAL_SLICE_HEADER => Self::ErrorNoInitialSliceHeader,
            de::DE265_ERROR_PREMATURE_END_OF_SLICE => Self::ErrorPrematureEndOfSlice,
            de::DE265_ERROR_UNSPECIFIED_DECODING_ERROR => Self::ErrorUnspecifiedDecodingError,
            de::DE265_ERROR_NOT_IMPLEMENTED_YET => Self::ErrorNotImplementedYet,
            de::DE265_WARNING_NO_WPP_CANNOT_USE_MULTITHREADING => {
                Self::WarningNoWppCannotUseMultithreading
            }
            de::DE265_WARNING_WARNING_BUFFER_FULL => Self::WarningWarningBufferFull,
            de::DE265_WARNING_PREMATURE_END_OF_SLICE_SEGMENT => {
                Self::WarningPrematureEndOfSliceSegment
            }
            de::DE265_WARNING_INCORRECT_ENTRY_POINT_OFFSET => {
                Self::WarningIncorrectEntryPointOffset
            }
            de::DE265_WARNING_CTB_OUTSIDE_IMAGE_AREA => Self::WarningCtbOutsideImageArea,
            de::DE265_WARNING_SPS_HEADER_INVALID => Self::WarningSpsHeaderInvalid,
            de::DE265_WARNING_PPS_HEADER_INVALID => Self::WarningPpsHeaderInvalid,
            de::DE265_WARNING_SLICEHEADER_INVALID => Self::WarningSliceHeaderInvalid,
            de::DE265_WARNING_INCORRECT_MOTION_VECTOR_SCALING => {
                Self::WarningIncorrectMotionVectorScaling
            }
            de::DE265_WARNING_NONEXISTING_PPS_REFERENCED => Self::WarningNonexistingPpsReferenced,
            de::DE265_WARNING_NONEXISTING_SPS_REFERENCED => Self::WarningNonexistingSpsReferenced,
            de::DE265_WARNING_BOTH_PREDFLAGS_ZERO => Self::WarningBothPredFlagsZero,
            de::DE265_WARNING_NONEXISTING_REFERENCE_PICTURE_ACCESSED => {
                Self::WarningNonexistingReferencePictureAccessed
            }
            de::DE265_WARNING_NUMMVP_NOT_EQUAL_TO_NUMMVQ => Self::WarningNumMvpNotEqualToNumMvq,
            de::DE265_WARNING_NUMBER_OF_SHORT_TERM_REF_PIC_SETS_OUT_OF_RANGE => {
                Self::WarningNumberOfShortTermRefPicSetsOutOfRange
            }
            de::DE265_WARNING_SHORT_TERM_REF_PIC_SET_OUT_OF_RANGE => {
                Self::WarningShortTermRefPicSetOutOfRange
            }
            de::DE265_WARNING_FAULTY_REFERENCE_PICTURE_LIST => {
                Self::WarningFaultyReferencePictureList
            }
            de::DE265_WARNING_EOSS_BIT_NOT_SET => Self::WarningEossBitNotSet,
            de::DE265_WARNING_MAX_NUM_REF_PICS_EXCEEDED => Self::WarningMaxNumRefPicsExceeded,
            de::DE265_WARNING_INVALID_CHROMA_FORMAT => Self::WarningInvalidChromaFormat,
            de::DE265_WARNING_SLICE_SEGMENT_ADDRESS_INVALID => {
                Self::WarningSliceSegmentAddressInvalid
            }
            de::DE265_WARNING_DEPENDENT_SLICE_WITH_ADDRESS_ZERO => {
                Self::WarningDependentSliceWithAddressZero
            }
            de::DE265_WARNING_NUMBER_OF_THREADS_LIMITED_TO_MAXIMUM => {
                Self::WarningNumberOfThreadsLimitedToMaximum
            }
            de::DE265_NON_EXISTING_LT_REFERENCE_CANDIDATE_IN_SLICE_HEADER => {
                Self::WarningNonExistingLtReferenceCandidateInSliceHeader
            }
            de::DE265_WARNING_CANNOT_APPLY_SAO_OUT_OF_MEMORY => {
                Self::WarningCannotApplySaoOutOfMemory
            }
            de::DE265_WARNING_SPS_MISSING_CANNOT_DECODE_SEI => {
                Self::WarningSpsMissingCannotDecodeSei
            }
            de::DE265_WARNING_COLLOCATED_MOTION_VECTOR_OUTSIDE_IMAGE_AREA => {
                Self::WarningCollocatedMotionVectorOutsideImageArea
            }
            de::DE265_WARNING_PCM_BITDEPTH_TOO_LARGE => Self::WarningPcmBitDepthTooLarge,
            de::DE265_WARNING_REFERENCE_IMAGE_BIT_DEPTH_DOES_NOT_MATCH => {
                Self::WarningReferenceImageBitDepthDoesNotMatch
            }
            de::DE265_WARNING_REFERENCE_IMAGE_SIZE_DOES_NOT_MATCH_SPS => {
                Self::WarningReferenceImageSizeDoesNotMatchSps
            }
            de::DE265_WARNING_CHROMA_OF_CURRENT_IMAGE_DOES_NOT_MATCH_SPS => {
                Self::WarningChromaOfCurrentImageDoesNotMatchSps
            }
            de::DE265_WARNING_BIT_DEPTH_OF_CURRENT_IMAGE_DOES_NOT_MATCH_SPS => {
                Self::WarningBitDepthOfCurrentImageDoesNotMatchSps
            }
            de::DE265_WARNING_REFERENCE_IMAGE_CHROMA_FORMAT_DOES_NOT_MATCH => {
                Self::WarningReferenceImageChromaFormatDoesNotMatch
            }
            de::DE265_WARNING_INVALID_SLICE_HEADER_INDEX_ACCESS => {
                Self::WarningInvalidSliceHeaderIndexAccess
            }
            unknown => Self::Unknown(unknown),
        };
        Err(error)
    }
}
