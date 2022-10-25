use rand::random;

use crate::{world::grid::Grid, util::math::{self}};

use super::Element;

pub struct ElementApi {
    pub element: Element,
    pub x: i32,
    pub y: i32,
}

impl ElementApi{
    pub fn get_element_mut<'a>(&mut self, grid: &'a mut Grid) -> &'a mut Element {
        grid.get_element_mut(self.x, self.y)
    }

    pub fn get_velocity_mut<'a>(&mut self, grid: &'a mut Grid) -> &'a mut glam::Vec2 {
        let element = self.get_element_mut(grid);
        &mut element.velocity
    }

    pub fn accelerate(&mut self, grid: &mut Grid, x: f32, y: f32) {
        let mut velocity = self.get_velocity_mut(grid);   
        velocity.x = (velocity.x + x).clamp(-5f32, 5f32);
        velocity.y = (velocity.y + y).clamp(-5f32, 5f32);
    }

    pub fn try_horizontal_movement(&mut self, grid: &mut Grid, speed_x: f32) -> bool {
        let element_type = self.element.element_type;
        let dir = if self.element.frame % 2 == 0 { 1 } else { -1 };
        let left = element_type.can_move_to(grid, self, self.x - dir, self.y);
        let right = element_type.can_move_to(grid, self, self.x + dir, self.y);

        if left {
            self.accelerate(grid, speed_x * -dir as f32, 0f32);
        } else if right {
            self.accelerate(grid, speed_x * dir as f32, 0f32);
        }

        left || right
    }


    pub fn try_vertical_movement(&mut self, grid: &mut Grid, speed_x: f32, speed_y: f32) -> bool {
        let element_type = self.element.element_type;
        let dir = if self.element.frame % 2 == 0 { 1 } else { -1 };
        let left = element_type.can_move_to(grid, self, self.x - dir, self.y + 1);
        let right = element_type.can_move_to(grid, self, self.x + dir, self.y + 1);

        if left {
            self.accelerate(grid, speed_x * -dir as f32, speed_y);
        } else if right {
            self.accelerate(grid, speed_x * dir as f32, speed_y);
        }

        left || right
    }

    pub fn move_element(&mut self, grid: &mut Grid) {
        let element = self.element;
        let vel_x = element.velocity.x as i32;
        let vel_y = element.velocity.y as i32;
        if vel_x == 0 && vel_y == 0 {
            return;
        }
        
        let mut move_x = self.x;
        let mut move_y = self.y;
        let mut moved = false;
        let mut continue_iteration = true;

        math::iterate_grid_line(self.x, self.y, move_x + vel_x, move_y + vel_y, |dest_x, dest_y| {
            if !continue_iteration {
                return;
            }

            if !element.element_type.can_move_to(grid, self, dest_x, dest_y) {
                continue_iteration = false;
                return;
            }

            move_x = dest_x;
            move_y = dest_y;
            moved = true;
        });

        {
            let mut velocity = self.get_velocity_mut(grid);
            if !moved && (velocity.x != 0f32 || velocity.y != 0f32) {
                if velocity.y.ceil() != 0f32 {
                    let delta = velocity.y.abs() * 0.9f32;
                    velocity.x += if random() { -delta } else { delta }; 

                    velocity.y *= 0.1;
                }

                velocity.x *= 0.7f32;
            }
        }

        grid.swap_cells(self.x, self.y, move_x, move_y);
    }
}