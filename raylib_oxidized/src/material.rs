use crate::{consts::MaterialMapIndex, texture2d::Texture2D};

pub struct Material<'a> {
    raw: &'a mut raylib_ffi::Material,
}

impl<'a> Material<'a> {
    /// # Safety
    /// 'ptr' must be valid (duh)
    pub unsafe fn from_raw_mut(ptr: *mut raylib_ffi::Material) -> Self {
        unsafe { Self { raw: &mut *ptr } }
    }

    /// Change the underlying texture for a specific map
    pub fn set_texture(&mut self, map_type: MaterialMapIndex, texture: Texture2D) {
        // Raylib hardcodes MAX_MATERIAL_MAPS to 12 in raylib.h
        const MAX_MATERIAL_MAPS: usize = 12;

        unsafe {
            let maps_slice = std::slice::from_raw_parts_mut(self.raw.maps, MAX_MATERIAL_MAPS);

            maps_slice[map_type as usize].texture = texture;
        }
    }
}
