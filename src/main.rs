mod patterns;

use std::fs;
use std::path::Path;
use image::{RgbaImage, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use hsl::HSL;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

enum Shape {
    Line((f32, f32), (f32, f32))
}

enum Pattern {
    Static([i32; 16]),
    Shapes(Vec<Shape>),
}

fn main() {
    let colors = generate_colors();

    fs::create_dir_all("output").unwrap();

    let mut results = Vec::new();

    let mut patterns = Vec::new();
    for static_pattern in patterns::STATIC_PATTERNS.iter() {
        patterns.push(Pattern::Static(*static_pattern));
    }

    let mut rng = StdRng::seed_from_u64(0);

    for _ in 0..8 {
        let mut shapes = Vec::new();
        for _ in 0..3 {
            shapes.push(Shape::Line(
                (rng.gen_range(0., 16.), rng.gen_range(0., 16.)),
                (rng.gen_range(0., 16.), rng.gen_range(0., 16.)),
            ));
        }

        patterns.push(Pattern::Shapes(shapes));
    }

    for (pattern_index, pattern) in patterns.iter().enumerate() {
        for (color_index, color) in colors.iter().enumerate() {
            let mut image = RgbaImage::new(16, 16);

            match pattern {
                Pattern::Static(pattern) => {
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
                }
                Pattern::Shapes(shapes) => {
                    for shape in shapes.iter() {
                        match shape {
                            Shape::Line(start, end) => {
                                draw_line_segment_mut(&mut image, *start, *end, *color);
                            }
                        }
                    }
                }
            }

            fs::create_dir_all(format!("output/icon-{}", pattern_index + 1)).unwrap();
            let file_name = format!("output/icon-{}/color-{}.png",
                                    pattern_index + 1, color_index + 1);
            let path = Path::new(&file_name);
            image.save(path).unwrap();

            results.push((file_name, pattern_index + 1, color_index + 1));
        }
    }

    update_readme(results);
}

fn generate_colors() -> Vec<Rgba<u8>> {
    let saturation = 0.5;
    let luminosity = 0.5;
    let mut hue = 0.;
    let mut colors = Vec::new();
    for _ in 0..16 {
        let color = HSL {
            h: hue,
            s: saturation,
            l: luminosity,
        };

        let rgb = color.to_rgb();
        colors.push(Rgba([rgb.0, rgb.1, rgb.2, 255]));

        hue += 22.5;
    }

    colors
}

fn update_readme(results: Vec<(String, usize, usize)>) {
    let mut readme_contents = String::from("# Icon Generator

## All icons:");

    let mut last_pattern_index = 0;
    for result in results.iter() {
        if result.1 != last_pattern_index {
            readme_contents += &format!("\n\n### Pattern {}\n", result.1);
            last_pattern_index = result.1;
        }

        readme_contents += &format!("\n![Icon {}, Color {}](/{})",
                                    result.1, result.2, result.0);
    }

    readme_contents += "\n";

    fs::write("README.md", readme_contents).unwrap();
}
