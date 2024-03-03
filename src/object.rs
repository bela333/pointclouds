use wgpu::{RenderPass};

use crate::material::Material;

pub trait Object{
    fn update(&mut self);
    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>);
}

pub struct BasicObject{
    material: Material
}

impl BasicObject{
    pub fn new(material: Material) -> Self{
        Self{
            material
        }
    }
}

impl Object for BasicObject{

    fn update(&mut self){
        // Update the object
    }

    fn draw<'a>(&'a self, pass: &mut RenderPass<'a>){
        // Draw the object
        pass.set_pipeline(self.material.get_render_pipeline());
        pass.draw(0..3, 0..1);
        
    }
}