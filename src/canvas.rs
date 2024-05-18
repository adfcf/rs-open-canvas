use std::ops::Mul;

use super::mathsf::Vec4;

// format ABGR
pub struct Color(u32);

const ALPHA: u32 = 0b11111111_00000000_00000000_00000000u32;
const BLUE: u32  = 0b00000000_11111111_00000000_00000000u32;
const GREEN: u32 = 0b00000000_00000000_11111111_00000000u32;
const RED: u32   = 0b00000000_00000000_00000000_11111111u32;

impl Color {
    pub fn gray_shade(f: f32) -> Self {
        let f = (255.0 * f.clamp(0f32, 1f32)) as u8;
        Self::from_rgb(f, f, f)
    }
    pub fn from_vec4(v: &Vec4) -> Self {
        Self::from_rgba((v.get_x() * 255f32) as u8, (v.get_y() * 255f32) as u8, (v.get_z() * 255f32) as u8, (v.get_w() * 255f32) as u8)
    }
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let mut c = ALPHA;
        c |= (red as u32) << (8*0); 
        c |= (green as u32) << (8*1);
        c |= (blue as u32) << (8*2);
        c |= (alpha as u32) << (8*3);
        Color(c)
    }
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        let mut c = ALPHA;
        c |= (red as u32) << (8*0); 
        c |= (green as u32) << (8*1);
        c |= (blue as u32) << (8*2);
        Color(c)
    }
    pub fn get_red(&self) -> u8 {
        (self.0 >> (8 * 0)) as u8
    }
    pub fn get_green(&self) -> u8 {
        (self.0 >> (8 * 1)) as u8
    }
    pub fn get_blue(&self) -> u8 {
        (self.0 >> (8 * 2)) as u8
    }
    pub fn get_alpha(&self) -> u8 {
        (self.0 >> (8 * 3)) as u8
    }
    pub fn set_red(&mut self, value: u8) {
        let value = (value as u32) << 8*0;
        self.0 &= !RED;
        self.0 |= value;
    }
    pub fn set_green(&mut self, value: u8) {
        let value = (value as u32) << 8*1;
        self.0 &= !GREEN;
        self.0 |= value;
    }
    pub fn set_blue(&mut self, value: u8) {
        let value = (value as u32) << 8*2;
        self.0 &= !BLUE;
        self.0 |= value;
    }
    pub fn set_alpha(&mut self, value: u8) {
        let value = (value as u32) << 8*3;
        self.0 &= !ALPHA;
        self.0 |= value;
    }
    pub fn add(&mut self, other: &Color) -> &mut Self {
        self.set_red(self.get_red().wrapping_add(other.get_alpha()));
        self.set_green(self.get_green().wrapping_add(other.get_alpha()));
        self.set_blue(self.get_blue().wrapping_add(other.get_alpha()));
        self.set_alpha(self.get_alpha().wrapping_add(other.get_alpha()));
        self
    }
    pub fn saturating_add(&mut self, other: &Color)-> &mut Self  {
        self.set_red(self.get_red().saturating_add(other.get_alpha()));
        self.set_green(self.get_green().saturating_add(other.get_alpha()));
        self.set_blue(self.get_blue().saturating_add(other.get_alpha()));
        self.set_alpha(self.get_alpha().saturating_add(other.get_alpha()));
        self
    }
    pub fn multiply(&mut self, other: &Color) -> &mut Self {
        self.set_red(self.get_red().mul(other.get_alpha()) / 255);
        self.set_green(self.get_green().mul(other.get_alpha()) / 255);
        self.set_blue(self.get_blue().mul(other.get_alpha()) / 255);
        self.set_alpha(self.get_alpha().mul(other.get_alpha()) / 255);
        self
    }
    pub fn complement_rgba(&self) -> Color {
        Self::from_rgba(
            255 - self.get_red(), 
            255 - self.get_green(), 
            255 - self.get_blue(), 
            255 - self.get_alpha())
    }
    pub fn complement(&self) -> Color {
        Self::from_rgb(
            255 - self.get_red(), 
            255 - self.get_green(), 
            255 - self.get_blue()
        )
    }
    pub fn black() -> Self {
        Self::from_rgb(u8::MIN, u8::MIN, u8::MIN)
    }
    pub fn white() -> Self {
        Self::from_rgb(u8::MAX, u8::MAX, u8::MAX)
    }
    pub fn gray() -> Self {
        Self::from_rgb(u8::MAX >> 1, u8::MAX >> 1, u8::MAX >> 1)
    }
    pub fn red() -> Self {
        Self::from_rgb(u8::MAX, u8::MIN, u8::MIN)
    }
    pub fn green() -> Self {
        Self::from_rgb(u8::MIN, u8::MAX, u8::MIN)
    }
    pub fn blue() -> Self {
        Self::from_rgb(u8::MIN, u8::MIN, u8::MAX)
    }
    pub fn yellow() -> Self {
        Self::from_rgb(u8::MAX, u8::MAX, u8::MIN)
    }
    pub fn magenta() -> Self {
        Self::from_rgb(u8::MAX, u8::MIN, u8::MAX)
    }
    pub fn cyan() -> Self {
        Self::from_rgb(u8::MIN, u8::MAX, u8::MAX)
    }
}

pub struct Canvas {
    depth_buffer: Vec<f32>,
    color_buffer: Vec<u32>,
    width: u32,
    height: u32,
}

impl Canvas {

    pub fn new(width: u32, height: u32) -> Canvas {
        let vec: Vec<u32> = vec![u32::MAX; (width * height) as usize];
        let buf: Vec<f32> = vec![f32::INFINITY; (width * height) as usize];
        Canvas {
            depth_buffer: buf,
            color_buffer: vec,
            width,
            height,
        }
    }

    pub fn fill_ij<F>(&mut self, filling_function: F) 
    where F: Fn(i32, i32) -> Color
    {
        for j in 0..self.get_height() {
            for i in 0..self.get_width() {
                self.put_pixel(i, j, filling_function(i, j));
            }
        }
    }

    pub fn put_pixel(&mut self, i: i32, j: i32, color: Color) {
        self.draw_pixel(i, j, f32::NAN, color);
    }

    pub fn draw_pixel(&mut self, i: i32, j: i32, z: f32, color: Color) {

        let i =  i + self.get_width() / 2;
        let j =  -j + self.get_height() / 2;
        let index = (i + (j * self.get_width())) as usize;

        // depth test fail
        if self.depth_buffer[index] <= z {
            return;
        } else {
            self.depth_buffer[index] = z;
        }

        self.color_buffer[index] = color.0;

    }

    pub fn clear_color(&mut self, color: Color) {
        self.color_buffer.fill_with(|| color.0);
    }

    pub fn clear_depth(&mut self, depth: f32) {
        self.depth_buffer.fill_with(|| depth);
    }

    pub fn get_width(&self) -> i32 {
        self.width as i32
    }

    pub fn get_height(&self) -> i32 {
        self.height as i32
    }

    pub fn raw(&self) -> *const u32 {
        self.color_buffer.as_ptr()
    }

}