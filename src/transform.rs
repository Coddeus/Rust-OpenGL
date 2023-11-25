/// A 3x3 matrix
#[repr(C)]
pub struct Mat3([f32; 9]);

impl Mat3 {
    /// Build a new (identity) Mat3, which applies no transformations.
    pub fn new() -> Self {
        Mat3([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ])
    }

    /// Multiplies this Mat3 (`self`) with another one (`mat`), further from the initial vertex position vector, so the resulting transformation will be the chaining of both matrices' transformations: first `self`, then `mat`.
    pub fn mult(&mut self, mat: Mat3) {
        *self = Mat3([
            // Swapped from this: [as, in our glsl code, Position is the right-most element multiplied. So, for `self` to apply its transformation before mat, it must be the right element of this Mat3 multiplication.
            // self.0[0] * mat.0[0] + self.0[1] * mat.0[3] + self.0[2] * mat.0[6]  , self.0[0] * mat.0[1] + self.0[1] * mat.0[4] + self.0[2] * mat.0[7]  , self.0[0] * mat.0[2] + self.0[1] * mat.0[5] + self.0[2] * mat.0[8]  , 
            // self.0[3] * mat.0[0] + self.0[4] * mat.0[3] + self.0[5] * mat.0[6]  , self.0[3] * mat.0[1] + self.0[4] * mat.0[4] + self.0[5] * mat.0[7]  , self.0[3] * mat.0[2] + self.0[4] * mat.0[5] + self.0[5] * mat.0[8]  , 
            // self.0[6] * mat.0[0] + self.0[7] * mat.0[3] + self.0[8] * mat.0[6]  , self.0[6] * mat.0[1] + self.0[7] * mat.0[4] + self.0[8] * mat.0[7]  , self.0[6] * mat.0[2] + self.0[7] * mat.0[5] + self.0[8] * mat.0[8]  , 
            mat.0[0] * self.0[0] + mat.0[1] * self.0[3] + mat.0[2] * self.0[6]  , mat.0[0] * self.0[1] + mat.0[1] * self.0[4] + mat.0[2] * self.0[7]  , mat.0[0] * self.0[2] + mat.0[1] * self.0[5] + mat.0[2] * self.0[8]  , 
            mat.0[3] * self.0[0] + mat.0[4] * self.0[3] + mat.0[5] * self.0[6]  , mat.0[3] * self.0[1] + mat.0[4] * self.0[4] + mat.0[5] * self.0[7]  , mat.0[3] * self.0[2] + mat.0[4] * self.0[5] + mat.0[5] * self.0[8]  , 
            mat.0[6] * self.0[0] + mat.0[7] * self.0[3] + mat.0[8] * self.0[6]  , mat.0[6] * self.0[1] + mat.0[7] * self.0[4] + mat.0[8] * self.0[7]  , mat.0[6] * self.0[2] + mat.0[7] * self.0[5] + mat.0[8] * self.0[8]  , 
        ]);
    }

    /// Add a scale transformation to the Mat3, for each axis.
    /// The scale center is (0.0, 0.0).
    pub fn scale(&mut self, x_scale: f32, y_scale: f32) {
        self.mult(Mat3([
            x_scale ,   0.0     ,   0.0 , 
            0.0     ,   y_scale ,   0.0 , 
            0.0     ,   0.0     ,   1.0 , 
        ]));
    }

    /// Add a rotation transformation to the Mat3, around (0.0, 0.0).
    pub fn rotate(&mut self, angle: f32) {
        self.mult(Mat3([
            angle.cos() ,   angle.sin() ,   0.0 , 
            -angle.sin(),   angle.cos() ,   0.0 , 
            0.0         ,   0.0         ,   1.0 , 
        ]));
    }

    /// Add a rotation transformation to the Mat3, around (x, y).
    pub fn rotate_around(&mut self, angle: f32, x: f32, y: f32) {
        self.translate(-x, -y);
        self.rotate(angle);
        self.translate(x, y);
    }

    /// Add a translation transformation to the Mat3.
    pub fn translate(&mut self, x_move: f32, y_move: f32) {
        self.mult(Mat3([
            1.0 ,   0.0 ,   x_move  , 
            0.0 ,   1.0 ,   y_move  , 
            0.0 ,   0.0 ,   1.0     , 
        ]));
    }
}

impl Into<*const f32> for Mat3 {
    fn into(self) -> *const f32 {
        self.0.as_ptr()
    }
}

/// A 4x4 matrix
#[repr(C)]
pub struct Mat4([f32; 16]);

