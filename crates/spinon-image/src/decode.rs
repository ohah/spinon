use std::{error::Error, fmt, io::Cursor, num::NonZeroU32};

use png::{BitDepth, ColorType, Decoder, Limits, Transformations};

const MAX_ENCODED_BYTES: usize = 16 * 1024 * 1024;
const MAX_AXIS_PIXELS: u32 = 8192;
const MAX_PIXEL_COUNT: usize = 8_388_608;
const MAX_RGBA_BYTES: usize = 32 * 1024 * 1024;
const DECODER_MEMORY_LIMIT: usize = 32 * 1024 * 1024;

/// 디코드한 이미지의 양수 픽셀 크기입니다.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageDimensions {
    width: NonZeroU32,
    height: NonZeroU32,
}

impl ImageDimensions {
    pub const fn width(self) -> u32 {
        self.width.get()
    }

    pub const fn height(self) -> u32 {
        self.height.get()
    }
}

/// 행 padding이 없는 straight-alpha RGBA8 이미지입니다.
///
/// 픽셀 sample은 PNG의 색 공간 값 그대로입니다. ICC/gamma 변환이나 GPU upload 완료를 뜻하지 않습니다.
#[derive(Debug, Eq, PartialEq)]
pub struct DecodedImage {
    dimensions: ImageDimensions,
    rgba8: Box<[u8]>,
}

impl DecodedImage {
    pub const fn dimensions(&self) -> ImageDimensions {
        self.dimensions
    }

    pub fn rgba8(&self) -> &[u8] {
        &self.rgba8
    }

    pub fn byte_len(&self) -> usize {
        self.rgba8.len()
    }
}

/// 제한된 정적 PNG subset을 동기 디코드합니다.
#[derive(Clone, Copy, Debug, Default)]
pub struct PngDecoder;

impl PngDecoder {
    /// 정적 PNG를 straight-alpha RGBA8 sample로 디코드합니다.
    pub fn decode(&self, encoded: &[u8]) -> Result<DecodedImage, ImageDecodeError> {
        if encoded.len() > MAX_ENCODED_BYTES {
            return Err(ImageDecodeError::EncodedInputTooLarge {
                actual: encoded.len(),
                maximum: MAX_ENCODED_BYTES,
            });
        }

        let limits = Limits {
            bytes: DECODER_MEMORY_LIMIT,
        };
        let mut decoder = Decoder::new_with_limits(Cursor::new(encoded), limits);
        decoder.set_ignore_text_chunk(true);
        decoder.set_transformations(Transformations::normalize_to_color8());

        let mut reader = decoder.read_info().map_err(ImageDecodeError::Decoder)?;
        validate_metadata(reader.info())?;

        let (width, height) = reader.info().size();
        let dimensions = validate_dimensions(width, height)?;
        let pixel_count = checked_pixel_count(width, height)?;
        let rgba_len = pixel_count
            .checked_mul(4)
            .filter(|bytes| *bytes <= MAX_RGBA_BYTES)
            .ok_or(ImageDecodeError::DecodedImageTooLarge)?;

        let output_len = reader
            .output_buffer_size()
            .ok_or(ImageDecodeError::DecodedImageTooLarge)?;
        if output_len > MAX_RGBA_BYTES {
            return Err(ImageDecodeError::DecodedImageTooLarge);
        }

        let mut decoded = Vec::new();
        decoded
            .try_reserve_exact(output_len)
            .map_err(|_| ImageDecodeError::AllocationFailed)?;
        decoded.resize(output_len, 0);

        let output = reader
            .next_frame(&mut decoded)
            .map_err(ImageDecodeError::Decoder)?;
        if output.width != width || output.height != height {
            return Err(ImageDecodeError::OutputDimensionsChanged);
        }
        if output.bit_depth != BitDepth::Eight {
            return Err(ImageDecodeError::UnsupportedOutputFormat);
        }

        let channels = channels_per_pixel(output.color_type)
            .ok_or(ImageDecodeError::UnsupportedOutputFormat)?;
        let row_bytes = usize::try_from(width)
            .ok()
            .and_then(|width| width.checked_mul(channels))
            .ok_or(ImageDecodeError::DecodedImageTooLarge)?;
        if output.line_size < row_bytes {
            return Err(ImageDecodeError::OutputLayoutMismatch);
        }
        let written_len = output
            .line_size
            .checked_mul(
                usize::try_from(height).map_err(|_| ImageDecodeError::DecodedImageTooLarge)?,
            )
            .filter(|length| *length <= decoded.len())
            .ok_or(ImageDecodeError::OutputLayoutMismatch)?;

        let mut rgba = Vec::new();
        rgba.try_reserve_exact(rgba_len)
            .map_err(|_| ImageDecodeError::AllocationFailed)?;
        for y in 0..height as usize {
            let row_start = y
                .checked_mul(output.line_size)
                .ok_or(ImageDecodeError::OutputLayoutMismatch)?;
            let row_end = row_start
                .checked_add(row_bytes)
                .filter(|end| *end <= written_len)
                .ok_or(ImageDecodeError::OutputLayoutMismatch)?;
            append_rgba_row(&decoded[row_start..row_end], output.color_type, &mut rgba)?;
        }
        if rgba.len() != rgba_len {
            return Err(ImageDecodeError::OutputLayoutMismatch);
        }

        reader.finish().map_err(ImageDecodeError::Decoder)?;
        validate_metadata(reader.info())?;

        Ok(DecodedImage {
            dimensions,
            rgba8: rgba.into_boxed_slice(),
        })
    }
}

