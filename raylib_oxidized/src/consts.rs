#[repr(usize)]
pub enum MaterialMapIndex {
    Albedo = 0,
    Metalness = 1,
    Normal = 2,
    Roughness = 3,
    Occlusion = 4,
    Emission = 5,
    Height = 6,
    Cubemap = 7,
    Irradiance = 8,
    Prefilter = 9,
    Brdf = 10,
}
