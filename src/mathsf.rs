pub struct Vec3(f32, f32, f32);
pub struct Vec4(f32, f32, f32, f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self (x, y, z)
    }
    pub fn zero() -> Self {
        Self (0f32, 0f32, 0f32)
    }
    pub fn addition(v0: &Vec3, v1: &Vec3) -> Self {
        Self (v0.0 + v1.0, v0.1 + v1.1, v0.2 + v1.2)
    }
    pub fn subtraction(v0: &Vec3, v1: &Vec3) -> Self {
        Self (v0.0 - v1.0, v0.1 - v1.1, v0.2 - v1.2)
    }
    pub fn lerp(v0: &Vec3, v1: &Vec3, f: f32) -> Self {
        Self::addition(v0, Self::subtraction(v1, v0).multiply(f))
    }
    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.0 = x;
        self.1 = y;
        self.2 = z;
    }
    pub fn get_x(&self) -> f32 {
        self.0
    }
    pub fn get_y(&self) -> f32 {
        self.1
    }
    pub fn get_z(&self) -> f32 {
        self.2
    }
    pub fn floor_xy(&mut self) {
        self.0 = self.0.floor();
        self.1 = self.1.floor();
    }
    pub fn ceil_xy(&mut self) {
        self.0 = self.0.ceil();
        self.1 = self.1.ceil();
    }
    pub fn multiply(&mut self, f: f32) -> &mut Self {
        self.0 *= f;
        self.1 *= f;
        self.2 *= f;
        self
    }
    pub fn add(&mut self, v: &Vec3) -> &mut Self {
        self.0 += v.0;
        self.1 += v.1;
        self.2 += v.2;
        self
    }
    pub fn negate(&mut self) -> &mut Self {
        self.multiply(-1f32)
    }
    pub fn length(&self) -> f32 {
        (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self (x, y, z, w)
    }
    pub fn zero() -> Self {
        Self (0f32, 0f32, 0f32, 0f32)
    }
    pub fn addition(va: &Vec4, vb: &Vec4) -> Self {
        Self (va.0 + vb.0, va.1 + vb.1, va.2 + vb.2, va.3 + vb.3)
    }
    pub fn subtraction(va: &Vec4, vb: &Vec4) -> Self {
        Self (va.0 - vb.0, va.1 - vb.1, va.2 - vb.2, va.3 - vb.3)
    }
    pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
        self.0 = x;
        self.1 = y;
        self.2 = z;
        self.3 = w;
    }
    pub fn get_x(&self) -> f32 {
        self.0
    }
    pub fn get_y(&self) -> f32 {
        self.1
    }
    pub fn get_z(&self) -> f32 {
        self.2
    }
    pub fn get_w(&self) -> f32 {
        self.3
    }
    pub fn multiply(&mut self, f: f32) -> &mut Self {
        self.0 *= f;
        self.1 *= f;
        self.2 *= f;
        self.3 *= f;
        self
    }
    pub fn add(&mut self, v: &Vec4) -> &mut Self {
        self.0 += v.0;
        self.1 += v.1;
        self.2 += v.2;
        self.3 += v.3;
        self
    }
    pub fn negate(&mut self) -> &mut Self {
        self.multiply(-1f32)
    }
    pub fn length(&self) -> f32 {
        (self.0*self.0 + self.1*self.1 + self.2*self.2 + self.3*self.3).sqrt()
    }
}