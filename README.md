rust-touptek
============

rust-touptek is a binding for the [ToupLite][] library, which allows to grab images from CMOS instrumentation cameras such as those used in AmScope microscopes.

[touplite]: http://www.touptek.com/download/showdownload.php?lang=en&id=2

Prerequisites
-------------

The ToupLite SDK, installable as a component of ToupLite for [Linux][touplitelinux] or [OS X][toupliteosx].

[touplitelinux]: http://www.touptek.com/download/showdownload.php?lang=en&id=28
[toupliteosx]: http://www.touptek.com/download/showdownload.php?lang=en&id=29

Usage
-----

Add the following to your `Cargo.toml`:

``` toml
[dependencies.touptek]
git = "git://github.com/whitequark/rust-touptek"
```

Capture some pictures:

``` rust
extern crate touptek;

fn main() {
    let cam = touptek::Toupcam::open(None).
                                expect("Need a connected camera!");
    cam.start(|event_rx| {
        loop {
            match event_rx.recv().unwrap() {
                touptek::Event::Image => {
                    let image = cam.pull_image(8);
                    println!("captured a {}x{} image",
                             image.resolution.width, image.resolution.height);
                    println!("first pixel: r {} g {} b {}",
                             image.data[0], image.data[1], image.data[2])
                },
                touptek::Event::Disconnected => break,
                _ => ()
            }
        }
    });
}
```

License
-------

[MIT license](LICENSE.txt)
