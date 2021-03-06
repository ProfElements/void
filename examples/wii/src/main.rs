#![no_std]
#![feature(start)]

extern crate alloc;

use core::ffi::c_void;

mod graphics;
use graphics::{Graphics, Image};

mod audio;
use audio::Audio;

use ogc_rs::{
    ffi::{TPL_GetTexture, TPL_OpenTPLFromMemory},
    prelude::*,
};

use void_audio::player::{AudioPlayer,AudioFormat, AudioFrequency, AudioOptions};
use void_gfx::{
    geometry::{Color, Vec2},
    primitives::{Ellipse, Line, Polyline, Rectangle, Text, Triangle},
    renderable::Renderable,
};

#[derive(Copy, Clone)]
#[repr(C, align(32))]
pub struct Align32<T>(pub T);

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn main() {
    let mut graphics = Graphics::new();
    let audio = Audio::new();
    let mut select =
        ogc_rs::utils::alloc_aligned_buffer(include_bytes!("../assets/select.pcm.raw"));

    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let gcn = Input::new(ControllerType::Gamecube, ControllerPort::One);
    let wii = Input::new(ControllerType::Wii, ControllerPort::One);
    wii.as_wpad()
        .set_data_format(WPadDataFormat::ButtonsAccelIR);

    println!("Hello, World");
    let background_rect = Rectangle::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(640.0, 480.0),
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    let central_ellipse = Ellipse::new(
        Vec2::new(640.0 / 2.0 - 128.0, 480.0 / 2.0 - 128.0),
        Vec2::new(256.0, 256.0),
        Color::new(0.0, 1.0, 1.0, 1.0),
    );

    let hori_line = Line::new(
        Vec2::new(640.0 / 2.0 - 128.0, 480.0 / 2.0),
        Vec2::new(640.0 / 2.0 + 128.0, 480.0 / 2.0), 
        Color::new(1.0, 0.0, 1.0, 1.0),
    );

    let veri_line = Line::new(
        Vec2::new(640.0 / 2.0, 480.0 / 2.0 - 128.0),
        Vec2::new(640.0 / 2.0, 480.0 / 2.0 + 128.0),
        Color::new(1.0, 0.0, 1.0, 1.0),
    );

    let corner_tri1 = Triangle::new(
        [
            Vec2::new(0.0, 0.0),
            Vec2::new(TRI_SIZE, 0.0),
            Vec2::new(0.0, TRI_SIZE),
        ],
        Color::new(1.0, 1.0, 0.0, 1.0),
    );
    let corner_tri2 = Triangle::new(
        [
            Vec2::new(640.0, 0.0),
            Vec2::new(640.0 - TRI_SIZE, 0.0),
            Vec2::new(640.0, TRI_SIZE),
        ],
        Color::new(1.0, 1.0, 0.0, 1.0),
    );
    let corner_tri3 = Triangle::new(
        [
            Vec2::new(0.0, 480.0),
            Vec2::new(TRI_SIZE, 480.0),
            Vec2::new(0.0, 480.0 - TRI_SIZE),
        ],
        Color::new(1.0, 1.0, 0.0, 1.0),
    );
    let corner_tri4 = Triangle::new(
        [
            Vec2::new(640.0, 480.0),
            Vec2::new(640.0 - TRI_SIZE, 480.0),
            Vec2::new(640.0, 480.0 - TRI_SIZE),
        ],
        Color::new(1.0, 1.0, 0.0, 1.0),
    );

    let poly_vertices = [
        Vec2::new(POLYLINE_SIZE, POLYLINE_SIZE),
        Vec2::new(640.0 - POLYLINE_SIZE, POLYLINE_SIZE),
        Vec2::new(640.0 - POLYLINE_SIZE, 480.0 - POLYLINE_SIZE),
        Vec2::new(POLYLINE_SIZE, 480.0 - POLYLINE_SIZE),
        Vec2::new(POLYLINE_SIZE, POLYLINE_SIZE),
    ];

    let poly_outline = Polyline::new(&poly_vertices, Color::new(0.0, 1.0, 0.0, 1.0));

    let hello_world = Text::new(
        Vec2::new(640.0 / 2.0 - 90.0, 480.0 / 2.0 + 134.0),
        "Hello, World!",
        24.0,
        Color::new(0.0, 0.0, 0.0, 1.0),
    );

    let pointer_bytes = include_bytes!("../assets/pointer.tpl");

    let mut pointer_tpl = unsafe { core::mem::zeroed() };
    let mut pointer_obj = unsafe { core::mem::zeroed() };
    let pointer_aligned = Align32(*pointer_bytes);
    unsafe {
        TPL_OpenTPLFromMemory(
            &mut pointer_tpl,
            pointer_aligned.0.as_ptr() as *mut c_void,
            pointer_aligned.0.len().try_into().unwrap(),
        );
        TPL_GetTexture(&mut pointer_tpl, 0, &mut pointer_obj);
    }

    let pointer_tex: Texture = pointer_obj.into();
    let mut pointer = Image::new(
        Vec2::new(0.0, 0.0),
        Color::new(1.0, 1.0, 1.0, 1.0),
        pointer_tex,
    );

    const TRI_SIZE: f32 = 64.0;
    const POLYLINE_SIZE: f32 = 16.0;
    'main_loop: loop {
        Input::update(ControllerType::Gamecube);
        Input::update(ControllerType::Wii);

        if gcn.is_button_down(Button::Start) {
            break 'main_loop;
        }

        if wii.is_button_down(Button::Home) {
            break 'main_loop;
        }

        if wii.is_button_down(Button::A) {
            audio
                .play_pcm_buffer(
                    AudioOptions::new()
                        .format(AudioFormat::Mono16BitLe)
                        .frequency(AudioFrequency::FourtyFourOneHz)
                        .volume((1.0, 1.0)),
                    0,
                    &mut select,
                )
                .unwrap();
            //audio.play_buffer(VoiceFormat::Mono16BitLE, 44100, (155, 155), &mut select);
        }

        background_rect.render(&mut graphics).unwrap();

        central_ellipse.render(&mut graphics).unwrap();

        corner_tri1.render(&mut graphics).unwrap();

        corner_tri2.render(&mut graphics).unwrap();

        corner_tri3.render(&mut graphics).unwrap();

        corner_tri4.render(&mut graphics).unwrap();

        poly_outline.render(&mut graphics).unwrap();

        hori_line.render(&mut graphics).unwrap();

        veri_line.render(&mut graphics).unwrap();

        hello_world.render(&mut graphics).unwrap();

        pointer.top_left = Vec2::new(wii.as_wpad().ir().0, wii.as_wpad().ir().1);
        pointer.render(&mut graphics).unwrap();

        graphics.flush();
    }
}
