use wgpu::{Device, TextureDescriptor, TextureFormat, TextureView};

pub struct TextureStore {
    textures: Vec<Texture>,
}

impl TextureStore {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
        }
    }
    pub fn get_resolver<'a>(&'a self, surface_view: &'a TextureView) -> TextureResolver<'a> {
        TextureResolver {
            store: &self,
            surface_view: surface_view,
        }
    }

    pub fn reserve(
        &mut self,
        device: &Device,
        texture_decriptor: &TextureDescriptor,
    ) -> TextureHandle {
        let texture = device.create_texture(texture_decriptor);
        let view = texture.create_view(&Default::default());
        let id = self.textures.len();
        self.textures.push(Texture { texture, view });
        TextureHandle(InnerTextureHandle::TextureID(TextureID { id }))
    }

    pub fn resolve_format(&self, view: TextureHandle) -> Option<TextureFormat> {
        match view.0 {
            InnerTextureHandle::Surface => None,
            InnerTextureHandle::TextureID(i) => Some(self.textures[i.id].texture.format()),
        }
    }
}

pub struct Texture {
    texture: wgpu::Texture,
    view: TextureView,
}

pub struct TextureResolver<'a> {
    store: &'a TextureStore,
    surface_view: &'a TextureView,
}

impl<'a> TextureResolver<'a> {
    pub fn resolve(&self, view: TextureHandle) -> &'a TextureView {
        match view.0 {
            InnerTextureHandle::Surface => self.surface_view,
            InnerTextureHandle::TextureID(i) => &self.store.textures[i.id].view,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TextureHandle(InnerTextureHandle);

impl TextureHandle {
    pub fn get_surface() -> Self {
        Self {
            0: InnerTextureHandle::Surface,
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct TextureID {
    id: usize,
}
#[derive(Debug, Copy, Clone)]
enum InnerTextureHandle {
    Surface,
    TextureID(TextureID),
}
