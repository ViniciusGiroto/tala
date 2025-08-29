use glam::{Mat4, vec3};

use crate::{
    material::{Formula, Material},
    prelude::MaterialIndex,
    ray::Wavelength,
    surface::*,
};

#[derive(Debug)]
pub struct System {
    pub surfaces: Vec<Surface>,
    pub materials: Vec<Material>,
    pub stop_index: u32,
    pub medium: MaterialIndex,
    pub wavelengths: Vec<Wavelength>,
}

impl System {
    fn material(&self, index: MaterialIndex) -> Option<&Material> {
        self.materials.get(index.get() as usize - 1)
    }

    pub fn surfaces(&self) -> impl Iterator<Item = (&Surface, Mat4)> {
        self.surfaces.iter().map({
            let mut transform = Mat4::IDENTITY;

            move |surface| {
                let old_transform = transform;

                match surface.kind() {
                    SurfaceKind::Spherical => {
                        let data = surface.data();
                        transform *= Mat4::from_translation(vec3(0.0, 0.0, data[THICKNESS].into()));
                    }
                    SurfaceKind::CoordinateBreak => {
                        transform *= coordinate_break::transformation_matrix(surface.data());
                    }
                    _ => {}
                }

                (surface, old_transform)
            }
        })
    }
}

// Implementation of query methods
impl System {
    pub fn thickness(&self) -> f32 {
        self.surfaces()
            .map(|(surface, _)| match surface.kind() {
                SurfaceKind::Spherical => surface.data()[THICKNESS].into(),
                SurfaceKind::CoordinateBreak => surface.data()[TRANSLATION_Z].into(),
                _ => 0.0,
            })
            .sum()
    }

    /// Based on the equation
    /// $D = P_0 + P_1 + P_0 P_1 d_{0,1} / n_{0,1}$
    ///
    pub fn power(&self, wavelength: Wavelength) -> Option<f32> {
        let mut material = self.material(self.medium).unwrap();
        let mut n = material.refractive_index(wavelength);

        let mut surfaces = self.surfaces();

        // Object
        {
            let (surface, _) = surfaces.next()?;
            if let Some(material_index) = surface.data[MATERIAL_INDEX].into() {
                material = self
                    .material(material_index)
                    .expect("Surface has an undefined material");
                n = material.refractive_index(wavelength);
            }
        }

        let mut thickness = 0.0;
        let mut power = 0.0;

        for (surface, _) in surfaces {
            let prev_thickness = thickness;
            let prev_n = n;
            let prev_power = power;

            thickness = surface.data[THICKNESS].into();

            // material_index = None means "same as previous surface"
            if let Some(material_index) = surface.data[MATERIAL_INDEX].into() {
                material = self
                    .material(material_index)
                    .expect("Surface has an undefined material");
                n = material.refractive_index(wavelength);
            }

            match surface.kind() {
                SurfaceKind::Spherical => {
                    let curvature: f32 = surface.data[CURVATURE].into();
                    let curr_power = (n - prev_n) * curvature;

                    power =
                        prev_power + curr_power - prev_power * curr_power * prev_thickness / prev_n;
                }
                SurfaceKind::Image => return Some(power),
                // Other kinds of surface are not supported yet for power calculations
                _ => return None,
            }
        }

        None
    }
}

impl Default for System {
    fn default() -> Self {
        Self {
            surfaces: vec![
                Surface {
                    kind: crate::surface::SurfaceKind::Object,
                    data: crate::surface::SurfaceData::default()
                        .with(THICKNESS, 100.0)
                        .with(SEMI_DIAMETER, 10.0),
                },
                Surface {
                    kind: crate::surface::SurfaceKind::Spherical,
                    data: crate::surface::SurfaceData::default()
                        .with(MATERIAL_INDEX, MaterialIndex::new(3))
                        .with(THICKNESS, 10.0)
                        .with(CURVATURE, 1.0 / 100.0)
                        .with(SEMI_DIAMETER, 10.0),
                },
                Surface {
                    kind: crate::surface::SurfaceKind::Spherical,
                    data: crate::surface::SurfaceData::default()
                        .with(MATERIAL_INDEX, MaterialIndex::new(1))
                        .with(THICKNESS, 10.0)
                        .with(CURVATURE, 1.0 / -100.0)
                        .with(SEMI_DIAMETER, 10.0),
                },
                Surface {
                    kind: crate::surface::SurfaceKind::Image,
                    data: crate::surface::SurfaceData::default(),
                },
            ],
            materials: vec![
                Material::new("Vacuum".to_string(), Formula::Constant { cte: 1.0 }),
                Material::new(
                    "Air".to_string(),
                    Formula::Sellmeier4 {
                        a: 1.0,
                        k: [14926.44e-8, 41807.57e-8],
                        l: [19.36e-6, 7.434e-3],
                    },
                ),
                Material::new(
                    "N-BK7".to_string(),
                    Formula::Sellmeier1 {
                        k: [1.03961212, 0.231792344, 1.01046945],
                        l: [0.00600069867, 0.0200179144, 103.560653],
                    },
                ),
                Material::new(
                    "SF2".to_string(),
                    Formula::Sellmeier1 {
                        k: [1.40301821, 0.231767504, 0.939056586],
                        l: [0.0105795466, 0.0493226978, 112.405955],
                    },
                ),
            ],

            stop_index: 0,
            medium: MaterialIndex::new(1).unwrap(),
            wavelengths: vec![0.5875618],
        }
    }
}
