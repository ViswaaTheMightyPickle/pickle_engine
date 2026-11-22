use std::path::Path;
use image::imageops::FilterType;
use crate::collision_importer::collision_importer_config::{CollisionImporterConfig, ResolutionMode};


#[derive(Debug)]
pub struct BinaryMask {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<bool>,
}


impl BinaryMask {
    // Binary collision 2d array for the image
    pub fn get(&self, x: u16, y: u16) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        self.pixels[(y as usize) * (self.width as usize) + (x as usize)]
    }

    //Converts image coordinates into a flat array index
    pub fn index(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }
    
}


pub enum ImageProcessingError {
    Image(image::ImageError),
    InvalidTheshold,
    InvalidResolution,
    InvalidScale,
    DimensionsTooLarge,
}

impl From<image::ImageError> for ImageProcessingError {
    fn from(err: image::ImageError) -> Self {
        Self::Image(err)
    }
}

//Detirmine desired image dimensions based on the config and the original image dimensions
fn calculate_dimensions(
    original_width: u32,
    original_height: u32,
    mode: ResolutionMode,
) -> Result<(u32, u32), ImageProcessingError> {
    let (width,height) = match mode {
        ResolutionMode::Dimensions {width, height} => {
            (width as u32, height as u32)
        }

        ResolutionMode::Scale {scale} => {
            if !scale.is_finite() || scale <= 0.0 {
                return Err(ImageProcessingError::InvalidScale);
            }

            let width = (original_width as f32 * scale).round();
            let height = (original_height as f32 * scale).round();

            if width < 1.0 || height < 1.0 {
                return Err(ImageProcessingError::InvalidResolution);
            }

            if width > u16::MAX as f32 || height > u16::MAX as f32 {
                return Err(ImageProcessingError::DimensionsTooLarge);
            }

            (width as u32, height as u32)
        }
    };

    if width == 0 || height == 0 {
        return Err(ImageProcessingError::InvalidResolution);
    }

    Ok((width, height))
}


// Load, resize and convert an image into a binary collision mask
pub fn load_collision_mask(
    path: impl AsRef<Path>,
    config: &CollisionImporterConfig,
) -> Result<BinaryMask, ImageProcessingError> {
    if !config.threshold.is_finite() || config.threshold < 0.0 || config.threshold > 1.0 {
        return Err(ImageProcessingError::InvalidTheshold);
    }

    //Load image
    let image = image::open(path)?;

    //Convert to grayscale just in case
    let grayscale = image.to_luma8();

    let original_width = grayscale.width();
    let original_height = grayscale.height();


    //Calculate output dimensions
    let (width, height) = calculate_dimensions(
        original_width,
        original_height,
        config.resolution_mode,
    )?;

    if width > u16::MAX as u32 || height > u16::MAX as u32 {
        return Err(ImageProcessingError::DimensionsTooLarge);
    }

    //resizing filter
    let filter = if width < original_width || height < original_height {
        FilterType::Lanczos3
    } else {
        FilterType::Triangle
    };

    //Resize image
    let resized = image::imageops::resize(
        &grayscale,
        width,
        height,
        filter,
    );

    //Convert grayscale into mesh
    let pixels = resized
        .pixels()
        .map(|pixel| {
            let intensity = pixel[0] as f32 / 255.0;

            intensity <= config.threshold
        })
        .collect();

    Ok(BinaryMask {
        width: width as u16,
        height: height as u16,
        pixels,
    })    
}