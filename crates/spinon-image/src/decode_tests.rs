use super::{ImageDecodeError, PngDecoder, UnsupportedImageMetadata};

const BASIC_PNG: &[u8] = include_bytes!("../../../tests/fixtures/resources/c14-1-png-basic.png");
const ADAM7_PNG: &[u8] = include_bytes!("../../../tests/fixtures/resources/c14-1-png-adam7.png");

#[test]
fn decodes_basic_png_to_bounded_rgba8() {
    let image = PngDecoder.decode(BASIC_PNG).unwrap();
    assert_eq!(image.dimensions().width(), 24);
    assert_eq!(image.dimensions().height(), 16);
    assert_eq!(image.byte_len(), 24 * 16 * 4);
    assert_eq!(&image.rgba8()[0..4], &[255, 255, 64, 255]);
    let right_top = (12 * 4)..(12 * 4 + 4);
    assert_eq!(&image.rgba8()[right_top], &[0, 255, 64, 255]);
}

#[test]
fn decodes_adam7_to_the_same_row_major_rgba8_pixels() {
    let basic = PngDecoder.decode(BASIC_PNG).unwrap();
    let interlaced = PngDecoder.decode(ADAM7_PNG).unwrap();
    assert_eq!(interlaced.dimensions(), basic.dimensions());
    assert_eq!(interlaced.rgba8(), basic.rgba8());
}

#[test]
fn expands_gray_rgb_palette_transparency_and_sixteen_bit_inputs() {
    let gray = encode_png(
        1,
        1,
        png::ColorType::Grayscale,
        png::BitDepth::Eight,
        &[73],
        None,
        None,
    );
    assert_eq!(
        PngDecoder.decode(&gray).unwrap().rgba8(),
        &[73, 73, 73, 255]
    );

    let gray_alpha = encode_png(
        1,
        1,
        png::ColorType::GrayscaleAlpha,
        png::BitDepth::Eight,
        &[33, 99],
        None,
        None,
    );
    assert_eq!(
        PngDecoder.decode(&gray_alpha).unwrap().rgba8(),
        &[33, 33, 33, 99]
    );

    let rgb = encode_png(
        1,
        1,
        png::ColorType::Rgb,
        png::BitDepth::Eight,
        &[1, 2, 3],
        None,
        None,
    );
    assert_eq!(PngDecoder.decode(&rgb).unwrap().rgba8(), &[1, 2, 3, 255]);

    let rgba = encode_png(
        1,
        1,
        png::ColorType::Rgba,
        png::BitDepth::Eight,
        &[4, 5, 6, 7],
        None,
        None,
    );
    assert_eq!(PngDecoder.decode(&rgba).unwrap().rgba8(), &[4, 5, 6, 7]);

    let packed_gray = encode_png(
        2,
        1,
        png::ColorType::Grayscale,
        png::BitDepth::One,
        &[0b0100_0000],
        None,
        None,
    );
    assert_eq!(
        PngDecoder.decode(&packed_gray).unwrap().rgba8(),
        &[0, 0, 0, 255, 255, 255, 255, 255]
    );

    let gray16 = encode_png(
        1,
        1,
        png::ColorType::Grayscale,
        png::BitDepth::Sixteen,
        &[0x7f, 0xff],
        None,
        None,
    );
    assert_eq!(
        PngDecoder.decode(&gray16).unwrap().rgba8(),
        &[127, 127, 127, 255]
    );

    let indexed = encode_png(
        2,
        1,
        png::ColorType::Indexed,
        png::BitDepth::Eight,
        &[0, 1],
        Some(&[10, 20, 30, 40, 50, 60]),
        Some(&[128, 255]),
    );
    assert_eq!(
        PngDecoder.decode(&indexed).unwrap().rgba8(),
        &[10, 20, 30, 128, 40, 50, 60, 255]
    );
}

