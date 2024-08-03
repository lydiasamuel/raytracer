use std::fmt::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use anyhow::anyhow;
use crate::geometry::group::Group;
use crate::tuples::tuple::Tuple;

const VERTEX_COMMAND: &str = "v";
const FACE_COMMAND: &str = "f";

pub struct ObjFileParser {
}

impl ObjFileParser {
    pub fn parse_obj_file(file_path: String) -> Result<Arc<Group>, anyhow::Error> {
        let mut vertices: Vec<Tuple> = Vec::new();

        // Open file
        if let Ok(lines) = Self::read_lines(file_path.clone()) {
            for line in lines.flatten() {
                if !line.trim().is_empty() {
                    let lowercase_line = line.to_lowercase();

                    let split: Vec<&str> = lowercase_line.split(' ').collect();

                    match split[0] {
                        VERTEX_COMMAND => {
                            Self::handle_vertex_command(split, &mut vertices)?;
                        },
                        FACE_COMMAND => {

                        }
                        _ => {
                            // return Err(format!("Error: Unrecognised obj file command on line: {}", i))
                        }
                    }
                }
            }

            Ok(Arc::new(Group::default()))
        } else {
            Err(anyhow!("Error: Could not read lines from: {}", file_path))?
        }
    }

    fn handle_vertex_command(split: Vec<&str>, vertices: &mut Vec<Tuple>) -> Result<(), anyhow::Error>{
        let x = f64::from_str(split[1])?;
        let y = f64::from_str(split[2])?;
        let z = f64::from_str(split[3])?;

        vertices.push(Tuple::point(x, y, z));

        Ok(())
    }

    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(file_path)?;
        Ok(io::BufReader::new(file).lines())
    }
}