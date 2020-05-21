/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Image_draw(
        arg1: *mut Fl_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Image_width(arg1: *mut Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_height(arg1: *mut Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_delete(arg1: *mut Fl_Image);
}
extern "C" {
    pub fn Fl_Image_count(self_: *mut Fl_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Image_data(self_: *mut Fl_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Image_copy(self_: *mut Fl_Image) -> *mut Fl_Image;
}
extern "C" {
    pub fn Fl_Image_scale(
        self_: *mut Fl_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_JPEG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_JPEG_Image_draw(
        arg1: *mut Fl_JPEG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_JPEG_Image_width(arg1: *mut Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_height(arg1: *mut Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_delete(arg1: *mut Fl_JPEG_Image);
}
extern "C" {
    pub fn Fl_JPEG_Image_count(self_: *mut Fl_JPEG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_JPEG_Image_data(self_: *mut Fl_JPEG_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_JPEG_Image_copy(self_: *mut Fl_JPEG_Image) -> *mut Fl_JPEG_Image;
}
extern "C" {
    pub fn Fl_JPEG_Image_scale(
        self_: *mut Fl_JPEG_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_JPEG_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_JPEG_Image;
}
extern "C" {
    pub fn Fl_JPEG_Image_from(data: *const ::std::os::raw::c_uchar) -> *mut Fl_JPEG_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_PNG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_PNG_Image_draw(
        arg1: *mut Fl_PNG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_PNG_Image_width(arg1: *mut Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_height(arg1: *mut Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_delete(arg1: *mut Fl_PNG_Image);
}
extern "C" {
    pub fn Fl_PNG_Image_count(self_: *mut Fl_PNG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_PNG_Image_data(self_: *mut Fl_PNG_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_PNG_Image_copy(self_: *mut Fl_PNG_Image) -> *mut Fl_PNG_Image;
}
extern "C" {
    pub fn Fl_PNG_Image_scale(
        self_: *mut Fl_PNG_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_PNG_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_PNG_Image;
}
extern "C" {
    pub fn Fl_PNG_Image_from(
        data: *const ::std::os::raw::c_uchar,
        size: ::std::os::raw::c_int,
    ) -> *mut Fl_PNG_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_SVG_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_SVG_Image_draw(
        arg1: *mut Fl_SVG_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_Image_width(arg1: *mut Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_height(arg1: *mut Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_delete(arg1: *mut Fl_SVG_Image);
}
extern "C" {
    pub fn Fl_SVG_Image_count(self_: *mut Fl_SVG_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_SVG_Image_data(self_: *mut Fl_SVG_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_SVG_Image_copy(self_: *mut Fl_SVG_Image) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_scale(
        self_: *mut Fl_SVG_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_SVG_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_SVG_Image;
}
extern "C" {
    pub fn Fl_SVG_Image_from(data: *const ::std::os::raw::c_char) -> *mut Fl_SVG_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_BMP_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_BMP_Image_draw(
        arg1: *mut Fl_BMP_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_BMP_Image_width(arg1: *mut Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_height(arg1: *mut Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_delete(arg1: *mut Fl_BMP_Image);
}
extern "C" {
    pub fn Fl_BMP_Image_count(self_: *mut Fl_BMP_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_BMP_Image_data(self_: *mut Fl_BMP_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_BMP_Image_copy(self_: *mut Fl_BMP_Image) -> *mut Fl_BMP_Image;
}
extern "C" {
    pub fn Fl_BMP_Image_scale(
        self_: *mut Fl_BMP_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_BMP_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_BMP_Image;
}
extern "C" {
    pub fn Fl_BMP_Image_from(data: *const ::std::os::raw::c_uchar) -> *mut Fl_BMP_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_GIF_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_GIF_Image_draw(
        arg1: *mut Fl_GIF_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_GIF_Image_width(arg1: *mut Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_height(arg1: *mut Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_delete(arg1: *mut Fl_GIF_Image);
}
extern "C" {
    pub fn Fl_GIF_Image_count(self_: *mut Fl_GIF_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_GIF_Image_data(self_: *mut Fl_GIF_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_GIF_Image_copy(self_: *mut Fl_GIF_Image) -> *mut Fl_GIF_Image;
}
extern "C" {
    pub fn Fl_GIF_Image_scale(
        self_: *mut Fl_GIF_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_GIF_Image_new(filename: *const ::std::os::raw::c_char) -> *mut Fl_GIF_Image;
}
extern "C" {
    pub fn Fl_GIF_Image_from(data: *const ::std::os::raw::c_uchar) -> *mut Fl_GIF_Image;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_RGB_Image {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_RGB_Image_draw(
        arg1: *mut Fl_RGB_Image,
        X: ::std::os::raw::c_int,
        Y: ::std::os::raw::c_int,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_RGB_Image_width(arg1: *mut Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_height(arg1: *mut Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_delete(arg1: *mut Fl_RGB_Image);
}
extern "C" {
    pub fn Fl_RGB_Image_count(self_: *mut Fl_RGB_Image) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_RGB_Image_data(self_: *mut Fl_RGB_Image) -> *const *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_RGB_Image_copy(self_: *mut Fl_RGB_Image) -> *mut Fl_RGB_Image;
}
extern "C" {
    pub fn Fl_RGB_Image_scale(
        self_: *mut Fl_RGB_Image,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        proportional: ::std::os::raw::c_int,
        can_expand: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_RGB_Image_new(
        bits: *const ::std::os::raw::c_uchar,
        W: ::std::os::raw::c_int,
        H: ::std::os::raw::c_int,
        depth: ::std::os::raw::c_int,
    ) -> *mut Fl_RGB_Image;
}
