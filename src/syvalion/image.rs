pub fn read_image(name: &str) -> Vec<u8>
{
    let im = image::open(name).unwrap().to_rgb8();
    im.into_raw()
}

pub fn save_image(name: &str, data: Vec<u8>, width: u32, height: u32)
{
    image::save_buffer(name, &data, width, height, image::ColorType::Rgb8).unwrap();
}
