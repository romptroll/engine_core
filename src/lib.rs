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

#[macro_export]
macro_rules! gl_call  {
    ($x:expr) => {
        crate::renderer::gl_clear_error();
        $x;
        if crate::renderer::gl_log_call(file!(), line!()) == false {
            
        }
    };
}

#[macro_export]
macro_rules! info_log {
    () => (crate::info(&format!("[File] {} [Line] {}", file!(), line!())));
    ($($arg:tt)*) => ({
        crate::info(&format!("[File] {} [Line] {} [Message] {}", file!(), line!(), format!($($arg)*)));
    })
}

#[macro_export]
macro_rules! warn_log {
    () => (crate::warn(&format!("[File] {} [Line] {}", file!(), line!())));
    ($($arg:tt)*) => ({
        crate::warn(&format!("[File] {} [Line] {} [Message] {}", file!(), line!(), format!($($arg)*)));
    })
}

#[macro_export]
macro_rules! error_log {
    () => (crate::error(&format!("[File] {} [Line] {}", file!(), line!())));
    ($($arg:tt)*) => ({
        crate::error(&format!("[File] {} [Line] {} [Message] {}", file!(), line!(), format!($($arg)*)));
    })
}

#[macro_export]
macro_rules! fatal_log {
    () => (crate::fatal(&format!("[File] {} [Line] {}", file!(), line!())));
    ($($arg:tt)*) => ({
        crate::fatal(&format!("[File] {} [Line] {} [Message] {}", file!(), line!(), format!($($arg)*)));
    })
}

use colored::*;

pub fn info(message: &str)      { println!("{}", format!("[INFO] {}", message).green().bold())  }
pub fn warn(message: &str)      { println!("{}", format!("[WARN] {}", message).yellow().bold()) }
pub fn error(message: &str)     { println!("{}", format!("[ERROR] {}", message).red().bold())   }
pub fn fatal(message: &str)     { println!("{}", format!("[FATAL] {}", message).red().bold())   }

#[cfg(test)]
mod tests {
    
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