impl Mat4 {
    /// Build a new (identity) Mat4, which applies no transformations.
    pub fn new() -> Self {
        Mat4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    /// Multiplies this Mat4 (`self`) with another one (`mat`), further from the initial vertex position vector, so the resulting transformation will be the chaining of both matrices' transformations: first `self`, then `mat`.
    pub fn mult(&mut self, mat: Mat4) {
        *self = Mat4([
            mat.0[0]  * self.0[0] + mat.0[1]  * self.0[4] + mat.0[2]  * self.0[8] + mat.0[3]  * self.0[12]  , mat.0[0]  * self.0[1] + mat.0[1]  * self.0[5] + mat.0[2]  * self.0[9] + mat.0[3]  * self.0[13]  , mat.0[0] * self.0[2]  + mat.0[1] * self.0[6]  + mat.0[2] * self.0[10]  + mat.0[3] * self.0[14]   , mat.0[0] * self.0[3]  + mat.0[1] * self.0[7]  + mat.0[2] * self.0[11]  + mat.0[3] * self.0[15], 
            mat.0[4]  * self.0[0] + mat.0[5]  * self.0[4] + mat.0[6]  * self.0[8] + mat.0[7]  * self.0[12]  , mat.0[4]  * self.0[1] + mat.0[5]  * self.0[5] + mat.0[6]  * self.0[9] + mat.0[7]  * self.0[13]  , mat.0[4] * self.0[2]  + mat.0[5] * self.0[6]  + mat.0[6] * self.0[10]  + mat.0[7] * self.0[14]   , mat.0[4] * self.0[3]  + mat.0[5] * self.0[7]  + mat.0[6] * self.0[11]  + mat.0[7] * self.0[15], 
            mat.0[8]  * self.0[0] + mat.0[9]  * self.0[4] + mat.0[10] * self.0[8] + mat.0[11] * self.0[12]  , mat.0[8]  * self.0[1] + mat.0[9]  * self.0[5] + mat.0[10] * self.0[9] + mat.0[11] * self.0[13]  , mat.0[8] * self.0[2]  + mat.0[9] * self.0[6]  + mat.0[10] * self.0[10] + mat.0[11] * self.0[14]  , mat.0[8] * self.0[3]  + mat.0[9] * self.0[7]  + mat.0[10] * self.0[11] + mat.0[11] * self.0[15], 
            mat.0[12] * self.0[0] + mat.0[13] * self.0[4] + mat.0[14] * self.0[8] + mat.0[15] * self.0[12]  , mat.0[12] * self.0[1] + mat.0[13] * self.0[5] + mat.0[14] * self.0[9] + mat.0[15] * self.0[13]  , mat.0[12] * self.0[2] + mat.0[13] * self.0[6] + mat.0[14] * self.0[10] + mat.0[15] * self.0[14]  , mat.0[12] * self.0[3] + mat.0[13] * self.0[7] + mat.0[14] * self.0[11] + mat.0[15] * self.0[15], 
        ]);
    }

    /// Add a scale transformation to the Mat4, for each axis.
    /// The scale center is (0.0, 0.0, 0.0).
    pub fn scale(&mut self, x_scale: f32, y_scale: f32, z_scale: f32) {
        self.mult(Mat4([
            x_scale ,   0.0     ,   0.0     , 0.0 , 
            0.0     ,   y_scale ,   0.0     , 0.0 , 
            0.0     ,   0.0     ,   z_scale , 0.0 , 
            0.0     ,   0.0     ,   0.0     , 1.0 , 
        ]));
    }

    /// Add a rotation transformation to the Mat4 around the X axis.
    /// The rotation center is (0.0, 0.0, 0.0).
    pub fn rotate_x(&mut self, angle: f32) {
        self.mult(Mat4([
            1.0     ,   0.0         ,   0.0         , 0.0 , 
            0.0     ,   angle.cos() ,   angle.sin() , 0.0 , 
            0.0     ,   -angle.sin(),   angle.cos() , 0.0 , 
            0.0     ,   0.0         ,   0.0         , 1.0 , 
        ]));
    }

    /// Add a rotation transformation to the Mat4 around the Y axis.
    /// The rotation center is (0.0, 0.0, 0.0).
    pub fn rotate_y(&mut self, angle: f32) {
        self.mult(Mat4([
            angle.cos() ,   0.0 ,   angle.sin() , 0.0 , 
            0.0         ,   1.0 ,   0.0         , 0.0 , 
            -angle.sin(),   0.0 ,   angle.cos() , 0.0 , 
            0.0         ,   0.0 ,   0.0         , 1.0 , 
        ]));
    }

    /// Add a rotation transformation to the Mat4 around the Z axis.
    /// The rotation center is (0.0, 0.0, 0.0).
    pub fn rotate_z(&mut self, angle: f32) {
        self.mult(Mat4([
            angle.cos() ,   angle.sin() ,   0.0 ,   0.0 , 
            -angle.sin(),   angle.cos() ,   0.0 ,   0.0 , 
            0.0         ,   0.0         ,   1.0 ,   0.0 , 
            0.0         ,   0.0         ,   0.0 ,   1.0 , 
        ]));
    }

    /// Add a translation transformation to the Mat4.
    pub fn translate(&mut self, x_move: f32, y_move: f32, z_move: f32) {
        self.mult(Mat4([
            1.0 ,   0.0 ,   0.0 ,   x_move ,
            0.0 ,   1.0 ,   0.0 ,   y_move ,
            0.0 ,   0.0 ,   1.0 ,   z_move ,
            0.0 ,   0.0 ,   0.0 ,   1.0 ,
        ]));
    }
}

impl Into<*const f32> for Mat4 {
    fn into(self) -> *const f32 {
        self.0.as_ptr()
    }
}