#[test]
fn rejects_bad_signature_truncation_and_bad_chunk_checksum() {
    assert!(matches!(
        PngDecoder.decode(b"not a png"),
        Err(ImageDecodeError::Decoder(_))
    ));
    assert!(matches!(
        PngDecoder.decode(&BASIC_PNG[..20]),
        Err(ImageDecodeError::Decoder(_))
    ));

    let mut corrupted = BASIC_PNG.to_vec();
    let idat = find_chunk(&corrupted, b"IDAT").unwrap();
    corrupted[idat.crc_offset] ^= 0x80;
    assert!(matches!(
        PngDecoder.decode(&corrupted),
        Err(ImageDecodeError::Decoder(_))
    ));
}

#[test]
fn rejects_encoded_dimensions_and_decoded_pixel_limits_before_inflate() {
    let oversized = vec![0; 16 * 1024 * 1024 + 1];
    assert!(matches!(
        PngDecoder.decode(&oversized),
        Err(ImageDecodeError::EncodedInputTooLarge { .. })
    ));

    let wide = replace_ihdr_dimensions(BASIC_PNG, 8193, 1);
    assert!(matches!(
        PngDecoder.decode(&wide),
        Err(ImageDecodeError::DimensionLimitExceeded { .. })
    ));

    let too_many = replace_ihdr_dimensions(BASIC_PNG, 4096, 2049);
    assert!(matches!(
        PngDecoder.decode(&too_many),
        Err(ImageDecodeError::PixelLimitExceeded { .. })
    ));

    let zero = replace_ihdr_dimensions(BASIC_PNG, 0, 16);
    assert!(matches!(
        PngDecoder.decode(&zero),
        Err(ImageDecodeError::Decoder(_))
    ));
}

#[test]
fn rejects_apng_density_exif_and_unmanaged_color_metadata() {
    let mut animated = with_chunk(BASIC_PNG, *b"acTL", &[0, 0, 0, 2, 0, 0, 0, 0]);
    animated = with_chunk_after(&animated, b"acTL", *b"fcTL", &frame_control_chunk());
    assert!(matches!(
        PngDecoder.decode(&animated),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::Animation
        ))
    ));

    let animation_control_only = with_chunk(BASIC_PNG, *b"acTL", &[0, 0, 0, 1, 0, 0, 0, 0]);
    assert!(matches!(
        PngDecoder.decode(&animation_control_only),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::Animation
        ))
    ));

    let density = with_chunk(BASIC_PNG, *b"pHYs", &[0, 0, 0, 1, 0, 0, 0, 1, 1]);
    assert!(matches!(
        PngDecoder.decode(&density),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::PixelDensity
        ))
    ));

    let exif = with_chunk(BASIC_PNG, *b"eXIf", b"Exif\0\0");
    assert!(matches!(
        PngDecoder.decode(&exif),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::Exif
        ))
    ));

    let exif_after_idat = with_chunk_after(BASIC_PNG, b"IDAT", *b"eXIf", b"Exif\0\0");
    assert!(matches!(
        PngDecoder.decode(&exif_after_idat),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::Exif
        ))
    ));

    let gamma = with_chunk(BASIC_PNG, *b"gAMA", &[0, 1, 0, 0]);
    assert!(matches!(
        PngDecoder.decode(&gamma),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::ColorProfile
        ))
    ));

    let srgb = with_chunk(BASIC_PNG, *b"sRGB", &[0]);
    assert!(PngDecoder.decode(&srgb).is_ok());

    let srgb_with_gamma = with_chunk_after(&gamma, b"gAMA", *b"sRGB", &[0]);
    assert!(matches!(
        PngDecoder.decode(&srgb_with_gamma),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::ColorProfile
        ))
    ));

    let icc = png_with_icc_profile();
    assert!(matches!(
        PngDecoder.decode(&icc),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::ColorProfile
        ))
    ));

    let hdr = with_chunk(BASIC_PNG, *b"cICP", &[9, 16, 0, 0]);
    assert!(matches!(
        PngDecoder.decode(&hdr),
        Err(ImageDecodeError::UnsupportedMetadata(
            UnsupportedImageMetadata::ColorProfile
        ))
    ));
}

#[derive(Clone, Copy)]
struct ChunkLocation {
    data_offset: usize,
    crc_offset: usize,
    end_offset: usize,
}

