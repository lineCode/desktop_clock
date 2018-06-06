extern crate sdl2;
extern crate sdl2_sys;
extern crate time;

use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::image::ImageRWops;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureQuery};
use sdl2::rwops::RWops;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2_sys::SDL_GetGlobalMouseState;
use sdl2_sys::{SDL_Color, SDL_CreateShapedWindow, SDL_SetWindowShape, SDL_WindowShapeMode,
               SDL_WindowShapeParams, WindowShapeMode};
use std::ffi::CString;
use sdl2::video::WindowPos;

const IMG_CLOCK: &'static [u8] = include_bytes!("../resources/clock.png");
const IMG_HOUR: &'static [u8] = include_bytes!("../resources/hour.png");
const IMG_MINUTE: &'static [u8] = include_bytes!("../resources/minute.png");
const IMG_SECOND: &'static [u8] = include_bytes!("../resources/second.png");
const FONT: &'static [u8] = include_bytes!("../resources/regular.otf");

#[no_mangle]
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    //屏幕宽高
    let display_bounds = video_subsystem.display_bounds(0).unwrap_or(Rect::new(0, 0, 500, 500));

    let shaped_window = unsafe {
        SDL_CreateShapedWindow(
            CString::new("桌面时钟").unwrap().as_ptr(),
            display_bounds.width()/2-175,
            display_bounds.height()/2-175,
            256,
            256,
            0,
        )
    };

    let mut window = unsafe { Window::from_ll(video_subsystem, shaped_window) };
    window.set_position(WindowPos::Centered, WindowPos::Centered);

    let mut shape_mode = SDL_WindowShapeMode {
        mode: WindowShapeMode::ShapeModeColorKey,
        parameters: SDL_WindowShapeParams {
            //黑色透明
            colorKey: SDL_Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        },
    };

    //创建圆形窗口

    let surface = {
        let surface = Surface::new(256, 256, PixelFormatEnum::RGBA8888).unwrap();
        let mut canvas = surface.into_canvas().unwrap();
        //填充黑色
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();
        //绘制白色的圆
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 0));
        draw_circle(&mut canvas, Point::new(127, 127), 127);
        canvas.present();
        canvas.into_surface()
    };
    let _result = unsafe { SDL_SetWindowShape(window.raw(), surface.raw(), &mut shape_mode) };

    //加载字体
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_rwops = RWops::from_bytes(FONT).unwrap();
    let font = ttf_context.load_font_from_rwops(font_rwops, 15).unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    //表盘
    let rwops = RWops::from_bytes(IMG_CLOCK).unwrap();
    let clock_texture = texture_creator
        .create_texture_from_surface(&rwops.load_png().unwrap())
        .unwrap();
    //时针
    let rwops = RWops::from_bytes(IMG_HOUR).unwrap();
    let hour_texture = texture_creator
        .create_texture_from_surface(&rwops.load_png().unwrap())
        .unwrap();
    //分针
    let rwops = RWops::from_bytes(IMG_MINUTE).unwrap();
    let minute_texture = texture_creator
        .create_texture_from_surface(&rwops.load_png().unwrap())
        .unwrap();
    //秒针
    let rwops = RWops::from_bytes(IMG_SECOND).unwrap();
    let second_texture = texture_creator
        .create_texture_from_surface(&rwops.load_png().unwrap())
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut down = false;
    let mut mouse_dx = 0;
    let mut mouse_dy = 0;
    let mut last_time = Instant::now();
    let mut start = false;
    let frame_time = Duration::from_millis(30);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    down = true;
                    mouse_dx = x;
                    mouse_dy = y;
                    //捕获鼠标
                    sdl_context.mouse().capture(true);
                }
                Event::MouseButtonUp { .. } => {
                    down = false;
                    sdl_context.mouse().capture(false);
                }
                Event::MouseMotion { .. } => {
                    if down{
                        let mut x = 0;
                        let mut y = 0;
                        unsafe{ SDL_GetGlobalMouseState(&mut x, &mut y); }
                        canvas.window_mut().set_position(WindowPos::Positioned(x-mouse_dx), WindowPos::Positioned(y-mouse_dy));
                    }
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        //::std::thread::sleep(Duration::from_millis(100));
        
        if start{
            if last_time.elapsed() < frame_time{
                continue;
            }else{
                last_time = Instant::now();
            }
        }else{
            start = true;
        }

        //绘制表盘
        canvas
            .copy(&clock_texture, None, Some(Rect::new(0, 0, 255, 255)))
            .unwrap();
        let now = time::now();
        //绘制日期
        let date = format!(
            "{}年{}月{}日",
            1900 + now.tm_year,
            now.tm_mon + 1,
            now.tm_mday
        );
        let font_surface = font.render(&date).solid(Color::RGB(220, 220, 220)).unwrap();
        let font_texture = texture_creator
            .create_texture_from_surface(&font_surface)
            .unwrap();
        let TextureQuery { width, height, .. } = font_texture.query();
        canvas
            .copy(&font_texture, None, Some(Rect::new(80, 200, width, height)))
            .unwrap();
        //绘制星期
        let wday = format!(
            "星期{}",
            match now.tm_wday {
                0 => "日",
                1 => "一",
                2 => "二",
                3 => "三",
                4 => "四",
                5 => "五",
                _ => "？",
            }
        );
        let wday_surface = font.render(&wday).solid(Color::RGB(220, 220, 220)).unwrap();
        let wday_texture = texture_creator
            .create_texture_from_surface(&wday_surface)
            .unwrap();
        let TextureQuery { width, height, .. } = wday_texture.query();
        canvas
            .copy(&wday_texture, None, Some(Rect::new(105, 50, width, height)))
            .unwrap();

        //绘制分针
        let min_degree = (now.tm_min * 60 + now.tm_sec) as f64 * (360.0 / (60 * 60) as f64);
        canvas
            .copy_ex(
                &minute_texture,
                Some(Rect::new(0, 0, 255, 255)),
                Some(Rect::new(0, 0, 255, 255)),
                min_degree,
                None,
                false,
                false,
            )
            .unwrap();

        //绘制时针 360度/720分
        let hour_degree = (now.tm_hour * 60 + now.tm_min) as f64 * (360.0 / 720.0);
        canvas
            .copy_ex(
                &hour_texture,
                Some(Rect::new(0, 0, 255, 255)),
                Some(Rect::new(0, 0, 255, 255)),
                hour_degree,
                None,
                false,
                false,
            )
            .unwrap();

        //绘制秒针: 1秒=1000_000_000纳秒=1000毫秒
        let sec_degree =
            (now.tm_sec * 1000 + now.tm_nsec / 1000_000) as f64 * (360.0 / (60 * 1000) as f64);
        canvas
            .copy_ex(
                &second_texture,
                Some(Rect::new(0, 0, 255, 255)),
                Some(Rect::new(0, 0, 255, 255)),
                sec_degree,
                None,
                false,
                false,
            )
            .unwrap();

        canvas.present();
    }
}

fn draw_circle(canvas: &mut Canvas<Surface>, center: Point, radius: i32) {
    for w in 0..radius * 2 {
        for h in 0..radius * 2 {
            let dx = radius - w; // horizontal offset
            let dy = radius - h; // vertical offset
            if (dx * dx + dy * dy) <= (radius * radius) {
                canvas
                    .draw_point(Point::new(center.x + dx, center.y + dy))
                    .unwrap();
            }
        }
    }
}
