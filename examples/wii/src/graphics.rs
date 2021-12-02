

use core::{convert::Infallible, ffi::c_void, ptr::null_mut};

use ogc_rs::{
    ffi::{
        GX_TG_MTX2x4, GX_CLIP_ENABLE, GX_CLR_RGBA, GX_COLOR0A0, GX_DIRECT, GX_F32, GX_GM_1_0,
        GX_IDENTITY, GX_MAX_Z24, GX_NONE, GX_PASSCLR, GX_PF_RGBA6_Z24, GX_PNMTX0, GX_POS_XY,
        GX_RGBA8, GX_TEVSTAGE0, GX_TEXCOORD0, GX_TEXMAP0, GX_TEX_ST, GX_TG_TEX0, GX_VTXFMT0, GX_MODULATE,
    },
    mem_cached_to_uncached,
    prelude::Color as GXColor,
    prelude::*,
};

use void_gfx::{
    geometry::{Vertex, Color, Vec2},
    renderer::{DrawHint, Renderer}, renderable::Renderable,
};



pub struct Graphics {
    framebuf_idx: usize,
    framebufs: [*mut c_void; 2],
}

impl Graphics {
    pub fn new() -> Self {
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

    pub fn flush(&mut self) {
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

pub struct Image<'a> {
    pub top_left: Vec2,
    pub tint: Color,
    pub texture: Texture<'a>
}

impl<'a> Image<'a> {
    pub fn new(top_left: Vec2, tint: Color, texture: Texture<'a>) -> Self {
        Self {
            top_left,
            tint,
            texture
        }
    }
}

impl<'a> Renderable for Image<'a> {
    fn render<R>(&self, _renderer: &mut R) -> Result<(), R::Error>
    where
            R: Renderer {
        Gx::load_texture(&self.texture, GX_TEXMAP0.try_into().unwrap());
        
        Gx::set_tev_op(GX_TEVSTAGE0.try_into().unwrap(), GX_MODULATE.try_into().unwrap());
        Gx::set_vtx_desc(VtxAttr::Tex0, GX_DIRECT.try_into().unwrap());
        
        let color = self.tint.into_rgba8();
        Gx::begin(Primitive::Quads, GX_VTXFMT0.try_into().unwrap(), 4);
        
        Gx::position_2f32(self.top_left.x, self.top_left.y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);
        Gx::tex_coord_2f32(0.0, 0.0);
        
        Gx::position_2f32(self.top_left.x + self.texture.width() as f32, self.top_left.y);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);
        Gx::tex_coord_2f32(1.0, 0.0);         

        Gx::position_2f32(self.top_left.x + self.texture.width() as f32, self.top_left.y + self.texture.height() as f32);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);
        Gx::tex_coord_2f32(1.0, 1.0);       
        
        Gx::position_2f32(self.top_left.x, self.top_left.y + self.texture.height() as f32);
        Gx::color_4u8(color[0], color[1], color[2], color[3]);
        Gx::tex_coord_2f32(0.0, 1.0);

        Gx::set_tev_op(GX_TEVSTAGE0.try_into().unwrap(), GX_PASSCLR.try_into().unwrap());
        Gx::set_vtx_desc(VtxAttr::Tex0, GX_NONE.try_into().unwrap());
        Ok(())
    }
}
