#[repr(C)]
pub struct Matrix([f32; 9]);

impl Matrix {
    /// Build a new (identity) matrix, which applies no transformations.
    pub fn new() -> Self {
        Matrix([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ])
    }

    /// Multiplies this matrix (`self`) with another one (`mat`), firther from the initial vertex position, so the resulting transformation will be the chaining of both matrices' transformations: first `self`, then `mat`.
    pub fn mult(&mut self, mat: Matrix) {
        *self = Matrix([
            // Swapped from this: [as, in our glsl code, Position is the right-most element multiplied. So, for `self` to apply its transformation before mat, it must be the right element of this matrix multiplication.
            // self.0[0] * mat.0[0] + self.0[1] * mat.0[3] + self.0[2] * mat.0[6]  , self.0[0] * mat.0[1] + self.0[1] * mat.0[4] + self.0[2] * mat.0[7]  , self.0[0] * mat.0[2] + self.0[1] * mat.0[5] + self.0[2] * mat.0[8]  , 
            // self.0[3] * mat.0[0] + self.0[4] * mat.0[3] + self.0[5] * mat.0[6]  , self.0[3] * mat.0[1] + self.0[4] * mat.0[4] + self.0[5] * mat.0[7]  , self.0[3] * mat.0[2] + self.0[4] * mat.0[5] + self.0[5] * mat.0[8]  , 
            // self.0[6] * mat.0[0] + self.0[7] * mat.0[3] + self.0[8] * mat.0[6]  , self.0[6] * mat.0[1] + self.0[7] * mat.0[4] + self.0[8] * mat.0[7]  , self.0[6] * mat.0[2] + self.0[7] * mat.0[5] + self.0[8] * mat.0[8]  , 
            mat.0[0] * self.0[0] + mat.0[1] * self.0[3] + mat.0[2] * self.0[6]  , mat.0[0] * self.0[1] + mat.0[1] * self.0[4] + mat.0[2] * self.0[7]  , mat.0[0] * self.0[2] + mat.0[1] * self.0[5] + mat.0[2] * self.0[8]  , 
            mat.0[3] * self.0[0] + mat.0[4] * self.0[3] + mat.0[5] * self.0[6]  , mat.0[3] * self.0[1] + mat.0[4] * self.0[4] + mat.0[5] * self.0[7]  , mat.0[3] * self.0[2] + mat.0[4] * self.0[5] + mat.0[5] * self.0[8]  , 
            mat.0[6] * self.0[0] + mat.0[7] * self.0[3] + mat.0[8] * self.0[6]  , mat.0[6] * self.0[1] + mat.0[7] * self.0[4] + mat.0[8] * self.0[7]  , mat.0[6] * self.0[2] + mat.0[7] * self.0[5] + mat.0[8] * self.0[8]  , 
        ]);
    }

    /// Add a scale transformation to the matrix, for each axis.
    /// The scale center is (0.0, 0.0) by default.
    pub fn scale(&mut self, x_scale: f32, y_scale: f32) {
        self.mult(Matrix([
            x_scale ,   0.0     ,   0.0 , 
            0.0     ,   y_scale ,   0.0 , 
            0.0     ,   0.0     ,   1.0 , 
        ]));
    }

    /// Add a rotation transformation to the matrix.
    /// The rotation center is (0.0, 0.0) by default.
    pub fn rotate(&mut self, angle: f32) {
        self.mult(Matrix([
            angle.cos() ,   angle.sin() ,   0.0 , 
            -angle.sin(),   angle.cos() ,   0.0 , 
            0.0         ,   0.0         ,   1.0 , 
        ]));
    }

    /// Add a translation transformation to the matrix.
    pub fn translate(&mut self, x_move: f32, y_move: f32) {
        self.mult(Matrix([
            1.0 ,   0.0 ,   x_move  , 
            0.0 ,   1.0 ,   y_move  , 
            0.0 ,   0.0 ,   1.0     , 
        ]));
    }
}

impl Into<*const f32> for Matrix {
    fn into(self) -> *const f32 {
        self.0.as_ptr()
    }
}