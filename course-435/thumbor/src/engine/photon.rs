use std::convert::TryFrom;

use axum::body::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use lazy_static::lazy_static;
use photon_rs::{PhotonImage, multiple, native::open_image_from_bytes, transform::{self, SamplingFilter}};

use super::{Engine, SpecTransform};
use crate::proto::*;

lazy_static! {
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(data: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&data)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                Some(spec::Data::Fliph(ref v)) => self.transform(v),
                Some(spec::Data::Flipv(ref v)) => self.transform(v),
                Some(spec::Data::Watermark(ref v)) => self.transform(v),
                _ => {},
            }
        }
    }

    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

impl SpecTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = transform::resize(&mut self.0, op.width, op.height, SamplingFilter::Gaussian);
        self.0 = img;
    }
}

impl SpecTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        let img = transform::crop(&mut self.0, op.x1, op.y1, op.x2, op.y2);
        self.0 = img;
    }
}

impl SpecTransform<&Fliph> for Photon {
    fn transform(&mut self, op: &Fliph) {
        transform::fliph(&mut self.0);
    }
}

impl SpecTransform<&Flipv> for Photon {
    fn transform(&mut self, op: &Flipv) {
        transform::flipv(&mut self.0);
    }
}

impl SpecTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x, op.y);
    }
}

// 将图片转换成对应格式的二进制序列
fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dyn_image = DynamicImage::ImageRgba8(img_buffer);
    let mut buffer = Vec::with_capacity(32768);
    dyn_image.write_to(&mut buffer, format).unwrap();
    buffer
}
