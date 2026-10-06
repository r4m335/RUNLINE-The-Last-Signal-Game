use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

#[derive(Resource)]
#[allow(dead_code)]
pub struct SignageAssets {
    pub mat_entrance_sign: Handle<StandardMaterial>,
    pub mat_station_placard: Handle<StandardMaterial>,
    pub mat_poster_veyron: Handle<StandardMaterial>,
    pub mat_graffiti_echo: Handle<StandardMaterial>,
    pub mat_security_placard: Handle<StandardMaterial>,
    pub mat_hazard_stripes: Handle<StandardMaterial>,
}

pub fn init_signage_assets(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    materials: &mut Assets<StandardMaterial>,
) {
    let img_entrance = create_entrance_sign_image();
    let img_station = create_station_placard_image();
    let img_poster = create_veyron_poster_image();
    let img_graffiti = create_echo_graffiti_image();
    let img_security = create_security_placard_image();
    let img_stripes = create_hazard_stripes_image();

    let h_entrance = images.add(img_entrance);
    let h_station = images.add(img_station);
    let h_poster = images.add(img_poster);
    let h_graffiti = images.add(img_graffiti);
    let h_security = images.add(img_security);
    let h_stripes = images.add(img_stripes);

    let signage = SignageAssets {
        mat_entrance_sign: materials.add(StandardMaterial {
            base_color_texture: Some(h_entrance.clone()),
            emissive: LinearRgba::new(1.8, 1.2, 0.3, 1.0),
            emissive_texture: Some(h_entrance),
            perceptual_roughness: 0.45,
            metallic: 0.3,
            ..default()
        }),
        mat_station_placard: materials.add(StandardMaterial {
            base_color_texture: Some(h_station.clone()),
            emissive: LinearRgba::new(1.2, 0.9, 0.4, 1.0),
            emissive_texture: Some(h_station),
            perceptual_roughness: 0.35,
            metallic: 0.2,
            ..default()
        }),
        mat_poster_veyron: materials.add(StandardMaterial {
            base_color_texture: Some(h_poster.clone()),
            emissive: LinearRgba::new(0.6, 0.5, 0.3, 1.0),
            emissive_texture: Some(h_poster),
            perceptual_roughness: 0.65,
            metallic: 0.05,
            ..default()
        }),
        mat_graffiti_echo: materials.add(StandardMaterial {
            base_color_texture: Some(h_graffiti.clone()),
            emissive: LinearRgba::new(0.5, 3.2, 4.5, 1.0),
            emissive_texture: Some(h_graffiti),
            perceptual_roughness: 0.20,
            metallic: 0.1,
            ..default()
        }),
        mat_security_placard: materials.add(StandardMaterial {
            base_color_texture: Some(h_security.clone()),
            emissive: LinearRgba::new(3.8, 0.3, 0.3, 1.0),
            emissive_texture: Some(h_security),
            perceptual_roughness: 0.25,
            metallic: 0.7,
            ..default()
        }),
        mat_hazard_stripes: materials.add(StandardMaterial {
            base_color_texture: Some(h_stripes.clone()),
            emissive: LinearRgba::new(0.8, 0.6, 0.05, 1.0),
            emissive_texture: Some(h_stripes),
            perceptual_roughness: 0.35,
            metallic: 0.2,
            ..default()
        }),
    };

    commands.insert_resource(signage);
}

// -----------------------------------------------------------------------------
// TEXTURE GENERATION
// -----------------------------------------------------------------------------

fn create_entrance_sign_image() -> Image {
    let w = 512;
    let h = 128;
    let mut buf = vec![0u8; w * h * 4];

    // Background: dark industrial iron with slight noise
    for y in 0..h {
        for x in 0..w {
            let base_c = if y < 12 || y >= h - 12 {
                // Border area for hazard chevrons
                if ((x + y) / 10) % 2 == 0 {
                    [230, 180, 20, 255]
                } else {
                    [20, 20, 22, 255]
                }
            } else {
                let rust = ((x * 13 + y * 29) % 17) as u8;
                [24 + rust, 26 + rust, 28 + rust, 255]
            };
            set_pixel(&mut buf, w, x, y, base_c);
        }
    }

    // Text: AURELIA METRO L NE 04 (damaged missing 'I')
    let text_color = [255, 195, 55, 255];
    draw_string(&mut buf, w, h, "AURELIA METRO L NE 04", 36, 32, 4, text_color);
    draw_string(&mut buf, w, h, "SECTOR 04 // SUB-LEVEL 7", 72, 76, 2, [180, 140, 40, 255]);

    // Damaged flickering scratches on right half
    for y in 24..104 {
        for x in 340..490 {
            if (x * 7 + y * 19) % 23 == 0 {
                set_pixel(&mut buf, w, x, y, [10, 10, 12, 255]);
            }
        }
    }

    make_image(w as u32, h as u32, buf)
}

