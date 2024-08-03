use crate::geometry::group::Group;
use crate::geometry::shape::Shape;
use crate::geometry::triangle::Triangle;
use crate::materials::material::Material;
use crate::matrices::matrix::Matrix;
use crate::tuples::tuple::Tuple;
use anyhow::anyhow;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

const VERTEX_COMMAND: &str = "v";
const FACE_COMMAND: &str = "f";
const GROUP_COMMAND: &str = "g";

pub struct ObjFileParser {
    vertices: Vec<Tuple>,
    groups: HashMap<String, Arc<Group>>,
    default_group: Uuid,
    current_group: String,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    casts_shadow: bool,
}

impl ObjFileParser {
    pub fn parse_obj_file(
        file_path: String,
        transform: Arc<Matrix>,
        material: Arc<dyn Material>,
        casts_shadow: bool,
    ) -> Result<ObjFileParser, anyhow::Error> {
        let default_group = Uuid::new_v4();

        let mut result = ObjFileParser {
            vertices: Vec::new(),
            groups: HashMap::new(),
            default_group,
            current_group: default_group.to_string(),
            transform,
            material,
            casts_shadow,
        };

        // Open file
        if let Ok(lines) = Self::read_lines(file_path.clone()) {
            for line in lines.flatten() {
                if !line.trim().is_empty() {
                    let split: Vec<&str> = line.split(' ').collect();

                    let command = split[0].to_lowercase();

                    if command == VERTEX_COMMAND {
                        result.handle_vertex_command(split)?;
                    } else if command == FACE_COMMAND {
                        result.handle_face_command(split)?;
                    } else if command == GROUP_COMMAND {
                        result.handle_group_command(split);
                    }
                }
            }

            Ok(result)
        } else {
            Err(anyhow!("Error: Could not read lines from: {}", file_path))?
        }
    }

    fn handle_vertex_command(&mut self, split: Vec<&str>) -> Result<(), anyhow::Error> {
        let x = f64::from_str(split[1])?;
        let y = f64::from_str(split[2])?;
        let z = f64::from_str(split[3])?;

        self.vertices.push(Tuple::point(x, y, z));

        Ok(())
    }

    fn handle_face_command(&mut self, split: Vec<&str>) -> Result<(), anyhow::Error> {
        let mut vertices: Vec<usize> = Vec::new();

        for i in 1..split.len() {
            vertices.push(usize::from_str(split[i])?);
        }

        let triangles = self.fan_triangulation(vertices);

        if let Some(group) = self.groups.get(&self.current_group) {
            group.add_children(triangles)
        } else {
            let group = Arc::new(Group::default());
            group.add_children(triangles);
            self.groups.insert(self.current_group.clone(), group);
        }

        Ok(())
    }

    fn handle_group_command(&mut self, split: Vec<&str>) {
        let group_name = split[1];

        self.current_group = group_name.to_string();
    }

    // Assumes we're dealing with convex polygons - i.e. those whose interior angles are all less
    // than or equal to 180 degrees
    fn fan_triangulation(&mut self, vertices: Vec<usize>) -> Vec<Arc<dyn Shape>> {
        let mut triangles: Vec<Arc<dyn Shape>> = Vec::new();

        for index in 1..(vertices.len() - 1) {
            let p1 = self.get_vertex(vertices[0]);
            let p2 = self.get_vertex(vertices[index]);
            let p3 = self.get_vertex(vertices[index + 1]);

            triangles.push(Arc::new(Triangle::new(
                p1,
                p2,
                p3,
                self.transform.clone(),
                self.material.clone(),
                self.casts_shadow,
            )));
        }

        triangles
    }

    fn get_vertex(&self, index: usize) -> Tuple {
        self.vertices[index - 1]
    }

