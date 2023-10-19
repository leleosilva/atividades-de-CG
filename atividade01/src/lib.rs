use std::fs::create_dir_all;
use image;

// Gerenciador que salva imagens no diretório de armazenamento definido por 'save_path'
pub struct ImgManager {
    save_path: String,
}

impl ImgManager {

    // Cria e retorna uma nova instância do ImgManager e um novo diretório caso ele ainda não exista
    pub fn new(path: String) -> Self {
        create_dir_all(&path).expect("Não foi possível criar o diretório desejado.");
        Self { save_path: path }
    }

    // Recebe uma imagem RGB e a salva em 'save_path', no formato PNG
    pub fn save_png(&self, img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, img_name: &str)
        -> Result<(), image::ImageError> {

        let mut path = self.save_path.to_owned();
        path.push_str(img_name);
        img.save_with_format(path, image::ImageFormat::Png)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /* Cria uma imagem no estilo gradiente de nome 'gradient_img.png',
     * verificando se foi possível salvá-la corretamente */
    fn test_create_gradient_img() {
        let img_manager = ImgManager::new(String::from("./imgs/"));

        const IMAGE_WIDTH: u32 = 256;
        const IMAGE_HEIGHT: u32 = 256;

        let mut img: image::RgbImage = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

        for j in 0..IMAGE_HEIGHT {
            for i in 0..IMAGE_WIDTH {

                let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
                let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
                let b = 0.67;

                let ir = (255.999 * r) as u8;
                let ig = (255.999 * g) as u8;
                let ib = (255.999 * b) as u8;

                img.put_pixel(i, j, image::Rgb([ir, ig, ib]));
            }
        }
        assert_eq!(img_manager.save_png(img, "gradient_img.png").is_ok(), true);
    }
}