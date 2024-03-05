use wgpu::{Device, TextureDescriptor, TextureView};

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
        device: Device,
        texture_decriptor: &TextureDescriptor,
    ) -> Reservation {
        let texture = device.create_texture(texture_decriptor);
        let view = texture.create_view(&Default::default());
        let id = self.textures.len();
        self.textures.push(Texture { texture, view });
        Reservation(InnerReservation::TextureID(TextureID { id }))
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
    pub fn resolve(&self, view: Reservation) -> &'a TextureView {
        match view.0 {
            InnerReservation::Surface => self.surface_view,
            InnerReservation::TextureID(i) => &self.store.textures[i.id].view,
        }
    }
}

pub struct Reservation(InnerReservation);

impl Reservation {
    pub fn get_surface() -> Self {
        Self {
            0: InnerReservation::Surface,
        }
    }
}

struct TextureID {
    id: usize,
}

enum InnerReservation {
    Surface,
    TextureID(TextureID),
}
