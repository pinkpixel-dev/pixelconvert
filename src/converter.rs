use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};
use std::path::Path;

/// Supported image formats for conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedFormat {
    Png,
    Jpeg,
    WebP,
    Avif,
    Gif,
    Bmp,
    Tiff,
    Ico,
}

#[allow(dead_code)]
impl SupportedFormat {
    pub fn extension(&self) -> &str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::WebP => "webp",
            Self::Avif => "avif",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
            Self::Ico => "ico",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::WebP => "image/webp",
            Self::Avif => "image/avif",
            Self::Gif => "image/gif",
            Self::Bmp => "image/bmp",
            Self::Tiff => "image/tiff",
            Self::Ico => "image/x-icon",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Png => "PNG",
            Self::Jpeg => "JPEG",
            Self::WebP => "WebP",
            Self::Avif => "AVIF",
            Self::Gif => "GIF",
            Self::Bmp => "BMP",
            Self::Tiff => "TIFF",
            Self::Ico => "ICO",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Png,
            Self::Jpeg,
            Self::WebP,
            Self::Avif,
            Self::Gif,
            Self::Bmp,
            Self::Tiff,
            Self::Ico,
        ]
    }
}

/// Conversion quality/compression settings
#[derive(Debug, Clone)]
pub struct ConversionOptions {
    pub quality: u8, // 0-100
    pub format: SupportedFormat,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            quality: 85,
            format: SupportedFormat::Png,
        }
    }
}

/// Image converter
pub struct ImageConverter {
    options: ConversionOptions,
}

impl ImageConverter {
    pub fn new(options: ConversionOptions) -> Self {
        Self { options }
    }

    /// Load an image from a file path
    pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
        let img = image::open(path.as_ref()).context("Failed to open image")?;
        Ok(img)
    }

    /// Convert and save an image
    pub fn convert<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: Q,
    ) -> Result<()> {
        let img = Self::load_image(input_path)?;
        self.save_image(&img, output_path)
    }

    /// Save an image with the configured format and quality
    fn save_image<P: AsRef<Path>>(&self, img: &DynamicImage, output_path: P) -> Result<()> {
        let output_path = output_path.as_ref();

        match self.options.format {
            SupportedFormat::Png => {
                img.save_with_format(output_path, ImageFormat::Png)
                    .context("Failed to save PNG")?;
            }
            SupportedFormat::Jpeg => {
                img.save_with_format(output_path, ImageFormat::Jpeg)
                    .context("Failed to save JPEG")?;
            }
            SupportedFormat::WebP => {
                self.save_webp(img, output_path)?;
            }
            SupportedFormat::Avif => {
                self.save_avif(img, output_path)?;
            }
            SupportedFormat::Gif => {
                img.save_with_format(output_path, ImageFormat::Gif)
                    .context("Failed to save GIF")?;
            }
            SupportedFormat::Bmp => {
                img.save_with_format(output_path, ImageFormat::Bmp)
                    .context("Failed to save BMP")?;
            }
            SupportedFormat::Tiff => {
                img.save_with_format(output_path, ImageFormat::Tiff)
                    .context("Failed to save TIFF")?;
            }
            SupportedFormat::Ico => {
                img.save_with_format(output_path, ImageFormat::Ico)
                    .context("Failed to save ICO")?;
            }
        }

        Ok(())
    }

    /// Save as WebP with quality settings
    fn save_webp<P: AsRef<Path>>(&self, img: &DynamicImage, output_path: P) -> Result<()> {
        let output_path = output_path.as_ref();

        // Get quality setting
        let quality = self.options.quality as f32;

        // Create encoder from image
        let encoder = webp::Encoder::from_image(img)
            .map_err(|e| anyhow::anyhow!("Failed to create WebP encoder: {:?}", e))?;

        // Encode with quality setting
        let webp_data = encoder.encode(quality);

        // Write to file
        std::fs::write(output_path, &*webp_data).context("Failed to write WebP file")?;

        Ok(())
    }

    /// Save as AVIF with quality settings
    fn save_avif<P: AsRef<Path>>(&self, img: &DynamicImage, output_path: P) -> Result<()> {
        let output_path = output_path.as_ref();
        let (width, height) = (img.width(), img.height());

        // Convert image to RGBA (ravif works with RGBA)
        let rgba = img.to_rgba8();
        let pixels: Vec<rgb::RGBA8> = rgba
            .pixels()
            .map(|p| rgb::RGBA8::new(p[0], p[1], p[2], p[3]))
            .collect();

        // Create AVIF encoder with quality settings
        let quality = self.options.quality as f32;
        let speed = 6; // Balance between speed and compression (1-10, 6 is reasonable)

        let encoder = ravif::Encoder::new()
            .with_quality(quality)
            .with_alpha_quality(quality)
            .with_speed(speed)
            .with_num_threads(Some(
                std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(4),
            ));

        // Encode to AVIF
        let encoded = encoder
            .encode_rgba(ravif::Img::new(
                &pixels[..],
                width as usize,
                height as usize,
            ))
            .map_err(|e| anyhow::anyhow!("Failed to encode AVIF: {:?}", e))?;

        // Write to file
        std::fs::write(output_path, encoded.avif_file).context("Failed to write AVIF file")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_extensions() {
        assert_eq!(SupportedFormat::Png.extension(), "png");
        assert_eq!(SupportedFormat::WebP.extension(), "webp");
        assert_eq!(SupportedFormat::Avif.extension(), "avif");
    }
}
