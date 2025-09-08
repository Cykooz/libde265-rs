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
        match input.decode() {
            Ok(DecodeResult::Done) => break,
            Ok(DecodeResult::HasImagesInBuffer) | Err(DeError::ErrorImageBufferFull) => {
                while let Some(image) = output.next_picture() {
                    images_count += 1;
                    assert_eq!(image.width(Channel::Y), 316);
                    assert_eq!(image.height(Channel::Y), 240);
                    assert_eq!(image.chroma_format(), ChromaFormat::C420);
                    assert!(!image.full_range());
                    assert_eq!(image.bits_per_pixel(Channel::Y), 8);
                    let (plane_buf, stride) = image.plane(Channel::Y);
                    assert_eq!(stride, 320);
                    assert_eq!(plane_buf.len(), 320 * 240);
                }
            }
            Err(DeError::ErrorWaitingForInputData) => {}
            Err(err) => panic!("{:?}", err),
        }

        let size = file.read(&mut buf).unwrap();
        if size == 0 {
            // EOF
            input.flush_data().unwrap();
        } else {
            input.push_data(&buf[0..size], 0, 0).unwrap();
        }
    }

    assert_eq!(images_count, 75);
}
