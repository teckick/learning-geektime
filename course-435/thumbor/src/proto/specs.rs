#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSpec {
    #[prost(message, repeated, tag="1")]
    pub specs: ::prost::alloc::vec::Vec<Spec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spec {
    #[prost(oneof="spec::Data", tags="1, 2, 3, 4, 5")]
    pub data: ::core::option::Option<spec::Data>,
}
/// Nested message and enum types in `Spec`.
pub mod spec {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="1")]
        Resize(super::Resize),
        #[prost(message, tag="2")]
        Crop(super::Crop),
        #[prost(message, tag="3")]
        Fliph(super::Fliph),
        #[prost(message, tag="4")]
        Flipv(super::Flipv),
        #[prost(message, tag="5")]
        Watermark(super::Watermark),
    }
}
/// 图片改变大小
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resize {
    #[prost(uint32, tag="1")]
    pub width: u32,
    #[prost(uint32, tag="2")]
    pub height: u32,
    #[prost(enumeration="resize::ResizeType", tag="3")]
    pub rtype: i32,
}
/// Nested message and enum types in `Resize`.
pub mod resize {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResizeType {
        Normal = 0,
        SeamCarve = 1,
    }
}
/// 图片截取
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Crop {
    #[prost(uint32, tag="1")]
    pub x1: u32,
    #[prost(uint32, tag="2")]
    pub y1: u32,
    #[prost(uint32, tag="3")]
    pub x2: u32,
    #[prost(uint32, tag="4")]
    pub y2: u32,
}
/// 水平翻转
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fliph {
}
/// 垂直翻转
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flipv {
}
/// 水印
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watermark {
    #[prost(uint32, tag="1")]
    pub x: u32,
    #[prost(uint32, tag="2")]
    pub y: u32,
}
