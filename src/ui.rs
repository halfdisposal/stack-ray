#![allow(clippy::too_many_arguments)]
use raylib::prelude::*;

pub struct Button {
    pub rect: Rectangle,
    pub label: String,
    pub default: Color,
    pub hover: Color,
    pub clicked: Color,
    pub font_size: i32
}

pub struct ButtonState {
    pub default: Color,
    pub hover: Color,
    pub clicked: Color,
}

pub struct ToggleButton {
    pub rect: Rectangle,
    pub label: String,
    pub state: ButtonState,
    pub on: bool,
    pub font_size: i32,
    pub toggle_thickness: f32
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str, state: ButtonState, font_size: i32) -> Self {
        println!("Button {label} Created!");
        Button { rect: ffi::Rectangle::new(x - width / 2.0, y - height / 2.0, width, height), label: label.to_string(), default: state.default, hover: state.hover, clicked: state.clicked, font_size}
    }

    pub fn contains(&self, point: Vector2) -> bool {
        point.x >= self.rect.x && point.x <= self.rect.x + self.rect.width &&
        point.y >= self.rect.y && point.y <= self.rect.y + self.rect.height
    }

    pub fn is_clicked(&self, rl: &RaylibHandle) -> bool {
        self.contains(rl.get_mouse_position()) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
    }
    
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let hovered = self.contains(d.get_mouse_position());
        let bg = if hovered { self.hover } else { self.default };

        d.draw_rectangle_rec(self.rect, bg);
        d.draw_rectangle_lines_ex(self.rect, 2.0, Color::BLACK);

        let font_size = self.font_size;
        let text_width = d.measure_text(&self.label, font_size);
        let tx = self.rect.x as i32 + (self.rect.width as i32 - text_width) / 2;
        let ty = self.rect.y as i32 + (self.rect.height as i32 - font_size) / 2;
        d.draw_text(&self.label, tx, ty, font_size, Color::BLACK);
    }
}


impl ToggleButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str, state: ButtonState, font_size: i32, toggle_thickness: f32, on: bool) -> Self {
        ToggleButton { rect: ffi::Rectangle::new(x - width / 2.0, y - height / 2.0, width, height), 
            label: label.to_string(), state, on, font_size, toggle_thickness}
    }

    pub fn contains(&self, point: Vector2) -> bool {
        point.x >= self.rect.x && point.x <= self.rect.x + self.rect.width &&
        point.y >= self.rect.y && point.y <= self.rect.y + self.rect.height
    }

    pub fn is_clicked(&mut self, rl: &RaylibHandle) -> bool {
        let clicked = self.contains(rl.get_mouse_position()) && rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        if clicked {
            self.on = !self.on;
            println!("INFO: Button {} state {}", self.label, self.on);
        }
        clicked
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let pad = 4.0;

        let h = self.toggle_thickness;
        let w = 2.0 * self.toggle_thickness + 2.0 * pad;
        let x = self.rect.x + self.rect.width - w - pad;
        let y = self.rect.y + (self.rect.height - self.toggle_thickness)/2.0;

        d.draw_rectangle_rec(self.rect, Color::new(255, 255, 255, 200));
        d.draw_rectangle_lines_ex(self.rect, 2.0, Color::BLACK);

        let bg_rect = ffi::Rectangle::new(x, y, w, h);

        d.draw_rectangle_rec(bg_rect, self.state.default);
        d.draw_rectangle_lines_ex(bg_rect, 2.0, self.state.hover);

        let s_h = self.toggle_thickness - 2.0 * pad;
        let s_w = s_h;
        let s_x = if self.on { bg_rect.x + pad } else { bg_rect.x + bg_rect.width - pad - s_w };
        let s_y = bg_rect.y + pad;
        let bg = if self.on { self.state.clicked } else { self.state.hover };
        let bg_rect2 = ffi::Rectangle::new(s_x, s_y, s_w, s_h);
        d.draw_rectangle_rec(bg_rect2, bg);


        let text_width = d.measure_text(&self.label, self.font_size);
        let tx = self.rect.x as i32 + pad as i32;
        let ty = self.rect.y as i32 + (self.rect.height as i32 - self.font_size) / 2;
        d.draw_text(&self.label, tx, ty, self.font_size, Color::BLACK);
    }
}
