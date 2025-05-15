/// A rectangle structure.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Rect {
    /// X coordinate of the rectangle.
    pub x: i32,
    /// Y coordinate of the rectangle.
    pub y: i32,
    /// Width of the rectangle.
    pub width: i32,
    /// Height of the rectangle.
    pub height: i32,
}

/// A point structure.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Point {
    /// X coordinate of the point.
    pub x: i32,
    /// Y coordinate of the point.
    pub y: i32,
}

/// A size structure.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Size {
    /// Width of the size.
    pub width: u32,
    /// Height of the size.
    pub height: u32,
}
