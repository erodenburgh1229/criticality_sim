pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {
    pub fn new(_x: T, _y: T) -> Self {
        Self {x: _x, y: _y}
    }
}
