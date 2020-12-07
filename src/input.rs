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

use crate::window::*;
use crate::window;

pub use window::Key;
pub use window::Mouse;

pub type KeyEventDispatcher                 = bus::BusReader::<(Key, Action)>;
pub type MouseEventDispatcher               = bus::BusReader::<(Mouse, Action)>;
pub type MouseMoveEventDispatcher           = bus::BusReader::<(f32, f32)>;
pub type FrameBufferSizeEventDispatcher     = bus::BusReader::<(u32, u32)>;

pub trait KeyListener {
    fn on_key_pressed(&mut self, key: Key);
    fn on_key_released(&mut self, key: Key);
    fn update(&mut self);
}

pub trait MouseListener {
    fn on_mouse_moved(&mut self, x: f32, y: f32);
    fn on_mouse_pressed(&mut self, button: Mouse);
    fn on_mouse_released(&mut self, button: Mouse);
    fn update(&mut self);
}

pub struct InputKeyHandler {
    keys:         [bool; 512],
    keys_down:    [bool; 512],
    keys_up:      [bool; 512],
}

pub struct InputMouseHandler {
    buttons:      [bool; 16],
    buttons_down: [bool; 16],
    buttons_up:   [bool; 16],
    mouse_x: f32,
    mouse_y: f32,
}

pub struct Input {
    key_handler         : InputKeyHandler,
    mouse_handler       : InputMouseHandler,
    
    key_reciver         : bus::BusReader::<(Key, Action)>,
    mouse_reciver       : bus::BusReader::<(Mouse, Action)>,
    mouse_move_reciver  : bus::BusReader::<(f32, f32)>,
} 

impl KeyListener for InputKeyHandler {
    fn on_key_pressed(&mut self, key: Key)          { let index = key as usize; self.keys[index] = true; self.keys_down[index] = true;  }
    fn on_key_released(&mut self, key: Key)         { let index = key as usize; self.keys[index] = false; self.keys_up[index] = true;   }

    fn update(&mut self) {
        for i in 0..512 {
			self.keys_down[i] = false;
		}
		for i in 0..512 {
			self.keys_up[i] = false;
        }
    }
}

impl MouseListener for InputMouseHandler {
    fn on_mouse_moved(&mut self, x: f32, y: f32)        { self.mouse_x = x; self.mouse_y = y; }
    fn on_mouse_pressed(&mut self, button: Mouse)       { let index = button as usize; self.buttons[index] = true; self.buttons_down[index] = true; }
    fn on_mouse_released(&mut self, button: Mouse)      { let index = button as usize; self.buttons[index] = false; self.buttons_up[index] = true;  }

    fn update(&mut self) {
		for i in 0..16 {
			self.buttons_down[i] = false;
		}
		for i in 0..16 {
			self.buttons_up[i] = false;
		}
    }
}

impl InputKeyHandler {
    fn new() -> InputKeyHandler {
        InputKeyHandler {
            keys:           [false; 512],
            keys_down:      [false; 512],
            keys_up:        [false; 512],
        }
    }
    
    fn key(&self, key: Key)         -> bool { self.keys[key as usize]       }
    fn key_down(&self, key: Key)    -> bool { self.keys_down[key as usize]  }
    fn key_up(&self, key: Key)      -> bool { self.keys_up[key as usize]    }
}

impl InputMouseHandler {
    fn new() -> InputMouseHandler {
        InputMouseHandler {
            buttons:        [false; 16],
            buttons_down:   [false; 16],
            buttons_up:     [false; 16],
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    fn mouse(&self, button: Mouse)      -> bool { self.buttons[button as usize]         }
    fn mouse_down(&self, button: Mouse) -> bool { self.buttons_down[button as usize]    }
    fn mouse_up(&self, button: Mouse)   -> bool { self.buttons_up[button as usize]      }

    fn mouse_x(&self)   -> f32 { self.mouse_x }
    fn mouse_y(&self)   -> f32 { self.mouse_y }
}

impl Input {
    pub fn new(win: &mut Window) -> Input {
        
        Input {
            key_handler: InputKeyHandler::new(),
            mouse_handler: InputMouseHandler::new(),

            key_reciver: win.create_key_listener(),
            mouse_reciver: win.create_mouse_listener(),
            mouse_move_reciver: win.create_mouse_move_listener(),
        }
    }

    pub fn update(&mut self) {
        self.key_handler.update();   
        self.mouse_handler.update();

        let mut loop_done = false;
        while !loop_done {
            match self.key_reciver.try_recv() {
                Ok((key, action)) => {
                    match action {
                        Action::Press => {
                            self.key_handler.on_key_pressed(key);
                        },
                        Action::Release => {
                            self.key_handler.on_key_released(key);
                        },
                        _ => {},
                    }
                },
                Err(_) => {
                    loop_done = true;
                }
            }
        }

        let mut loop_done = false;
        while !loop_done {
            match self.mouse_reciver.try_recv() {
                Ok((mouse, action)) => {
                    match action {
                        Action::Press => {
                            self.mouse_handler.on_mouse_pressed(mouse);
                        },
                        Action::Release => {
                            self.mouse_handler.on_mouse_released(mouse);
                        },
                        _ => {},
                    }
                },
                Err(_) => {
                    loop_done = true;
                }
            }
        }

        let mut loop_done = false;
        while !loop_done {
            match self.mouse_move_reciver.try_recv() {
                Ok((x, y)) => {
                    self.mouse_handler.on_mouse_moved(x, y);
                },
                Err(_) => {
                    loop_done = true;
                }
            }
        }
    }
  
    pub fn key(&self, key: Key)         -> bool { self.key_handler.key(key)        }
    pub fn key_down(&self, key: Key)    -> bool { self.key_handler.key_down(key)   }
    pub fn key_up(&self, key: Key)      -> bool { self.key_handler.key_up(key)     }

    pub fn mouse(&self, button: Mouse)      -> bool { self.mouse_handler.mouse(button)     }
    pub fn mouse_down(&self, button: Mouse) -> bool { self.mouse_handler.mouse_down(button)}
    pub fn mouse_up(&self, button: Mouse)   -> bool { self.mouse_handler.mouse_up(button)  }

    pub fn mouse_x(&self)   -> f32 { self.mouse_handler.mouse_x() }
    pub fn mouse_y(&self)   -> f32 { self.mouse_handler.mouse_y() }
}
