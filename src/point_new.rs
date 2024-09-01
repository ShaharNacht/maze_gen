use std::fmt::{self, Debug};
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Sub};

use sdl2::rect::Point as SdlPoint;
use sdl2::video::Window;

use crate::layout::Layout;
use crate::maze::Maze;

pub struct Point<S: Space> {
    pub x: S::Number,
    pub y: S::Number,
}

pub trait Space {
    type Number: Copy
        + Add<Output = Self::Number>
        + Sub<Output = Self::Number>
        + Mul<Output = Self::Number>
        + Div<Output = Self::Number>;
}

impl Space for Maze {
    type Number = usize;
}

impl Space for Window {
    type Number = i32;
}

impl Space for Layout {
    type Number = f64;
}

pub trait ConvertPoint<I: Space, O: Space> {
    fn convert_point(&self, input: Point<I>) -> Point<O>;
}

impl<S: Space> Debug for Point<S>
where
    S::Number: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<S: Space> Clone for Point<S>
where
    S::Number: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<S: Space> Copy for Point<S> where S::Number: Copy {}

impl<S: Space> PartialEq for Point<S>
where
    S::Number: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<S: Space> Eq for Point<S> where S::Number: Eq {}

impl<S: Space> Hash for Point<S>
where
    S::Number: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<S: Space> Point<S> {
    pub fn new(x: S::Number, y: S::Number) -> Self {
        Self { x, y }
    }
}

impl<S: Space> From<(S::Number, S::Number)> for Point<S> {
    fn from((x, y): (S::Number, S::Number)) -> Self {
        Self::new(x, y)
    }
}
impl<S: Space> From<Point<S>> for (S::Number, S::Number) {
    fn from(value: Point<S>) -> Self {
        (value.x, value.y)
    }
}

impl From<Point<Window>> for SdlPoint {
    fn from(value: Point<Window>) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<SdlPoint> for Point<Window> {
    fn from(value: SdlPoint) -> Self {
        Self::new(value.x(), value.y())
    }
}

impl<S: Space, T: Into<Point<S>>> Add<T> for Point<S> {
    type Output = Point<S>;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();

        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<S: Space, T: Into<Point<S>>> Sub<T> for Point<S> {
    type Output = Point<S>;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();

        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<S: Space, T: Into<Point<S>>> Mul<T> for Point<S> {
    type Output = Point<S>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();

        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<S: Space, T: Into<Point<S>>> Div<T> for Point<S> {
    type Output = Point<S>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();

        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Mul<<Maze as Space>::Number> for Point<Maze> {
    type Output = Point<Maze>;

    fn mul(self, rhs: <Maze as Space>::Number) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<<Window as Space>::Number> for Point<Window> {
    type Output = Point<Window>;

    fn mul(self, rhs: <Window as Space>::Number) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<<Layout as Space>::Number> for Point<Layout> {
    type Output = Point<Layout>;

    fn mul(self, rhs: <Layout as Space>::Number) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<<Maze as Space>::Number> for Point<Maze> {
    type Output = Point<Maze>;

    fn div(self, rhs: <Maze as Space>::Number) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<<Window as Space>::Number> for Point<Window> {
    type Output = Point<Window>;

    fn div(self, rhs: <Window as Space>::Number) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<<Layout as Space>::Number> for Point<Layout> {
    type Output = Point<Layout>;

    fn div(self, rhs: <Layout as Space>::Number) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