fn create_station_placard_image() -> Image {
    let w = 512;
    let h = 128;
    let mut buf = vec![0u8; w * h * 4];

    // Background: Subway enamel dark blue with white border
    for y in 0..h {
        for x in 0..w {
            let is_border = x < 8 || x >= w - 8 || y < 8 || y >= h - 8;
            let col = if is_border {
                [220, 225, 230, 255]
            } else {
                [18, 38, 68, 255]
            };
            set_pixel(&mut buf, w, x, y, col);
        }
    }

    let text_color = [245, 240, 220, 255];
    draw_string(&mut buf, w, h, "SECTOR 04 OLD METRO", 52, 28, 3, text_color);
    draw_string(&mut buf, w, h, "PLATFORM 07", 168, 62, 3, [255, 205, 50, 255]);
    draw_string(&mut buf, w, h, "<- SURFACE // INDUSTRIAL ->", 78, 96, 2, [180, 200, 215, 255]);

    make_image(w as u32, h as u32, buf)
}

fn create_veyron_poster_image() -> Image {
    let w = 256;
    let h = 384;
    let mut buf = vec![0u8; w * h * 4];

    // Background: weathered corporate cream poster with water stains
    for y in 0..h {
        for x in 0..w {
            let stain = (((x * 11 + y * 7) % 31) as i16 - 15).clamp(-10, 10) as i16;
            let r = (205 + stain).clamp(0, 255) as u8;
            let g = (195 + stain).clamp(0, 255) as u8;
            let b = (180 + stain).clamp(0, 255) as u8;
            set_pixel(&mut buf, w, x, y, [r, g, b, 255]);
        }
    }

    // Header bar (brutalist black)
    for y in 24..90 {
        for x in 16..w - 16 {
            set_pixel(&mut buf, w, x, y, [24, 26, 30, 255]);
        }
    }

    draw_string(&mut buf, w, h, "VEYRON", 54, 38, 4, [245, 245, 245, 255]);
    draw_string(&mut buf, w, h, "DYNAMICS", 62, 68, 2, [210, 175, 45, 255]);

    // Veyron Corporate Chevron Emblem
    for cy in 120..190 {
        for cx in 64..w - 64 {
            let rel_x = cx as i32 - 128;
            let rel_y = cy as i32 - 145;
            if (rel_y - rel_x.abs() / 2).abs() < 9 {
                set_pixel(&mut buf, w, cx, cy, [180, 140, 30, 255]);
            }
        }
    }

    draw_string(&mut buf, w, h, "BUILD TOMORROW", 36, 230, 2, [30, 32, 35, 255]);
    draw_string(&mut buf, w, h, "TODAY", 96, 258, 3, [195, 30, 35, 255]);
    draw_string(&mut buf, w, h, "INFRASTRUCTURE DIV", 42, 320, 1, [90, 95, 100, 255]);

    make_image(w as u32, h as u32, buf)
}

fn create_echo_graffiti_image() -> Image {
    let w = 256;
    let h = 256;
    let mut buf = vec![0u8; w * h * 4];

    // Background: weathered dark subway tiles
    for y in 0..h {
        for x in 0..w {
            let is_grout = x % 32 == 0 || y % 32 == 0;
            let col = if is_grout {
                [22, 24, 26, 255]
            } else {
                let dirt = ((x * 19 + y * 23) % 13) as u8;
                [42 + dirt, 44 + dirt, 46 + dirt, 255]
            };
            set_pixel(&mut buf, w, x, y, col);
        }
    }

    // Stylized ECHO Diamond Insignia
    let cyan = [10, 235, 255, 255];
    for y in 40..130 {
        for x in 60..196 {
            let dx = (x as i32 - 128).abs();
            let dy = (y as i32 - 85).abs();
            if (dx + dy) < 45 && (dx + dy) > 32 {
                set_pixel(&mut buf, w, x, y, cyan);
            }
        }
    }

    // Graffiti text: ECHO LIVES
    draw_string(&mut buf, w, h, "ECHO LIVES", 44, 150, 3, cyan);
    draw_string(&mut buf, w, h, "DO NOT SURRENDER", 38, 195, 1, [0, 210, 240, 255]);

    // Drips dripping downward
    for (drip_x, length) in [(78, 28), (128, 38), (174, 24)] {
        for dy in 0..length {
            let py = 176 + dy;
            if py < h {
                set_pixel(&mut buf, w, drip_x, py, cyan);
                set_pixel(&mut buf, w, drip_x + 1, py, cyan);
            }
        }
    }

    make_image(w as u32, h as u32, buf)
}