    // The output is wrapped in a Result to allow matching on errors.
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(file_path)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn obj_to_group(self, transform: Arc<Matrix>) -> Arc<Group> {
        let result = Arc::new(Group::new(transform));

        for group in self.groups.values() {
            result.add_child(group.clone());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Shape;
    use crate::materials::phong::Phong;
    use crate::matrices::matrix::Matrix;
    use crate::scene::obj_file_parser::ObjFileParser;
    use crate::tuples::tuple::Tuple;
    use std::sync::Arc;

    #[test]
    fn given_a_bad_obj_file_when_parsing_should_silently_ignore_all_rubbish_lines() {
        // Arrange
        let file_path = "tests/obj_files/gibberish.obj";

        // Act
        let result = ObjFileParser::parse_obj_file(
            file_path.to_string(),
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
        )
        .unwrap();

        // Assert
        assert_eq!(0, result.vertices.len());
    }

    #[test]
    fn given_an_obj_file_with_just_vertices_when_parsing_should_correctly_parse_out_each_one() {
        // Arrange
        let file_path = "tests/obj_files/vertices.obj";

        // Act
        let result = ObjFileParser::parse_obj_file(
            file_path.to_string(),
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
        )
        .unwrap();

        // Assert
        assert_eq!(4, result.vertices.len());
        assert_eq!(Tuple::point(-1.0, 1.0, 0.0), result.vertices[0]);
        assert_eq!(Tuple::point(-1.0, 0.5, 0.0), result.vertices[1]);
        assert_eq!(Tuple::point(1.0, 0.0, 0.0), result.vertices[2]);
        assert_eq!(Tuple::point(1.0, 1.0, 0.0), result.vertices[3]);
    }

    #[test]
    fn given_an_obj_file_with_faces_when_parsing_should_correctly_parse_out_each_one() {
        // Arrange
        let file_path = "tests/obj_files/faces.obj";

        // Act
        let result = ObjFileParser::parse_obj_file(
            file_path.to_string(),
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
        )
        .unwrap();

        // Assert
        assert_eq!(true, result.groups.get(&result.current_group).is_some());

        let group = result.groups.get(&result.current_group).unwrap();

        assert_eq!(2, group.count());

        let t1_points = group.get_child(0).unwrap().points();
        let t2_points = group.get_child(1).unwrap().points();

        assert_eq!(result.vertices[0], t1_points.0);
        assert_eq!(result.vertices[1], t1_points.1);
        assert_eq!(result.vertices[2], t1_points.2);
        assert_eq!(result.vertices[0], t2_points.0);
        assert_eq!(result.vertices[2], t2_points.1);
        assert_eq!(result.vertices[3], t2_points.2);
    }

    #[test]
    fn given_an_obj_file_with_named_groups_when_parsing_should_correctly_assign_each_face_to_the_right_group(
    ) {
        // Arrange
        let file_path = "tests/obj_files/named_groups.obj";

        // Act
        let result = ObjFileParser::parse_obj_file(
            file_path.to_string(),
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
        )
        .unwrap();

        // Assert
        assert_eq!(true, result.groups.get("FirstGroup").is_some());

        let first_group = result.groups.get("FirstGroup").unwrap();

        assert_eq!(1, first_group.count());

        let t1_points = first_group.get_child(0).unwrap().points();

        assert_eq!(result.vertices[0], t1_points.0);
        assert_eq!(result.vertices[1], t1_points.1);
        assert_eq!(result.vertices[2], t1_points.2);

        assert_eq!(true, result.groups.get("SecondGroup").is_some());

        let second_group = result.groups.get("SecondGroup").unwrap();

        assert_eq!(1, second_group.count());

        let t2_points = second_group.get_child(0).unwrap().points();

        assert_eq!(result.vertices[0], t2_points.0);
        assert_eq!(result.vertices[2], t2_points.1);
        assert_eq!(result.vertices[3], t2_points.2);
    }

    #[test]
    fn given_an_obj_file_with_named_groups_when_converting_it_to_a_group_should_correctly_include_all_groups(
    ) {
        // Arrange
        let file_path = "tests/obj_files/named_groups.obj";

        // Act
        let result = ObjFileParser::parse_obj_file(
            file_path.to_string(),
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
        )
        .unwrap();

        // Assert
        assert_eq!(true, result.groups.get("FirstGroup").is_some());

        let first_group: Arc<dyn Shape> = result.groups.get("FirstGroup").unwrap().clone();

        let second_group: Arc<dyn Shape> = result.groups.get("SecondGroup").unwrap().clone();

        let group = result.obj_to_group(Arc::new(Matrix::identity(4)));

        assert!(Arc::ptr_eq(&group.get_child(0).unwrap(), &first_group));
        assert!(Arc::ptr_eq(&group.get_child(1).unwrap(), &second_group));
    }
}
