use image;

// Gerenciador que salva imagens no diretório de armazenamento definido por 'save_path'
pub struct ImgManager {
    save_path: String,
}

impl ImgManager {

    // Cria uma nova instância do ImgManager
    pub fn new(path: String) -> Self {
        Self { save_path: path }
    }

    // Recebe uma imagem RGB e a salva em 'save_path'
    pub fn save_png(&self, img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>)
        -> Result<(), image::ImageError> {
        img.save_with_format(&self.save_path, image::ImageFormat::Png)
    }
    
}

fn main() {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /* Cria uma imagem no estilo gradiente de nome 'gradient_img.png',
     * verificando se foi possível salvá-la corretamente */
    fn test_create_gradient_img() {
        let img_manager = ImgManager::new(String::from("gradient_img.png"));

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
        assert_eq!(img_manager.save_png(img).is_ok(), true);
    }
}