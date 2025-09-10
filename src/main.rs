use macroquad::prelude::*;

struct Clicker {
    clicks: i32,
}

impl Clicker {
    fn new() -> Self {
        Self { clicks: 0 }
    }

    fn update(&mut self) {
        let button_rect = self.get_button_rect();

        // 🖱️👆 Обработка мыши И тача — в macroquad они работают одинаково!
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse = mouse_position().into();
            if button_rect.contains(mouse) {
                self.clicks += 1;
            }
        }
    }

    fn get_button_rect(&self) -> Rect {
        let w = screen_width() * 0.3;   // 30% ширины экрана
        let h = screen_height() * 0.1;  // 10% высоты
        let x = screen_width() / 2.0 - w / 2.0;
        let y = screen_height() / 2.0 - h / 2.0;
        Rect::new(x, y, w, h)
    }

    fn draw(&self) {
        clear_background(DARKGRAY);

        let button_rect = self.get_button_rect();

        // 🎨 Кнопка
        draw_rectangle(button_rect.x, button_rect.y, button_rect.w, button_rect.h, BLUE);

        // 🔤 Размер шрифта — пропорционален ВЫСОТЕ кнопки
        let font_size = (button_rect.h * 0.6).max(16.0); // 60% от высоты кнопки, минимум 16px

        // 📐 Центрируем текст в кнопке
        let text = "CLICK ME";
        let text_width = measure_text(text, None, font_size as u16, 1.0).width;
        let text_x = button_rect.x + (button_rect.w - text_width) / 2.0;
        let text_y = button_rect.y + button_rect.h / 2.0 + font_size / 3.0;

        draw_text(text, text_x, text_y, font_size, WHITE);

        // 📊 Счётчик — адаптивный, но не слишком мелкий
        let score_font_size = (screen_height() * 0.08).max(24.0);
        draw_text(
            &format!("Кликов: {}", self.clicks),
            20.0,
            score_font_size + 10.0,
            score_font_size,
            GOLD
        );
    }
}

#[macroquad::main("Perfect Clicker")]
async fn main() {
    let mut game = Clicker::new();
    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}
