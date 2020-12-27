/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

#[macro_use]
pub mod log;
pub mod window;
pub mod input;

#[cfg(test)]
mod tests {
    
    use crate::{input::Input, window::{Window, Mouse, MouseButtonLeft}};

    #[test]
    fn multiple_windows() {
        assert!(crate::window::Window::new(600, 400, "win1").is_some());
        assert!(crate::window::Window::new(600, 400, "win2").is_some());
    }

    #[test]
    fn input() {
        let mut win = Window::new(600, 400, "input").unwrap();
        win.make_current();

        let mut input = Input::new(&mut win);

        loop {
            win.poll_events();
            input.update();

            if input.mouse_up(MouseButtonLeft) {
                info_log!("{} : {}", input.mouse_x(), input.mouse_y());
            }

            if input.mouse_scroll_y() != 0.0 {
                info_log!("{}", input.mouse_scroll_y());
            }

            if input.chars().len() > 0 {
                info_log!("{}", input.chars().iter().collect::<String>());
            }
        }
    }

    #[test]
    fn info_test() {
        info_log!("hello there");
    }

    #[test]
    fn warn_test() {
        warn_log!("hello there");
    }

    #[test]
    fn error_test() {
        error_log!("hello there");
    }

    #[test]
    fn fatal_test() {
        fatal_log!("hello there");
    }
}