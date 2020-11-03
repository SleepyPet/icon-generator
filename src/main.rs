use std::fs;
use std::path::Path;
use image::{RgbaImage, Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

fn main() {
    let patterns = [
        [
            0b1111111111111111,
            0b1111111111111111,
            0b0000000000000000,
            0b0000000000000000,
            0b1111111111111111,
            0b1111111111111111,
            0b0000000000000000,
            0b0000000000000000,
            0b1111111111111111,
            0b1111111111111111,
            0b0000000000000000,
            0b0000000000000000,
            0b1111111111111111,
            0b1111111111111111,
            0b0000000000000000,
            0b0000000000000000,
        ],
        [
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
            0b1100110011001100,
        ],
        [
            0b1111111111111111,
            0b1000000000000001,
            0b1011111111111101,
            0b1010000000000101,
            0b1010111111110101,
            0b1010100000010101,
            0b1010101111010101,
            0b1010101001010101,
            0b1010101001010101,
            0b1010101111010101,
            0b1010100000010101,
            0b1010111111110101,
            0b1010000000000101,
            0b1011111111111101,
            0b1000000000000001,
            0b1111111111111111,
        ],
        [
            0b1100110011001100,
            0b1100110011001100,
            0b0011001100110011,
            0b0011001100110011,
            0b1100110011001100,
            0b1100110011001100,
            0b0011001100110011,
            0b0011001100110011,
            0b1100110011001100,
            0b1100110011001100,
            0b0011001100110011,
            0b0011001100110011,
            0b1100110011001100,
            0b1100110011001100,
            0b0011001100110011,
            0b0011001100110011,
        ],
        [
            0b0001110000111000,
            0b0001110000111000,
            0b0001110000111000,
            0b1111111111111111,
            0b1111111111111111,
            0b1111111111111111,
            0b0001110000111000,
            0b0001110000111000,
            0b0001110000111000,
            0b0001110000111000,
            0b1111111111111111,
            0b1111111111111111,
            0b1111111111111111,
            0b0001110000111000,
            0b0001110000111000,
            0b0001110000111000,
        ],
        [
            0b0000111100001111,
            0b0000111100001111,
            0b0000111100001111,
            0b0000111100001111,
            0b1111000011110000,
            0b1111000011110000,
            0b1111000011110000,
            0b1111000011110000,
            0b0000111100001111,
            0b0000111100001111,
            0b0000111100001111,
            0b0000111100001111,
            0b1111000011110000,
            0b1111000011110000,
            0b1111000011110000,
            0b1111000011110000,
        ],
    ];

    let colors = [
        Rgba([255, 51, 51, 255]),
        Rgba([34, 255, 34, 255]),
        Rgba([34, 153, 255, 255]),
        Rgba([68, 238, 238, 255]),
        Rgba([238, 51, 238, 255]),
        Rgba([238, 238, 51, 255]),
        Rgba([255, 145, 0, 255]),
        Rgba([153, 153, 153, 255]),
    ];

    fs::create_dir_all("output").unwrap();

    let mut results = Vec::new();

    for (pattern_index, pattern) in patterns.iter().enumerate() {
        for (color_index, color) in colors.iter().enumerate() {
            let mut image = RgbaImage::new(16, 16);

            for (y, line) in pattern.iter().enumerate() {
                let mut bitmask = 32768; // 16th bit set to start from the left.
                for x in 0..16 {
                    if line & bitmask > 0 {
                        draw_filled_rect_mut(&mut image, Rect::at(x, y as i32)
                            .of_size(1, 1), *color);
                    }
                    bitmask >>= 1;
                }
            }

            fs::create_dir_all(format!("output/icon-{}", pattern_index + 1)).unwrap();
            let file_name = format!("output/icon-{}/color-{}.png",
                                    pattern_index + 1, color_index + 1);
            let path = Path::new(&file_name);
            image.save(path).unwrap();

            results.push((file_name, pattern_index + 1, color_index + 1));
        }

        let mut readme_contents = String::from("# Icon Generator

## All icons:

");

        let mut last_pattern_index = 1;
        for result in results.iter() {
            if result.1 != last_pattern_index {
                readme_contents += "\n";
                last_pattern_index = result.1;
            }

            readme_contents += &format!("\n![Icon {}, Color {}](/{})",
                                       result.1, result.2, result.0);
        }

        fs::write("README.md", readme_contents).unwrap();
    }
}
