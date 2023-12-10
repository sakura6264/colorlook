use image::{RgbImage, RgbaImage};

/// Struct representing an image data.
#[derive(Debug)]
pub struct ImageData {
    width: u32,
    height: u32,
    channels: u8,
    data: Vec<u8>,
}

impl ImageData {
    /// Returns the width of the image data.
    ///
    /// # Returns
    /// The width of the image data.
    #[allow(unused)]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image data.
    ///
    /// # Returns
    /// The height of the image data.
    #[allow(unused)]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the number of channels of the image data.
    ///
    /// # Returns
    /// The number of channels of the image data.
    #[allow(unused)]
    pub fn channels(&self) -> u8 {
        self.channels
    }

    /// Returns the raw data of the image data.
    ///
    /// # Returns
    /// The raw data of the image data.
    #[allow(unused)]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl From<&RgbImage> for ImageData {
    #[allow(unused)]
    fn from(value: &RgbImage) -> Self {
        let (width, height) = value.dimensions();
        Self {
            width,
            height,
            channels: 3,
            data: value.to_vec(),
        }
    }
}

impl From<&RgbaImage> for ImageData {
    #[allow(unused)]
    fn from(value: &RgbaImage) -> Self {
        let (width, height) = value.dimensions();
        Self {
            width,
            height,
            channels: 4,
            data: value.to_vec(),
        }
    }
}