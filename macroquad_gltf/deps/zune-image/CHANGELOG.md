# Changelog

### - Zune: (Changes affecting all formats)

- Changed from requiring `&[u8]` for decoders into anything that
  implements `ZReaderTrait`.
  This means that for all formats it's a breaking change, it is still recommended to use `&[u8]` for
  reading, but one can now implement any type of reader (even async, not recommended)
- Log levels reduced to trace

### - zune-bmp

- New decoder, fast as usual, benchmarks will be added.
- Supports almost all types of bmp in [bmpsuite](https://entropymine.com/jason/bmpsuite/bmpsuite/html/bmpsuite.html)(two
  are failing now)
- Fuzz tested (but needs some more testing before full release)

### - zune-hdr

- New decoder and encoder dropped
- Decoder is about 2.7X faster than image-rs/hdr decoder(benchmarks in repo, reproducable
  by `cargo bench --workspace "hdr"`)
- Encoder is also fast.

#### zune-jpeg

- merge Arm changes, arm decode got a bit faster
-

### zune-jpegxl

- Make it compile in `no-std` environments, this looses the threading capabilities (threads require std)

### zune-png

- Add ability to decode Animated PNG. (one can extract frames but not blend them)
- 