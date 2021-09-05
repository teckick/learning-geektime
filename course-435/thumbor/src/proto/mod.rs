use std::convert::TryFrom;

use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use prost::Message;

mod specs;
pub use specs::*;

use self::resize::ResizeType;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD)?;
        let image_spec = ImageSpec::decode(&data[..])?;
        Ok(image_spec)
    }
}

impl Spec {
    pub fn new_resize_seam_curve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width: width,
                height: height,
                rtype: resize::ResizeType::SeamCarve as i32,
            })),
        }
    }

    pub fn new_resize(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width: width,
                height: height,
                rtype: resize::ResizeType::Normal as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x: x, y: y })),
        }
    }

    pub fn new_flipv() -> Self {
        Self {
            data: Some(spec::Data::Flipv(Flipv {})),
        }
    }

    pub fn new_fliph() -> Self {
        Self {
            data: Some(spec::Data::Fliph(Fliph {})),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, convert::TryInto};

    use super::*;

    #[test]
    fn encoded_spec_should_be_decoded() {
        let spec1 = Spec::new_resize(600, 600);
        let spec2 = Spec::new_resize_seam_curve(500, 500);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
    }
}
