use crate::{global, element::{self, elements, element_api::ElementApi}};

pub struct Grid {
    pub data: Vec<element::Element>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            data: vec![element::elements::EMPTY; (global::WIDTH * global::HEIGHT) as usize],
        }
    }

    pub fn width(&self) -> i32 {
        global::WIDTH
    }
    pub fn height(&self) -> i32 {
        global::HEIGHT
    }
    
    pub fn get_index(&self, x: i32, y: i32) -> usize {
        (self.height() * y + x) as usize
    }

    pub fn within_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width() as i32 && y < self.height() as i32
    }

    pub fn get_element(&self, x: i32, y: i32) -> element::Element {
        self.data[self.get_index(x, y)]
    }

    pub fn get_element_mut(&mut self, x: i32, y: i32) -> &mut element::Element {
        let index = self.get_index(x, y);
        &mut self.data[index]
    }

    pub fn set_element(&mut self, x: i32, y: i32, element: element::Element) {
        let index = self.get_index(x, y);
        self.data[index] = element;
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.get_element(x, y) == elements::EMPTY
    }

    pub fn is_empty_and_within_bounds(&self, x: i32, y: i32) -> bool {
        self.within_bounds(x, y) && self.is_empty(x, y)
    }

    pub fn swap_cells(&mut self, src_x: i32, src_y: i32, dest_x: i32, dest_y: i32) {
        let height = self.height() as i32;
        self.data.swap(
            (height * src_y + src_x) as usize,
            (height * dest_y + dest_x) as usize,
        );
    }

    pub fn update_element(&mut self, x: i32, y: i32) {
        let element = self.get_element(x, y);
        let mut api = ElementApi { element, x, y };

        element.element_type.update(self, &mut api, x, y);
    }
}
