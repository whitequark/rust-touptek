extern crate touptek;
extern crate png;
extern crate simd;

fn set_alpha(rgba: &mut Vec<u8>, alpha: u8) {
    let alpha = simd::u8x16::new(0, 0, 0, alpha, 0, 0, 0, alpha,
                                 0, 0, 0, alpha, 0, 0, 0, alpha);
    let mut index = 0;
    let length = rgba.len();
    while index < length {
        (simd::u8x16::load(rgba, index) | alpha).store(rgba, index);
        index += 16
    }
}

fn main() {
    let cam = touptek::Toupcam::open(None).
                                expect("Need a connected camera!");
    cam.start(|event_rx| {
        loop {
            match event_rx.recv().unwrap() {
                touptek::Event::Image => {
                    let touptek::Image {
                        resolution: touptek::Resolution { width, height },
                        mut data, ..
                    } = cam.pull_image(32);

                    // The camera will return images with pixels as 32-bit
                    // samples, but the bits corresponding to the alpha channel
                    // are all set to 0, which would make our png's completely
                    // white.
                    set_alpha(&mut data, 255);

                    let filename = "frame.png";
                    png::store_png(&mut png::Image {
                        width: width, height: height,
                        pixels: png::PixelsByColorType::RGBA8(data)
                    }, filename).unwrap();
                    println!("Saved a frame as {:?}", filename);

                    break
                },
                _ => ()
            }
        }
    });
}