fn create_security_placard_image() -> Image {
    let w = 256;
    let h = 256;
    let mut buf = vec![0u8; w * h * 4];

    // Background: matte black
    for y in 0..h {
        for x in 0..w {
            set_pixel(&mut buf, w, x, y, [12, 14, 16, 255]);
        }
    }

    // Red warning header bar
    for y in 12..64 {
        for x in 12..w - 12 {
            set_pixel(&mut buf, w, x, y, [215, 20, 30, 255]);
        }
    }
    draw_string(&mut buf, w, h, "RESTRICTED", 48, 28, 2, [255, 255, 255, 255]);

    draw_string(&mut buf, w, h, "VEYRON DYNAMICS", 36, 88, 2, [240, 240, 240, 255]);
    draw_string(&mut buf, w, h, "SECURITY CHECKPOINT", 24, 115, 1, [215, 20, 30, 255]);
    draw_string(&mut buf, w, h, "LETHAL RESPONSE ACTIVE", 18, 145, 1, [200, 200, 200, 255]);
    draw_string(&mut buf, w, h, "TRANSIT SECTOR 04", 42, 175, 1, [150, 150, 150, 255]);

    // Barcode at bottom
    for y in 205..235 {
        for x in 32..w - 32 {
            let bar = (x * 7) % 11 < 6;
            let col = if bar { [220, 220, 220, 255] } else { [12, 14, 16, 255] };
            set_pixel(&mut buf, w, x, y, col);
        }
    }

    make_image(w as u32, h as u32, buf)
}

fn create_hazard_stripes_image() -> Image {
    let w = 128;
    let h = 128;
    let mut buf = vec![0u8; w * h * 4];

    for y in 0..h {
        for x in 0..w {
            let col = if ((x + y) / 16) % 2 == 0 {
                [245, 185, 0, 255] // Hazard Yellow
            } else {
                [18, 18, 20, 255]  // Hazard Black
            };
            set_pixel(&mut buf, w, x, y, col);
        }
    }

    make_image(w as u32, h as u32, buf)
}

fn make_image(w: u32, h: u32, data: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn set_pixel(buf: &mut [u8], stride: usize, x: usize, y: usize, color: [u8; 4]) {
    if x < stride && y * stride + x < buf.len() / 4 {
        let idx = (y * stride + x) * 4;
        buf[idx] = color[0];
        buf[idx + 1] = color[1];
        buf[idx + 2] = color[2];
        buf[idx + 3] = color[3];
    }
}

// -----------------------------------------------------------------------------
// BITMAP FONT RENDERER (5x7 PIXEL MATRIX)
// -----------------------------------------------------------------------------
fn draw_string(
    buf: &mut [u8],
    stride: usize,
    max_h: usize,
    text: &str,
    start_x: usize,
    start_y: usize,
    scale: usize,
    color: [u8; 4],
) {
    let mut cur_x = start_x;
    for ch in text.chars() {
        let glyph = get_glyph(ch);
        draw_glyph(buf, stride, max_h, glyph, cur_x, start_y, scale, color);
        cur_x += (5 + 1) * scale;
    }
}

fn draw_glyph(
    buf: &mut [u8],
    stride: usize,
    max_h: usize,
    rows: [u8; 7],
    x: usize,
    y: usize,
    scale: usize,
    color: [u8; 4],
) {
    for row_idx in 0..7 {
        let row_bits = rows[row_idx];
        for col_idx in 0..5 {
            if (row_bits & (1 << (4 - col_idx))) != 0 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x + col_idx * scale + sx;
                        let py = y + row_idx * scale + sy;
                        if py < max_h {
                            set_pixel(buf, stride, px, py, color);
                        }
                    }
                }
            }
        }
    }
}

fn get_glyph(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        'C' => [0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111],
        'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        'G' => [0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'I' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111],
        'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100],
        'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => [0b10001, 0b11011, 0b10101, 0b10001, 0b10001, 0b10001, 0b10001],
        'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001],
        'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
        '0' => [0b01110, 0b10011, 0b10101, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        '3' => [0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110],
        '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b00001, 0b11110],
        '6' => [0b01110, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b01110],
        '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '_' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
        '/' => [0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00000, 0b00000],
        '<' => [0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010],
        '>' => [0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000],
        _ => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
    }
}
