use std::path::PathBuf;
use anyhow::Result;
use image::DynamicImage;
use image::GenericImageView;
use image::Pixel;
use tokio::task::block_in_place;

pub struct ImageWriter;

impl ImageWriter {

    pub async fn download(folder: &PathBuf, game_id: &str, pokemon: &str, side: &str, side_url: &str) -> Result<()> {
        let response = reqwest::get(&format!("https://img.pokemondb.net/sprites/{}/{}/{}.png", game_id, side_url, pokemon)).await?;
        let bytes = response.bytes().await?.to_vec();
        block_in_place(move || {
            let mut image = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png).unwrap();
            let (top, bottom) = get_heights(&image);
            image = image.crop(0, top, image.width(), bottom - top + 1);
            image.save(folder.join(format!("normal_{}.png", side))).unwrap();
        });
        Ok(())
    }

}

fn get_heights(image: &DynamicImage) -> (u32, u32) {
    let mut top = 0;
    let mut bottom = image.height();

    for b_counter in 0..image.height() {
        if !transparent_row(image, b_counter) {
            top = b_counter;
            break;
        }
    }

    for t_counter in (0..image.height()).rev() {
        if !transparent_row(image, t_counter) {
            bottom = t_counter;
            break;
        }
    }

    (top, bottom)

}

fn transparent_row(image: &DynamicImage, y: u32) -> bool {
    for x in 0..image.width() {
        if !transparent(image, x, y) {
            return false;
        }
    }
    return true;
}

fn transparent(image: &DynamicImage, x: u32, y: u32) -> bool {
    image.get_pixel(x, y).channels()[3] == 0
}