/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

#![cfg(feature = "farbfeld")]
//! Farbfeld decoding and encoding support
//!
//! This uses the delegate library [`zune-farbfeld`](zune_farbfeld)
//! for encoding and decoding images
use zune_core::bit_depth::BitDepth;
use zune_core::bytestream::ZReaderTrait;
use zune_core::colorspace::ColorSpace;
use zune_core::options::EncoderOptions;
pub use zune_farbfeld::*;

use crate::codecs::{create_options_for_encoder, ImageFormat};
use crate::errors::{ImageErrors, ImgEncodeErrors};
use crate::image::Image;
use crate::metadata::ImageMetadata;
use crate::traits::{DecodeInto, DecoderTrait, EncoderTrait};

impl<T> DecoderTrait<T> for FarbFeldDecoder<T>
where
    T: ZReaderTrait
{
    fn decode(&mut self) -> Result<Image, ImageErrors> {
        let pixels = self.decode()?;
        let colorspace = self.get_colorspace();
        let (width, height) = self.get_dimensions().unwrap();

        let mut image = Image::from_u16(&pixels, width, height, colorspace);

        image.metadata.format = Some(ImageFormat::Farbfeld);

        Ok(image)
    }

    fn get_dimensions(&self) -> Option<(usize, usize)> {
        self.get_dimensions()
    }

    fn get_out_colorspace(&self) -> ColorSpace {
        self.get_colorspace()
    }

    fn get_name(&self) -> &'static str {
        "Farbfeld Decoder"
    }

    fn is_experimental(&self) -> bool {
        true
    }

    fn read_headers(&mut self) -> Result<Option<ImageMetadata>, crate::errors::ImageErrors> {
        self.decode_headers()?;

        let (width, height) = self.get_dimensions().unwrap();
        let depth = self.get_bit_depth();

        let metadata = ImageMetadata {
            format: Some(ImageFormat::Farbfeld),
            colorspace: self.get_colorspace(),
            depth: depth,
            width: width,
            height: height,
            ..Default::default()
        };

        Ok(Some(metadata))
    }
}

/// A small wrapper against the Farbfeld encoder that ties
/// the bridge between Image struct and the buffer
/// which [zune_farbfeld::FarbFeldEncoder](zune_farbfeld::FarbFeldEncoder)
/// understands
#[derive(Default)]
pub struct FarbFeldEncoder {
    options: Option<EncoderOptions>
}

impl FarbFeldEncoder {
    /// Create a new encoder
    pub fn new() -> FarbFeldEncoder {
        FarbFeldEncoder::default()
    }
    /// Create a new encoder with specified options
    pub fn new_with_options(options: EncoderOptions) -> FarbFeldEncoder {
        FarbFeldEncoder {
            options: Some(options)
        }
    }
}

impl EncoderTrait for FarbFeldEncoder {
    fn get_name(&self) -> &'static str {
        "farbfeld"
    }

    fn encode_inner(&mut self, image: &Image) -> Result<Vec<u8>, ImageErrors> {
        let options = create_options_for_encoder(self.options, image);

        assert_eq!(image.get_depth(), BitDepth::Sixteen);

        let data = &image.to_u8()[0];

        let encoder_options = zune_farbfeld::FarbFeldEncoder::new(data, options);

        let data = encoder_options
            .encode()
            .map_err(<FarbFeldEncoderErrors as Into<ImgEncodeErrors>>::into)?;

        Ok(data)
    }

    fn supported_colorspaces(&self) -> &'static [ColorSpace] {
        &[ColorSpace::RGBA]
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::Farbfeld
    }

    fn supported_bit_depth(&self) -> &'static [BitDepth] {
        &[BitDepth::Sixteen]
    }

    fn default_depth(&self, _: BitDepth) -> BitDepth {
        BitDepth::Sixteen
    }

    fn default_colorspace(&self, _: ColorSpace) -> ColorSpace {
        ColorSpace::RGBA
    }
}

impl From<FarbFeldEncoderErrors> for ImgEncodeErrors {
    fn from(value: FarbFeldEncoderErrors) -> Self {
        ImgEncodeErrors::ImageEncodeErrors(format!("{:?}", value))
    }
}

impl<T> DecodeInto for FarbFeldDecoder<T>
where
    T: ZReaderTrait
{
    fn decode_into(&mut self, buffer: &mut [u8]) -> Result<(), ImageErrors> {
        self.decode_into(buffer)?;

        Ok(())
    }

    fn output_buffer_size(&mut self) -> Result<usize, ImageErrors> {
        self.decode_headers()?;

        // unwrap is okay because we successfully decoded image headers
        Ok(self.output_buffer_size().unwrap())
    }
}
