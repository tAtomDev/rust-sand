use super::{Element, ElementType, Grid, Density, element_api::ElementApi};

pub const EMPTY: Element = Element::new(ElementType::Empty, ggez::graphics::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, Density(0));
pub const SAND: Element = Element::new(ElementType::Sand, ggez::graphics::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 }, Density(200));
pub const WATER: Element = Element::new(ElementType::Water, ggez::graphics::Color { r: 0.1, g: 0.2, b: 1.0, a: 1.0 }, Density(100));

pub fn update_sand(element: &ElementType, grid: &mut Grid, api: &mut ElementApi, x: i32, y: i32) {
    if element.can_move_to(grid, api, x, y + 1) {
        api.accelerate(grid, 0.0, 0.5f32);

        let mut velocity = api.get_velocity_mut(grid);
        velocity.x *= 0.65f32;
    } else {
        let mut velocity = api.get_velocity_mut(grid);
        velocity.x *= 0.9f32;
        api.try_vertical_movement(grid, 0.3f32, 0.5f32);
    }

    api.move_element(grid);
}

pub fn update_water(element: &ElementType, grid: &mut Grid, api: &mut ElementApi, x: i32, y: i32) {
    let down: bool = element.can_move_to(grid, api, x, y + 1);

    if down {
        api.accelerate(grid, 0.0, 0.5f32);

        let mut velocity = api.get_velocity_mut(grid);
        velocity.x *= 0.65f32;
        if velocity.y < 1f32 {
            velocity.y = 1f32;
        }
    } else {
        {
            let mut velocity = api.get_velocity_mut(grid);
            velocity.x *= 0.9f32;
        }

        let moved = api.try_vertical_movement(grid, 0.3f32, 0.5f32);
        if !moved {
            api.try_horizontal_movement(grid, 0.5f32);
        }
    }

    api.move_element(grid);
}