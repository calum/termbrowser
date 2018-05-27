use image;
use reqwest;
use image::ImageFormat;
use image::ImageError;
use image::DynamicImage;
use std::io::BufReader;

pub enum ClientError {
    Image(ImageError),
    Http(reqwest::Error),
    NotSupported
}

impl From<ImageError> for ClientError {
    fn from(error: ImageError) -> Self {
        ClientError::Image(error)
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError::Http(error)
    }
}

fn guess_format(name: &str) -> Result<ImageFormat,ClientError> {
    if name.ends_with(".png") {
        Ok(ImageFormat::PNG)
    } else if name.ends_with(".jpg") {
        Ok(ImageFormat::JPEG)
    } else if name.ends_with(".gif") {
        Ok(ImageFormat::GIF)
    } else if name.ends_with(".webp") {
        Ok(ImageFormat::WEBP)
    } else if name.ends_with(".tiff") {
        Ok(ImageFormat::TIFF)
    } else if name.ends_with(".bmp") {
        Ok(ImageFormat::BMP)
    } else {
        Err(ClientError::NotSupported)
    }
}

pub fn get_image(name: &str) -> Result<DynamicImage,ClientError> {
    Ok(image::load(
        BufReader::new(reqwest::get("https://commons.wikimedia.org/wiki/Special:FilePath/".to_string() + name)?),
        guess_format(name)?
    )?)
}