fn validate_dimensions(width: u32, height: u32) -> Result<ImageDimensions, ImageDecodeError> {
    if width == 0 || height == 0 {
        return Err(ImageDecodeError::InvalidDimensions { width, height });
    }
    if width > MAX_AXIS_PIXELS || height > MAX_AXIS_PIXELS {
        return Err(ImageDecodeError::DimensionLimitExceeded {
            width,
            height,
            maximum_axis: MAX_AXIS_PIXELS,
        });
    }
    checked_pixel_count(width, height)?;
    Ok(ImageDimensions {
        width: NonZeroU32::new(width)
            .ok_or(ImageDecodeError::InvalidDimensions { width, height })?,
        height: NonZeroU32::new(height)
            .ok_or(ImageDecodeError::InvalidDimensions { width, height })?,
    })
}

fn checked_pixel_count(width: u32, height: u32) -> Result<usize, ImageDecodeError> {
    let pixels = usize::try_from(width)
        .ok()
        .and_then(|width| {
            usize::try_from(height)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .ok_or(ImageDecodeError::DecodedImageTooLarge)?;
    if pixels > MAX_PIXEL_COUNT {
        return Err(ImageDecodeError::PixelLimitExceeded {
            actual: pixels,
            maximum: MAX_PIXEL_COUNT,
        });
    }
    Ok(pixels)
}

fn validate_metadata(info: &png::Info<'_>) -> Result<(), ImageDecodeError> {
    if info.animation_control.is_some() || info.frame_control.is_some() {
        return Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::Animation,
        ));
    }
    if info.pixel_dims.is_some() {
        return Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::PixelDensity,
        ));
    }
    if info.exif_metadata.is_some() {
        return Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::Exif,
        ));
    }
    if info.icc_profile.is_some()
        || info.gama_chunk.is_some()
        || info.chrm_chunk.is_some()
        || info.coding_independent_code_points.is_some()
        || info.mastering_display_color_volume.is_some()
        || info.content_light_level.is_some()
    {
        return Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::ColorProfile,
        ));
    }
    Ok(())
}

fn channels_per_pixel(color_type: ColorType) -> Option<usize> {
    match color_type {
        ColorType::Grayscale => Some(1),
        ColorType::GrayscaleAlpha => Some(2),
        ColorType::Rgb => Some(3),
        ColorType::Rgba => Some(4),
        ColorType::Indexed => None,
    }
}

