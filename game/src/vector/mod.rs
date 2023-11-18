use num::Num;

pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x,
            y,
        }
    }

    pub fn copy(&self) -> Vector2<T> {
        return Vector2::new(
            self.x,
            self.y);
    }
}

impl<T> Vector2<T> where
    T: Num,
    T: Copy,
{
    pub fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(
            self.x + other.x,
            self.y + other.y,
        )
    }

    pub fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(
            self.x - other.x,
            self.y - other.y,
        )
    }

    pub fn mul(self, other: Vector2<T>) -> Vector2<T> {
        Vector2::new(
            self.x * other.x,
            self.y * other.y,
        )
    }
}
