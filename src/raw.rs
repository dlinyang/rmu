/// The raw math type alias

/// R² 32bit float vector
pub type Vec2f = [f32;2];

/// R³ 32bit float vector
pub type Vec3f = [f32;3];

/// R⁴ 32bit float vector
pub type Vec4f = [f32;4];

/// 2x2 32bit float matrix
pub type Mat2f = [[f32;2];2];

/// 3x3 32bit float matrix
pub type Mat3f = [[f32;3];3];

/// 4x4 32bit float matrix
pub type Mat4f = [[f32;4];4];

/// 2 rank 32bit float Identity element matrix
pub const ID2F: Mat2f = [[1.0, 0.0]
                        ,[0.0, 1.0]];

/// 3 rank 32bit float Identity element matrix
pub const ID3F: Mat3f = [[1.0, 0.0, 0.0]
                        ,[0.0, 1.0, 0.0]
                        ,[0.0, 0.0, 1.0]];

/// 4 rank 32bit float Identity element matrix
pub const ID4F: Mat4f = [[1.0, 0.0, 0.0, 0.0]
                        ,[0.0, 1.0, 0.0, 0.0]
                        ,[0.0, 0.0, 1.0, 0.0]
                        ,[0.0, 0.0, 0.0, 1.0]];