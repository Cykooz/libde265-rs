# libde265-rs

Safe wrapper around the [libde265-sys](https://github.com/Cykooz/libde265-sys)
crate to decode H625 streams.

[CHANGELOG](https://github.com/Cykooz/libde265-rs/blob/master/CHANGELOG.md)

## System dependencies

- `libde265-dev` >= 1.0

### Linux

Crate `libde265-sys` uses `pkg-confing` command to find installed `libde265`.

You can also enable `embedded-libde265` feature to compile `libde265` from
embedded into `libde265-sys` crate sources and then link it statically.

### Windows

Crate `libde265-sys` uses [vcpkg crate](https://crates.io/crates/vcpkg)
to find `libde265` installed with help of `vcpkg`.

You can use [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg)
to install `libde265` with help of `cargo` command:

```shell
cargo vcpkg -v build
```

`cargo-vcpkg` can fetch and build a `vcpkg` installation of required
packages from scratch. It merges package requirements specified in
the `Cargo.toml` of crates in the dependency tree.

## Examples

### Decode H265 stream

```rust
use std::fs::File;
use std::io::Read;

use libde265_rs::*;

#[test]
fn decode_h265() {
    let (mut input, mut output) = new_decoder().unwrap();

    let mut images_count = 0;
    let mut file = File::open("./data/girlshy.h265").unwrap();
    let mut buf = vec![0; 1024];
    loop {
        let size = file.read(&mut buf).unwrap();
        if size == 0 {
            // EOF
            input.flush_data().unwrap();
        } else {
            input.push_data(&buf[0..size], 0, 0).unwrap();
        }

        match input.decode() {
            Ok(DecodeResult::Done) => break,
            Ok(DecodeResult::HasImagesInBuffer) | Err(DeError::ErrorImageBufferFull) => {
                while let Some(image) = output.next_picture() {
                    images_count += 1;

                    assert_eq!(image.chroma_format(), ChromaFormat::C420);
                    assert_eq!(image.width(Channel::Y), 316);
                    assert_eq!(image.height(Channel::Y), 240);
                    assert_eq!(image.bits_per_pixel(Channel::Y), 8);
                    assert!(!image.full_range());

                    let (plane_buf, stride) = image.plane(Channel::Y);
                    assert_eq!(stride, 320);
                    assert_eq!(plane_buf.len(), 320 * 240);
                }
            }
            Err(DeError::ErrorWaitingForInputData) => {}
            Err(err) => panic!("{err:?}"),
        }
    }

    assert_eq!(images_count, 75);
}
```
