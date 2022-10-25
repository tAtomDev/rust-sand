use rand::random;

use crate::world;

use self::element_api::ElementApi;

pub mod elements;
pub mod element_api;

pub type Grid = world::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Density(u8);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElementType {
    Empty,
    Sand,
    Water
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Element {
    pub color: ggez::graphics::Color,
    pub element_type: ElementType,
    pub frame: u8,
    pub density: Density,
    pub velocity: glam::Vec2
}

impl Element {
    pub const fn new(element_type: ElementType, color: ggez::graphics::Color, density: Density) -> Self {
        Self {
            color, element_type,
            frame: 0,
            density,
            velocity: glam::Vec2::ZERO
        }
    }
}

impl PartialEq<ElementType> for Element {
    fn eq(&self, other: &ElementType) -> bool {
        self.element_type == *other
    }
}

impl ElementType {
    pub fn update(&self, grid: &mut Grid, api: &mut ElementApi, x: i32, y: i32) {
        match *self {
            ElementType::Sand => elements::update_sand(self, grid, api, x, y),
            ElementType::Water => elements::update_water(self, grid, api, x, y),
            _ => { }
        }
    }

    fn can_move_to(&self, grid: &Grid, api: &mut ElementApi, x: i32, y: i32) -> bool {
        if !grid.within_bounds(x, y) {
            return false;
        }

        return grid.is_empty(x, y) || (api.element.density.0 > grid.get_element(x, y).density.0 && random())
    }
}