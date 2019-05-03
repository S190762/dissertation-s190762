use crate::{
    eng::{
        core::{
            context::Context,
        }
    }
};

use std::{
    cell::RefCell,
    io,
    fs::File,
    collections::HashMap,
    io::prelude::*,
    path::{
        Path,
        
        PathBuf,
    },
    rc::Rc,
    io::Cursor
};
use glium::texture::RawImage2d;
use glium::texture::Texture2d;
use image::ImageBuffer;

pub type RefResourceManager = Rc<RefCell<ResourceManager>>;

pub struct ImageResource {
   pub gl_texture: glium::texture::Texture2d
}

impl Clone for ImageResource {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct ResourceManager {
    images: HashMap<PathBuf, ImageResource>
}

unsafe impl Send for ResourceManager {}
unsafe impl Sync for ResourceManager {}

impl ResourceManager {
    pub fn load_imageres(&mut self, ctx: &mut Context, path: &Path) {
        /*if !self.images.contains_key(path) {
            let mut l_file = File::open(&path).unwrap();
            let mut l_contents_str = String::new();
            l_file.read_to_string(&mut l_contents_str).unwrap();
            let l_bytes = l_contents_str.as_bytes();
            let l_image = image::load(Cursor::new(l_bytes), image::PNG).unwrap().to_rgba();
            let l_dim = l_image.dimensions();
            let l_raw = glium::texture::RawImage2d::from_raw_rgba_reversed(&l_image.into_raw(), l_dim);
            let l_tex = glium::texture::Texture2d::new(&ctx.sdl_window, l_raw).unwrap();
            self.images.insert(path.to_owned(), ImageResource { gl_texture: l_tex });
        }*/
    }
    pub fn get_imageres(&mut self, path: &Path) -> Option<&ImageResource> {
        self.images.get(path).map_or(None, |tex| Some(&tex))
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        ResourceManager { images: HashMap::new() }
    }
}