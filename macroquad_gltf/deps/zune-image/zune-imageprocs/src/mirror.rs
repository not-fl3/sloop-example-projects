/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MirrorMode {
    North,
    South,
    East,
    West
}
/// Mirror an image by duplicating pixels from one edge to the other half
///
/// E.g a mirror along the east direction looks like
///
/// ```text           
///  old image     new image
///  ┌─────────┐   ┌──────────┐
///  │a b c d e│   │a b c b a │
///  │f g h i j│   │f g h g f │
///  └─────────┘   └──────────┘
/// ```
pub fn mirror<T: Copy>(in_pixels: &mut [T], width: usize, height: usize, mode: MirrorMode) {
    if mode == MirrorMode::East || mode == MirrorMode::West {
        for width_stride in in_pixels.chunks_exact_mut(width) {
            // split into 2
            let (left, right) = width_stride.split_at_mut(width / 2);

            if mode == MirrorMode::West {
                // write
                left.iter().zip(right.iter_mut().rev()).for_each(|(l, r)| {
                    *r = *l;
                });
            }
            if mode == MirrorMode::East {
                // write
                left.iter_mut().zip(right.iter().rev()).for_each(|(l, r)| {
                    *l = *r;
                });
            }
        }
    } else if mode == MirrorMode::North || mode == MirrorMode::South {
        // split the image along the halfway axis
        let halfway = width * (height / 2);

        let (top, bottom) = in_pixels.split_at_mut(halfway);

        for (top_width_stride, bottom_width_stride) in top
            .chunks_exact_mut(width)
            .zip(bottom.rchunks_exact_mut(width))
        {
            if mode == MirrorMode::North {
                bottom_width_stride.copy_from_slice(top_width_stride);
            } else if mode == MirrorMode::South {
                top_width_stride.copy_from_slice(bottom_width_stride);
            }
        }
    }
}
