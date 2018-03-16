extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use piston_window::*;
use vecmath::*;

// const WIDTH: u32 = 10;
// const HEIGHT: u32 = 10;

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (300, 300);

    let mut window: PistonWindow = WindowSettings::new("opengl_graphics image_test", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut draw = false;
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    let mut last_pos: Option<[f64; 2]> = None;

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            let circle = CircleArc::new([1.0, 0.0, 0.0, 255.0], 20.0, 1.0, 2.0);
            // canvas.put_pixel(0, 0, circle);
            // circle.draw(rectangle(0, 0), canvas);

            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
                circle.draw([10.0, 10.0, 100.0, 100.0], &c.draw_state, c.transform, g);
            });
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true;
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
                last_pos = None;

                // save
                canvas.save("paint.png").unwrap();
            }
        };


        if draw {
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0] as f32, pos[1] as f32);

                if let Some(p) = last_pos {
                    let (last_x, last_y) = (p[0] as f32, p[1] as f32);
                    let distance = vec2_len(vec2_sub(p, pos)) as u32;

                    for i in 0..distance {
                        let diff_x = x - last_x;
                        let diff_y = y - last_y;
                        let delta = i as f32 / distance as f32;
                        let new_x = (last_x + (diff_x * delta)) as u32;
                        let new_y = (last_y + (diff_y * delta)) as u32;
                        if new_x < width && new_y < height {
                            canvas.put_pixel(new_x, new_y, im::Rgba([0, 0, 0, 255]));
                        };
                    };
                };

                last_pos = Some(pos)
            };
        }
    }
}
