/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

use zune_core::bit_depth::BitType;
use zune_imageprocs::mirror::mirror;
pub use zune_imageprocs::mirror::MirrorMode;

use crate::errors::ImageErrors;
use crate::image::Image;
use crate::traits::OperationsTrait;

/// Rearrange the pixels up side down
pub struct Mirror {
    mode: MirrorMode
}

impl Mirror {
    pub fn new(mode: MirrorMode) -> Mirror {
        Self { mode }
    }
}

impl OperationsTrait for Mirror {
    fn get_name(&self) -> &'static str {
        "Mirror"
    }

    fn execute_impl(&self, image: &mut Image) -> Result<(), ImageErrors> {
        let (width, height) = image.get_dimensions();
        let depth = image.get_depth();

        for channel in image.get_channels_mut(false) {
            match depth.bit_type() {
                BitType::U8 => {
                    mirror(
                        channel.reinterpret_as_mut::<u8>().unwrap(),
                        width,
                        height,
                        self.mode
                    );
                }

                BitType::U16 => {
                    mirror(
                        channel.reinterpret_as_mut::<u16>().unwrap(),
                        width,
                        height,
                        self.mode
                    );
                }
                _ => todo!()
            }
        }

        Ok(())
    }
    fn supported_types(&self) -> &'static [BitType] {
        &[BitType::U8, BitType::U16]
    }
}
