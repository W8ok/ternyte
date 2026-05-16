use sdl3::{
    image,
    keyboard::Keycode,
    pixels::Color,
    render::*,
    ttf,
    ttf::{Font, Sdl3TtfContext},
    video::WindowContext,
};

pub struct Resource {
    pub texture_creator: TextureCreator<WindowContext>,
    pub textures: Vec<Texture>,
    pub font: Option<Font<'static>>,
    pub ttf_context: Option<Sdl3TtfContext>,
}

impl Resource {
    pub fn new(canvas: &WindowCanvas) -> Self {
        let texture_creator = canvas.texture_creator();
        let font = None;
        let ttf_context = Some(ttf::init().unwrap());

        Self {
            texture_creator,
            textures: Vec::new(),
            font,
            ttf_context,
        }
    }

    #[inline]
    pub fn load_texture(&mut self, img_path: &str) {
        let mut texture =
            image::LoadTexture::load_texture(&self.texture_creator, img_path).unwrap();
        texture.set_scale_mode(ScaleMode::Nearest);
        self.textures.push(texture);
    }

    #[inline]
    pub fn load_font(&mut self, path: &str) {
        let ttf_context = self.ttf_context.as_ref().unwrap();
        let font = ttf_context.load_font(path, 8.0).unwrap();
        self.font = Some(font);
    }
}

