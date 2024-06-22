use std::{error::Error, fmt::Write, fs};

use array2d::Array2D;

use crate::tuples::color::Color;

pub struct Canvas {
    grid: Array2D<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        return Canvas::filled_with(Color::new(0.0, 0.0, 0.0), width, height);
    }

    pub fn filled_with(color: Color, width: usize, height: usize) -> Canvas {
        return Canvas {
            grid: Array2D::filled_with(color, height, width),
        };
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), array2d::Error> {
        return self.grid.set(y, x, color);
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
        return self.grid.get(y, x);
    }

    fn clamp_color(color: Color) -> (u8, u8, u8) {
        return (
            Canvas::clamp(color.red),
            Canvas::clamp(color.green),
            Canvas::clamp(color.blue),
        );
    }

    fn clamp(value: f64) -> u8 {
        let x = 255.0 * value;

        if x > 255.0 {
            return 255;
        } else if x < 0.0 {
            return 0;
        } else {
            return x.ceil() as u8;
        }
    }

    pub fn to_ppm(&self) -> Result<String, std::fmt::Error> {
        let mut output = String::new();

        // Write header information at the start of the file
        writeln!(output, "P3")?;
        writeln!(
            output,
            "{}",
            format!("{} {}", self.grid.num_columns(), self.grid.num_rows())
        )?;
        writeln!(output, "255")?;

        for y in 0..self.grid.num_rows() {
            let mut current_line_len: usize = 0;

            for x in 0..self.grid.num_columns() {
                let color = *self.pixel_at(x, y).unwrap();

                let (red, green, blue) = Canvas::clamp_color(color);

                Canvas::write_color_value(red, &mut current_line_len, &mut output)?;
                Canvas::write_color_value(green, &mut current_line_len, &mut output)?;
                Canvas::write_color_value(blue, &mut current_line_len, &mut output)?;
            }

            // Don't write a new line if the previous method already did
            if current_line_len > 0 {
                write!(output, "\n")?;
            }
        }

        return Ok(output);
    }

    fn write_color_value(
        value: u8,
        current_line_len: &mut usize,
        output: &mut String,
    ) -> Result<(), std::fmt::Error> {
        let s = format!("{}", value);

        // If current line length is going to hit 70 then just start a new one
        if *current_line_len + (s.len() + 1) >= 70 {
            write!(output, "\n")?;
            *current_line_len = 0;
        }

        // Don't put a spacer if we've just started a new line
        if *current_line_len > 0 {
            write!(output, " ")?;
            *current_line_len += 1;
        }

        // Write the actual string value
        write!(output, "{}", s)?;
        *current_line_len += s.len();

        // If we only have one spot left at the end of the line use it to start a new one
        if *current_line_len + 1 == 70 {
            write!(output, "\n")?;
            *current_line_len = 0;
        }

        return Ok(());
    }

    pub fn write_to_file(&self, file_path: String) -> Result<(), Box<dyn Error>> {
        let output = self.to_ppm()?;

        fs::write(file_path, output)?;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_a_typical_size_when_creating_a_blank_canvas_should_expect_every_color_to_be_zeroed() {
        let width = 10;
        let height = 20;
        let canvas = Canvas::new(width, height);

        let expected = Color::new(0.0, 0.0, 0.0);

        for x in 0..width {
            for y in 0..height {
                let result = *canvas.pixel_at(x, y).unwrap();

                assert_eq!(expected, result);
            }
        }
    }

    #[test]
    fn given_a_blank_canvas_when_writing_to_a_pixel_should_set_that_pixels_color_correctly() {
        let width = 10;
        let height = 20;
        let mut canvas = Canvas::new(width, height);

        let red = Color::new(1.0, 0.0, 0.0);

        let _ = canvas.write_pixel(2, 3, red);

        assert_eq!(red, *canvas.pixel_at(2, 3).unwrap())
    }

    #[test]
    fn given_a_canvas_with_a_few_color_pixels_when_converting_to_ppm_should_output_file_correctly()
    {
        let width = 5;
        let height = 3;
        let mut canvas = Canvas::new(width, height);

        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        let _ = canvas.write_pixel(0, 0, c1);
        let _ = canvas.write_pixel(2, 1, c2);
        let _ = canvas.write_pixel(4, 2, c3);

        let expected = "P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
";

        let result = canvas.to_ppm().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn given_a_canvas_pixels_of_all_one_color_when_converting_to_ppm_should_output_file_correctly()
    {
        let width = 10;
        let height = 2;
        let canvas = Canvas::filled_with(Color::new(1.0, 0.8, 0.6), width, height);

        let expected = "P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
";

        let result = canvas.to_ppm().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn given_a_blank_canvas_when_converting_to_ppm_should_correctly_terminate_file() {
        let width = 5;
        let height = 3;
        let canvas = Canvas::new(width, height);

        let result = canvas.to_ppm().unwrap();

        assert_eq!(true, result.ends_with("\n"));
    }
}
