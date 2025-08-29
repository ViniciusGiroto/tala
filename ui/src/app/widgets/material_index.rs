use egui::{ComboBox, Response, Ui};
use optics::{material::Material, prelude::MaterialIndex};

pub fn material_index_optional(
    ui: &mut Ui,
    value: &mut Option<MaterialIndex>,
    materials: &[Material],
    optional_name: impl AsRef<str>,
) -> Response {
    let mut picked = value.map_or(0, |v| v.get() as usize);

    let available_width = ui.available_width();

    let response = ComboBox::from_id_salt("material")
        .width(available_width)
        .show_index(ui, &mut picked, materials.len() + 1, |i| {
            if i == 0 {
                optional_name.as_ref()
            } else {
                materials[i - 1].name()
            }
        });

    *value = MaterialIndex::new(picked as u32);

    response
}

pub fn material_index(ui: &mut Ui, value: &mut MaterialIndex, materials: &[Material]) -> Response {
    let mut picked = value.get() as usize - 1;

    let available_width = ui.available_width();

    let response = ComboBox::from_id_salt("material")
        .width(available_width)
        .show_index(ui, &mut picked, materials.len(), |i| materials[i].name());

    // This is safe because picked + 1 is always > 0
    *value = MaterialIndex::new(picked as u32 + 1).unwrap();

    response
}