fn append_rgba_row(
    row: &[u8],
    color_type: ColorType,
    rgba: &mut Vec<u8>,
) -> Result<(), ImageDecodeError> {
    match color_type {
        ColorType::Grayscale => {
            for gray in row {
                rgba.extend_from_slice(&[*gray, *gray, *gray, u8::MAX]);
            }
        }
        ColorType::GrayscaleAlpha => {
            for pair in row.chunks_exact(2) {
                rgba.extend_from_slice(&[pair[0], pair[0], pair[0], pair[1]]);
            }
        }
        ColorType::Rgb => {
            for pixel in row.chunks_exact(3) {
                rgba.extend_from_slice(&[pixel[0], pixel[1], pixel[2], u8::MAX]);
            }
        }
        ColorType::Rgba => rgba.extend_from_slice(row),
        ColorType::Indexed => return Err(ImageDecodeError::UnsupportedOutputFormat),
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// 이 profile에서 명시적으로 거부하는 PNG metadata 종류입니다.
pub enum UnsupportedImageMetadata {
    Animation,
    PixelDensity,
    Exif,
    ColorProfile,
}

/// PNG 디코드와 자원 제한 오류입니다.
#[derive(Debug)]
pub enum ImageDecodeError {
    EncodedInputTooLarge {
        actual: usize,
        maximum: usize,
    },
    InvalidDimensions {
        width: u32,
        height: u32,
    },
    DimensionLimitExceeded {
        width: u32,
        height: u32,
        maximum_axis: u32,
    },
    PixelLimitExceeded {
        actual: usize,
        maximum: usize,
    },
    DecodedImageTooLarge,
    AllocationFailed,
    UnsupportedOutputFormat,
    OutputDimensionsChanged,
    OutputLayoutMismatch,
    UnsupportedMetadata(UnsupportedImageMetadata),
    Decoder(png::DecodingError),
}

impl fmt::Display for ImageDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncodedInputTooLarge { actual, maximum } => {
                write!(f, "PNG 입력이 {maximum} bytes 제한을 넘었습니다: {actual}")
            }
            Self::InvalidDimensions { width, height } => {
                write!(f, "PNG 크기가 비어 있거나 잘못되었습니다: {width}x{height}")
            }
            Self::DimensionLimitExceeded {
                width,
                height,
                maximum_axis,
            } => write!(
                f,
                "PNG 축 크기가 {maximum_axis} pixel 제한을 넘었습니다: {width}x{height}"
            ),
            Self::PixelLimitExceeded { actual, maximum } => {
                write!(f, "PNG pixel 수가 {maximum} 제한을 넘었습니다: {actual}")
            }
            Self::DecodedImageTooLarge => f.write_str("PNG 디코드 결과가 byte 제한을 넘었습니다"),
            Self::AllocationFailed => f.write_str("PNG 디코드 buffer를 예약하지 못했습니다"),
            Self::UnsupportedOutputFormat => {
                f.write_str("PNG 출력 색상 형식을 RGBA8로 변환할 수 없습니다")
            }
            Self::OutputDimensionsChanged => f.write_str("PNG 출력 크기가 헤더와 다릅니다"),
            Self::OutputLayoutMismatch => {
                f.write_str("PNG 디코더의 row stride 또는 buffer 길이가 맞지 않습니다")
            }
            Self::UnsupportedMetadata(metadata) => match metadata {
                UnsupportedImageMetadata::Animation => {
                    f.write_str("animated PNG는 지원하지 않습니다")
                }
                UnsupportedImageMetadata::PixelDensity => {
                    f.write_str("pHYs pixel density가 있는 PNG는 지원하지 않습니다")
                }
                UnsupportedImageMetadata::Exif => {
                    f.write_str("EXIF metadata가 있는 PNG는 지원하지 않습니다")
                }
                UnsupportedImageMetadata::ColorProfile => {
                    f.write_str("색상 변환이 필요한 PNG color metadata는 지원하지 않습니다")
                }
            },
            Self::Decoder(error) => write!(f, "PNG decode 실패: {error}"),
        }
    }
}

impl Error for ImageDecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Decoder(error) => Some(error),
            _ => None,
        }
    }
}
