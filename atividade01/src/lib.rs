use std::fs::create_dir_all;

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

        // Criação do background
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

    #[test]
    /* Cria uma imagem de nome 'circle_img.png' que possui diversos círculos,
     * e verifica se foi possível salvá-la corretamente */
    fn test_create_circle_img() {
        let img_manager = ImgManager::new(String::from("./imgs/"));

        const IMAGE_WIDTH: u32 = 512;
        const IMAGE_HEIGHT: u32 = 512;

        let mut img: image::RgbImage = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
        
        // Criação do background
        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {

                let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
                let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
                let b = 0.25;

                let ir = (255.999 * r) as u8;
                let ig = (255.999 * g) as u8;
                let ib = (255.999 * b) as u8;

                img.put_pixel(i, j, image::Rgb([ir, ig, ib]));
            }
        }
        
        // Criação dos círculos
        let circle1_img = imageproc::drawing::draw_filled_circle(
            &img,
            ((IMAGE_WIDTH / 2) as i32, (IMAGE_HEIGHT / 2) as i32),
            150,
            image::Rgb([225, 138, 40])
        );

        let circle2_img = imageproc::drawing::draw_filled_circle(
            &circle1_img,
            (100, 325),
            70,
            image::Rgb([84, 51, 175])
        );

        let circle3_img = imageproc::drawing::draw_filled_circle(
            &circle2_img,
            (380, 220),
            90,
            image::Rgb([175, 154, 51])
        );

        assert_eq!(img_manager.save_png(circle3_img, "circle_img.png").is_ok(), true);
    }

    #[test]
    /* Cria uma imagem de nome 'square_img.png' que possui diversos quadrados,
     * e verifica se foi possível salvá-la corretamente */
    fn test_create_square_img() {
        let img_manager = ImgManager::new(String::from("./imgs/"));

        const IMAGE_WIDTH: u32 = 350;
        const IMAGE_HEIGHT: u32 = 350;

        let mut img: image::RgbImage = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

        // Criação do background
        for j in 0..IMAGE_HEIGHT {
            for i in (0..IMAGE_WIDTH).rev() {

                let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
                let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
                let b = 0.0;

                let ir = (255.999 * r) as u8;
                let ig = (255.999 * g) as u8;
                let ib = (255.999 * b) as u8;

                img.put_pixel(i, j, image::Rgb([ir, ig, ib]));
            }
        }

        // Criação dos quadrados
        let square1_img = imageproc::drawing::draw_filled_rect(
            &img,
            imageproc::rect::Rect::at(70, 80).of_size(120, 120),
            image::Rgb([255, 0, 0])
        );

        let square2_img = imageproc::drawing::draw_filled_rect(
            &square1_img,
            imageproc::rect::Rect::at(170, 150).of_size(100, 100),
            image::Rgb([0, 255, 0])
        );

        let square3_img = imageproc::drawing::draw_filled_rect(
            &square2_img,
            imageproc::rect::Rect::at(130, 205).of_size(105, 105),
            image::Rgb([0, 0, 255])
        );

        assert_eq!(img_manager.save_png(square3_img, "square_img.png").is_ok(), true);
    }
}