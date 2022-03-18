use std::cell::RefCell;
use std::fs::File;
use std::io::{Write, Error};

use canvas::Canvas;

use crate::tuples::color::Color;

pub struct MyCanvas {
    pub width: usize,
    pub height: usize,
    canvas: RefCell<Canvas<[u8; 3]>>
}

impl MyCanvas {
    pub fn new(width: usize, height: usize) -> MyCanvas {
        return MyCanvas {
            width,
            height,
            canvas: RefCell::new(Canvas::with_width_and_height(width, height))
        }
    }

    pub fn draw(&self, color: Color, x: usize, y: usize) {
        if self.is_within_bounds(x, y) {
            let r = MyCanvas::clamp(color.red);
            let g = MyCanvas::clamp(color.green);
            let b = MyCanvas::clamp(color.blue);

            self.canvas.borrow_mut()[(x, y)] = [r, g, b];
        }
    }

    fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        return x <= self.width && y < self.height;
    }

    fn clamp(x: f32) -> u8 {
        let c = 255.0 * x;

        if c > 255.0 {
            return 255;
        }
        else if c < 0.0 {
            return 0;
        }
        else {
            return c as u8;
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Color {
        let c = self.canvas.borrow()[(x, y)];
        
        let r = (c[0] as f32) / 255.0;
        let g = (c[1] as f32) / 255.0;
        let b = (c[2] as f32) / 255.0;

        return Color::new(r, g, b);
    }

    pub fn to_ppm(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;

        // Write header information at the start of the file
        let header = writeln!(output, "P3");
        match header {
            Ok(()) => {},
            Err(error) => panic!("Error writing header info to PPM output: {:?}", error)
        }

        let header = writeln!(output, "{}", format!("{} {}", self.width, self.height));
        match header {
            Ok(()) => {},
            Err(error) => panic!("Error writing header info to PPM output: {:?}", error)
        }

        let header = writeln!(output, "255");
        match header {
            Ok(()) => {},
            Err(error) => panic!("Error writing header info to PPM output: {:?}", error)
        }

        for row in 0..self.height {
            let mut count = 0;
            for column in 0..self.width {
                let c = self.canvas.borrow()[(column, row)];
                let pixel = format!("{} {} {}", c[0], c[1], c[2]);

                count += pixel.len() + 1; // Extra space at the end

                if count > 70 { // Start a newline if it goes past 70
                    let body = write!(output, "\n");
                    match body {
                        Ok(()) => {},
                        Err(error) => panic!("Error writing content to PPM output: {:?}", error)
                    }
                    count = (count + 1) % 70; 
                }

                let body = write!(output, "{} ", pixel);
                match body {
                    Ok(()) => {},
                    Err(error) => panic!("Error writing content to PPM output: {:?}", error)
                }
            }
            
            let body = write!(output, "\n");
            match body {
                Ok(()) => {},
                Err(error) => panic!("Error writing content to PPM output: {:?}", error)
            }
        }

        return Ok(());
    }
}