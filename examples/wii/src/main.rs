#![no_std]
#![feature(start)]

extern crate alloc;

use core::{convert::Infallible, ffi::c_void, ptr::null_mut};

use ogc_rs::{
    ffi::{
        GX_TG_MTX2x4, GX_CLIP_ENABLE, GX_CLR_RGBA, GX_COLOR0A0, GX_DIRECT, GX_F32, GX_GM_1_0,
        GX_IDENTITY, GX_MAX_Z24, GX_NONE, GX_PASSCLR, GX_PF_RGBA6_Z24, GX_PNMTX0, GX_POS_XY,
        GX_RGBA8, GX_TEVSTAGE0, GX_TEXCOORD0, GX_TEXMAP0, GX_TEX_ST, GX_TG_TEX0, GX_VTXFMT0,
    },
    mem_cached_to_uncached,
    prelude::Color as GXColor,
    prelude::*,
};

use void_gfx::{
    geometry::{Color, Vec2, Vertex},
    primitives::{Ellipse, Line, Polyline, Rectangle, Text, Triangle},
    renderable::Renderable,
    renderer::{DrawHint, Renderer},
};

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn main() {
    let mut graphics = Graphics::new();

    Input::init(ControllerType::Gamecube);
    Input::init(ControllerType::Wii);

    let gcn = Input::new(ControllerType::Gamecube, ControllerPort::One);
    let wii = Input::new(ControllerType::Wii, ControllerPort::One);

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
        1,
        Color::new(1.0, 0.0, 1.0, 1.0),
    );

    let veri_line = Line::new(
        Vec2::new(640.0 / 2.0, 480.0 / 2.0 - 128.0),
        Vec2::new(640.0 / 2.0, 480.0 / 2.0 + 128.0),
        1,
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
    
    
    let mut pointer = Ellipse::new(Vec2::new(0.0, 0.0), Vec2::new(32.0, 32.0), Color::new(1.0, 0.0, 0.0, 1.0));

    const TRI_SIZE: f32 = 64.0;
    const POLYLINE_SIZE: f32 = 16.0;
    'main_loop: loop {
        Input::update(ControllerType::Gamecube);
        Input::update(ControllerType::Wii);
        wii.as_wpad().set_data_format(WPadDataFormat::ButtonsAccelIR);

        if gcn.is_button_down(Button::Start) {
            break 'main_loop;
        }

        if wii.is_button_down(Button::Home) {
            break 'main_loop;
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

pub struct Graphics {
    framebuf_idx: usize,
    framebufs: [*mut c_void; 2],
}

impl Graphics {
    fn new() -> Self {
        let mut framebuffers = [null_mut(); 2];
        let curr_fb = 0;

        let mut video = Video::init();
        Video::set_black(true);

        framebuffers[0] = video.framebuffer;
        framebuffers[1] =
            mem_cached_to_uncached!(System::allocate_framebuffer(&video.render_config));

        Console::init(&video);
        Video::configure(&video.render_config);

        unsafe {
            Video::set_next_framebuffer(framebuffers[curr_fb]);
        }
        Video::flush();
        Video::wait_vsync();

        Gx::init(1024 * 256);
        Gx::set_copy_clear(GXColor::with_alpha(0, 0, 0, 0), GX_MAX_Z24);
        Gx::set_pixel_fmt(GX_PF_RGBA6_Z24.try_into().unwrap(), ZCompress::Linear);

        let y_scale = Gx::get_y_scale_factor(
            video.render_config.embed_framebuffer_height,
            video.render_config.extern_framebuffer_height,
        );
        let extern_framebuffer_height: u16 = Gx::set_disp_copy_y_scale(y_scale).try_into().unwrap();
        let half_aspect_ratio =
            video.render_config.vi_height == 2 * video.render_config.extern_framebuffer_height;

        Gx::set_disp_copy_src(
            0,
            0,
            video.render_config.framebuffer_width,
            video.render_config.embed_framebuffer_height,
        );
        Gx::set_disp_copy_dst(
            video.render_config.framebuffer_width,
            extern_framebuffer_height,
        );
        Gx::set_copy_filter(
            video.render_config.anti_aliasing > 0,
            &mut video.render_config.sample_pattern as &mut _,
            true,
            &mut video.render_config.v_filter as &mut _,
        );
        Gx::set_field_mode(video.render_config.field_rendering > 0, half_aspect_ratio);
        Gx::set_disp_copy_gamma(GX_GM_1_0.try_into().unwrap());

        Gx::clear_vtx_desc();
        Gx::inv_vtx_cache();
        Gx::invalidate_tex_all();

        Gx::set_vtx_desc(VtxAttr::Pos, GX_DIRECT.try_into().unwrap());
        Gx::set_vtx_desc(VtxAttr::Color0, GX_DIRECT.try_into().unwrap());
        Gx::set_vtx_desc(VtxAttr::Tex0, GX_NONE.try_into().unwrap());

        Gx::set_vtx_attr_fmt(
            GX_VTXFMT0.try_into().unwrap(),
            VtxAttr::Pos,
            GX_POS_XY,
            GX_F32,
            0,
        );
        Gx::set_vtx_attr_fmt(
            GX_VTXFMT0.try_into().unwrap(),
            VtxAttr::Color0,
            GX_CLR_RGBA,
            GX_RGBA8,
            0,
        );

        Gx::set_vtx_attr_fmt(
            GX_VTXFMT0.try_into().unwrap(),
            VtxAttr::Tex0,
            GX_TEX_ST,
            GX_F32,
            0,
        );

        Gx::set_z_mode(false, CmpFn::LessEq, true);

        Gx::set_num_chans(1);
        Gx::set_num_tex_gens(1);
        Gx::set_tev_op(
            GX_TEVSTAGE0.try_into().unwrap(),
            GX_PASSCLR.try_into().unwrap(),
        );
        Gx::set_tev_order(
            GX_TEVSTAGE0.try_into().unwrap(),
            GX_TEXCOORD0.try_into().unwrap(),
            GX_TEXMAP0,
            GX_COLOR0A0.try_into().unwrap(),
        );
        Gx::set_tex_coord_gen(
            GX_TEXCOORD0.try_into().unwrap(),
            GX_TG_MTX2x4,
            GX_TG_TEX0,
            GX_IDENTITY,
        );

        let mut model_view = Mat3x4::IDENTITY;
        model_view.gu_translation_apply((0., 0., -100.0));
        model_view.load_as_pos_mtx(GX_PNMTX0);

        let mut ortho = Mat4::gu_ortho(
            0.,
            video.render_config.embed_framebuffer_height.into(),
            0.,
            video.render_config.framebuffer_width.into(),
            0.0,
            1000.0,
        );
        ortho.load_as_proj_mat(ProjectionType::Orthographic);

        Gx::set_viewport(
            0.,
            0.,
            video.render_config.framebuffer_width.into(),
            video.render_config.embed_framebuffer_height.into(),
            0.0,
            1.0,
        );
        Gx::set_blend_mode(
            BlendMode::Blend,
            BlendCtrl::SrcAlpha,
            BlendCtrl::InvSrcAlpha,
            LogicOp::Clear,
        );
        Gx::set_alpha_update(true);
        Gx::set_color_update(true);
        Gx::set_cull_mode(CullMode::None);

        Gx::set_clip_mode(GX_CLIP_ENABLE.try_into().unwrap());
        Gx::set_scissor(
            0,
            0,
            video.render_config.framebuffer_width.into(),
            video.render_config.embed_framebuffer_height.into(),
        );

        Video::set_black(false);

        Self {
            framebuf_idx: curr_fb,
            framebufs: framebuffers,
        }
    }

    fn flush(&mut self) {
        Gx::draw_done();
        Gx::invalidate_tex_all();

        self.framebuf_idx ^= 1;

        Gx::set_z_mode(true, CmpFn::LessEq, true);
        Gx::set_color_update(true);
        unsafe {
            Gx::copy_disp(self.framebufs[self.framebuf_idx], true);
        }
        Video::flush();
        Video::wait_vsync();
    }
    
    /*
    fn draw_line(&mut self, line: &Line) -> Result<(), Infallible> {
        let color = line.color.into_rgba8();
        Gx::begin(Primitive::Lines, GX_VTXFMT0.try_into().unwrap(), 2);

        Gx::position_2f32(line.start.x, line.start.y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::position_2f32(line.end.x, line.end.y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::end();

        Ok(())
    }

    fn draw_polyline(&mut self, poly: &Polyline) -> Result<(), Infallible> {
        let color = poly.color.into_rgba8();

        Gx::begin(
            Primitive::LineStrip,
            GX_VTXFMT0.try_into().unwrap(),
            poly.vertices.len() as u16,
        );

        for vertex in poly.vertices {
            Gx::position_2f32(vertex.x, vertex.y);
            Gx::color_4u8(color[0], color[1], color[2], color[3]);
        }

        Gx::end();

        Ok(())
    }

    fn draw_triangle(&mut self, tri: &Triangle) -> Result<(), Infallible> {
        let color = tri.color.into_rgba8();

        Gx::begin(Primitive::Triangles, GX_VTXFMT0.try_into().unwrap(), 3);

        Gx::position_2f32(tri.vertices[0].x, tri.vertices[0].y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::position_2f32(tri.vertices[1].x, tri.vertices[1].y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::position_2f32(tri.vertices[2].x, tri.vertices[2].y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::end();

        Ok(())
    }

    fn draw_rectangle(&mut self, rect: &Rectangle) -> Result<(), Infallible> {
        let color = rect.color.into_rgba8();
        Gx::begin(Primitive::Quads, GX_VTXFMT0.try_into().unwrap(), 4);

        Gx::position_2f32(rect.top_left.x, rect.top_left.x);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::position_2f32(rect.top_left.x + rect.size.x, rect.top_left.y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::position_2f32(rect.top_left.x + rect.size.x, rect.top_left.y + rect.size.y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::position_2f32(rect.top_left.x, rect.top_left.y + rect.size.y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        Gx::end();

        Ok(())
    }

    fn draw_ellipse(&mut self, ellipse: &Ellipse) -> Result<(), Infallible> {
        let color = ellipse.color.into_rgba8();
        const TWO_PI: f32 = core::f32::consts::PI * 2.0;

        const STEPS: u16 = 32;
        const VERT_COUNT: u16 = STEPS + 2;

        let x_radius = ellipse.size.x * 0.5;
        let y_radius = ellipse.size.y * 0.5;

        let center = Vec2::new(ellipse.top_left.x + x_radius, ellipse.top_left.y + y_radius);

        // Accounts for center vertex.
        Gx::begin(
            Primitive::TriangleFan,
            GX_VTXFMT0.try_into().unwrap(),
            VERT_COUNT + 1,
        );

        Gx::position_2f32(center.x, center.y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);

        for n in 0..VERT_COUNT {
            let curr_step = n as f32 * TWO_PI / STEPS as f32;
            let x = center.x + (x_radius * cosf(curr_step));
            let y = center.y + (y_radius * sinf(curr_step));

            Gx::position_2f32(x, y);
            Gx::color_4u8(color[0], color[1], color[2], color[3]);
        }

        Ok(())
    }

    fn draw_text(&mut self, text: &Text) -> Result<(), Infallible> {
        use rusttype::{point, Font, Scale};
        let font = Font::try_from_bytes(include_bytes!("../assets/font.ttf")).unwrap();
        let v_metrics = font.v_metrics(Scale::uniform(text.px_size));
        let glyphs: Vec<_> = font
            .layout(
                text.text,
                Scale::uniform(text.px_size),
                point(20., 20. + v_metrics.ascent),
            )
            .collect();

        let rgba8 = text.color.into_rgba8();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    if v > 0.2 {
                        Gx::begin(Primitive::Points, GX_VTXFMT0.try_into().unwrap(), 1);
                        Gx::position_2f32(
                            text.top_left.x + x as f32 + bounding_box.min.x as f32,
                            text.top_left.y + y as f32 + bounding_box.min.y as f32,
                        );
                        Gx::color_4u8(rgba8[0], rgba8[1], rgba8[2], ceilf(v * 255.0) as u8);
                        Gx::end();
                    }
                })
            }
        }

        Ok(())
    }
    */
}

impl Renderer for Graphics {
    type Error = Infallible;
    type Image = ();

    fn render_vertex_list(
        &mut self,
        vertices: &[Vertex],
        draw_hint: DrawHint,
    ) -> Result<(), Self::Error> {
        Gx::begin(
            draw_hint.into_prim(),
            GX_VTXFMT0.try_into().unwrap(),
            vertices.len().try_into().unwrap(),
        );

        for vertex in vertices {
            Gx::position_2f32(vertex.pos.x, vertex.pos.y);
            Gx::color_4f32(
                vertex.color.r,
                vertex.color.g,
                vertex.color.b,
                vertex.color.a,
            );
        }

        Gx::end();

        Ok(())
    }

    /*
    fn render_line(&mut self, line: &Line) -> Result<(), Self::Error> {
        self.draw_line(line)
    }

    fn render_polyline(&mut self, poly: &Polyline) -> Result<(), Self::Error> {
        self.draw_polyline(poly)
    }

    fn render_triangle(&mut self, tri: &Triangle) -> Result<(), Self::Error> {
        self.draw_triangle(tri)
    }

    fn render_rectangle(&mut self, rect: &Rectangle) -> Result<(), Self::Error> {
        self.draw_rectangle(rect)
    }

    fn render_ellipse(&mut self, ellipse: &Ellipse) -> Result<(), Self::Error> {
        self.draw_ellipse(ellipse)
    }

    fn render_text(&mut self, text: &Text) -> Result<(), Self::Error> {
        self.draw_text(text)
    }
    */
}

trait IntoPrim {
    fn into_prim(self) -> Primitive;
}

impl IntoPrim for DrawHint {
    fn into_prim(self) -> Primitive {
        match self {
            DrawHint::Lines => Primitive::Lines,
            DrawHint::Points => Primitive::Points,
            DrawHint::Triangles => Primitive::Triangles,
            DrawHint::LineStrip => Primitive::LineStrip,
            DrawHint::TriFan => Primitive::TriangleFan,
            DrawHint::TriStrip => Primitive::TriangleStrip,
            DrawHint::Quads => Primitive::Quads,
        }
    }
}