fn find_chunk(png: &[u8], expected: &[u8; 4]) -> Option<ChunkLocation> {
    let mut cursor: usize = 8;
    while cursor.checked_add(12)? <= png.len() {
        let data_len = usize::try_from(u32::from_be_bytes(
            png.get(cursor..cursor.checked_add(4)?)?.try_into().ok()?,
        ))
        .ok()?;
        let kind_start = cursor.checked_add(4)?;
        let data_offset = cursor.checked_add(8)?;
        let crc_offset = data_offset.checked_add(data_len)?;
        let end_offset = crc_offset.checked_add(4)?;
        if end_offset > png.len() {
            return None;
        }
        if png.get(kind_start..data_offset)? == expected {
            return Some(ChunkLocation {
                data_offset,
                crc_offset,
                end_offset,
            });
        }
        cursor = end_offset;
    }
    None
}

fn with_chunk(png: &[u8], kind: [u8; 4], data: &[u8]) -> Vec<u8> {
    let ihdr = find_chunk(png, b"IHDR").unwrap();
    let mut result = Vec::with_capacity(png.len() + data.len() + 12);
    result.extend_from_slice(&png[..ihdr.end_offset]);
    append_chunk(&mut result, kind, data);
    result.extend_from_slice(&png[ihdr.end_offset..]);
    result
}

fn with_chunk_after(png: &[u8], target: &[u8; 4], kind: [u8; 4], data: &[u8]) -> Vec<u8> {
    let location = find_chunk(png, target).unwrap();
    let mut result = Vec::with_capacity(png.len() + data.len() + 12);
    result.extend_from_slice(&png[..location.end_offset]);
    append_chunk(&mut result, kind, data);
    result.extend_from_slice(&png[location.end_offset..]);
    result
}

fn frame_control_chunk() -> [u8; 26] {
    let mut data = [0; 26];
    data[4..8].copy_from_slice(&24_u32.to_be_bytes());
    data[8..12].copy_from_slice(&16_u32.to_be_bytes());
    data[20..22].copy_from_slice(&1_u16.to_be_bytes());
    data[22..24].copy_from_slice(&10_u16.to_be_bytes());
    data
}

fn replace_ihdr_dimensions(png: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut result = png.to_vec();
    let ihdr = find_chunk(&result, b"IHDR").unwrap();
    result[ihdr.data_offset..ihdr.data_offset + 4].copy_from_slice(&width.to_be_bytes());
    result[ihdr.data_offset + 4..ihdr.data_offset + 8].copy_from_slice(&height.to_be_bytes());
    let crc = crc32(&result[ihdr.data_offset - 4..ihdr.crc_offset]);
    result[ihdr.crc_offset..ihdr.crc_offset + 4].copy_from_slice(&crc.to_be_bytes());
    result
}

fn append_chunk(output: &mut Vec<u8>, kind: [u8; 4], data: &[u8]) {
    output.extend_from_slice(&(data.len() as u32).to_be_bytes());
    output.extend_from_slice(&kind);
    output.extend_from_slice(data);
    let crc = crc32(&output[output.len() - data.len() - 4..]);
    output.extend_from_slice(&crc.to_be_bytes());
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 == 1 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn encode_png(
    width: u32,
    height: u32,
    color_type: png::ColorType,
    bit_depth: png::BitDepth,
    data: &[u8],
    palette: Option<&[u8]>,
    transparency: Option<&[u8]>,
) -> Vec<u8> {
    let mut output = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut output, width, height);
        encoder.set_color(color_type);
        encoder.set_depth(bit_depth);
        if let Some(palette) = palette {
            encoder.set_palette(palette);
        }
        if let Some(transparency) = transparency {
            encoder.set_trns(transparency);
        }
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(data).unwrap();
    }
    output
}

fn png_with_icc_profile() -> Vec<u8> {
    let mut info = png::Info::with_size(1, 1);
    info.color_type = png::ColorType::Rgba;
    info.bit_depth = png::BitDepth::Eight;
    info.icc_profile = Some(b"test profile".as_slice().into());

    let mut output = Vec::new();
    let encoder = png::Encoder::with_info(&mut output, info).unwrap();
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&[1, 2, 3, 255]).unwrap();
    drop(writer);
    output
}
