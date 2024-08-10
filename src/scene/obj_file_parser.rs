use crate::geometry::group::Group;
use crate::geometry::shape::Shape;
use crate::geometry::smooth_triangle::SmoothTriangle;
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
const VERTEX_NORMAL_COMMAND: &str = "vn";

pub struct ObjFileParser {
    vertices: Vec<Tuple>,
    vertex_normals: Vec<Tuple>,
    groups: HashMap<String, Arc<Group>>,
    default_group: Uuid,
    current_group: String,
    transform: Arc<Matrix>,
    material: Arc<dyn Material>,
    casts_shadow: bool,
}

struct Indices {
    pub index: usize,
    pub normal_index: Option<usize>,
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
            vertex_normals: Vec::new(),
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
                    let mut parameters: Vec<&str> = line.split(' ').collect();

                    // Remove any empty parameters caused by multiple spaces
                    parameters.retain(|&x| x != "");

                    let command = parameters[0].to_lowercase();

                    if command == VERTEX_COMMAND {
                        result.handle_vertex_command(parameters)?;
                    } else if command == FACE_COMMAND {
                        result.handle_face_command(parameters)?;
                    } else if command == GROUP_COMMAND {
                        result.handle_group_command(parameters);
                    } else if command == VERTEX_NORMAL_COMMAND {
                        result.handle_vertex_normal_command(parameters)?;
                    }
                }
            }

            Ok(result)
        } else {
            Err(anyhow!("Error: Could not read lines from: {}", file_path))?
        }
    }

    fn handle_vertex_command(&mut self, parameters: Vec<&str>) -> Result<(), anyhow::Error> {
        let x = f64::from_str(parameters[1])?;
        let y = f64::from_str(parameters[2])?;
        let z = f64::from_str(parameters[3])?;

        self.vertices.push(Tuple::point(x, y, z));

        Ok(())
    }

    fn handle_face_command(&mut self, parameters: Vec<&str>) -> Result<(), anyhow::Error> {
        let mut vertex_indices: Vec<Indices> = Vec::new();

        for i in 1..parameters.len() {
            let option = parameters[i];

            let split: Vec<&str> = option.split('/').collect();

            if split.len() == 1 {
                let index = usize::from_str(option)?;

                vertex_indices.push(Indices {
                    index,
                    normal_index: None,
                })
            } else {
                let index = usize::from_str(split[0])?;
                let normal_index = usize::from_str(split[2])?;

                vertex_indices.push(Indices {
                    index,
                    normal_index: Some(normal_index),
                })
            }
        }

        let triangles = self.fan_triangulation(vertex_indices);

        // If the group is already present, just add the new triangles as children
        if let Some(group) = self.groups.get(&self.current_group) {
            group.add_children(triangles)
        } else {
            let group = Arc::new(Group::default());
            group.add_children(triangles);
            self.groups.insert(self.current_group.clone(), group);
        }

        Ok(())
    }

    fn handle_group_command(&mut self, parameters: Vec<&str>) {
        let group_name = parameters[1];

        self.current_group = group_name.to_string();
    }

    fn handle_vertex_normal_command(&mut self, parameters: Vec<&str>) -> Result<(), anyhow::Error> {
        let x = f64::from_str(parameters[1])?;
        let y = f64::from_str(parameters[2])?;
        let z = f64::from_str(parameters[3])?;

        self.vertex_normals.push(Tuple::vector(x, y, z));

        Ok(())
    }

    // Assumes we're dealing with convex polygons - i.e. those whose interior angles are all less
    // than or equal to 180 degrees
    fn fan_triangulation(&mut self, vertex_indices: Vec<Indices>) -> Vec<Arc<dyn Shape>> {
        let mut triangles: Vec<Arc<dyn Shape>> = Vec::new();

        for index in 1..(vertex_indices.len() - 1) {
            let p1 = self.get_vertex(vertex_indices[0].index);
            let p2 = self.get_vertex(vertex_indices[index].index);
            let p3 = self.get_vertex(vertex_indices[index + 1].index);

            // If there's no normal index we're dealing with a non-smooth triangle
            if vertex_indices[0].normal_index.is_none() {
                triangles.push(Arc::new(Triangle::new(
                    p1,
                    p2,
                    p3,
                    self.transform.clone(),
                    self.material.clone(),
                    self.casts_shadow,
                )));
            } else {
                let n1 = self.get_vertex_normal(
                    vertex_indices[0]
                        .normal_index
                        .expect("Error: Expected vertex normal for face to be present."),
                );
                let n2 = self.get_vertex_normal(
                    vertex_indices[index]
                        .normal_index
                        .expect("Error: Expected vertex normal for face to be present."),
                );
                let n3 = self.get_vertex_normal(
                    vertex_indices[index + 1]
                        .normal_index
                        .expect("Error: Expected vertex normal for face to be present."),
                );

                triangles.push(Arc::new(SmoothTriangle::new(
                    p1,
                    p2,
                    p3,
                    n1,
                    n2,
                    n3,
                    self.transform.clone(),
                    self.material.clone(),
                    self.casts_shadow,
                )));
            }
        }

        triangles
    }

    fn get_vertex(&self, index: usize) -> Tuple {
        self.vertices[index - 1]
    }

    fn get_vertex_normal(&self, index: usize) -> Tuple {
        self.vertex_normals[index - 1]
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

        assert_eq!(2, group.num_of_children());

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

        assert_eq!(1, first_group.num_of_children());

        let t1_points = first_group.get_child(0).unwrap().points();

        assert_eq!(result.vertices[0], t1_points.0);
        assert_eq!(result.vertices[1], t1_points.1);
        assert_eq!(result.vertices[2], t1_points.2);

        assert_eq!(true, result.groups.get("SecondGroup").is_some());

        let second_group = result.groups.get("SecondGroup").unwrap();

        assert_eq!(1, second_group.num_of_children());

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

        assert_eq!(2, group.num_of_children());

        let contains_first_group = Arc::ptr_eq(&group.get_child(0).unwrap(), &first_group)
            || Arc::ptr_eq(&group.get_child(1).unwrap(), &first_group);

        let contains_second_group = Arc::ptr_eq(&group.get_child(0).unwrap(), &second_group)
            || Arc::ptr_eq(&group.get_child(1).unwrap(), &second_group);

        assert!(contains_first_group);
        assert!(contains_second_group);
    }

    #[test]
    fn given_an_obj_file_with_just_vertex_normals_when_parsing_should_correctly_parse_out_each_one()
    {
        // Arrange
        let file_path = "tests/obj_files/vertex_normals.obj";

        // Act
        let result = ObjFileParser::parse_obj_file(
            file_path.to_string(),
            Arc::new(Matrix::identity(4)),
            Arc::new(Phong::default()),
            true,
        )
        .unwrap();

        // Assert
        assert_eq!(3, result.vertex_normals.len());
        assert_eq!(Tuple::vector(0.0, 0.0, 1.0), result.vertex_normals[0]);
        assert_eq!(Tuple::vector(0.707, 0.0, -0.707), result.vertex_normals[1]);
        assert_eq!(Tuple::vector(1.0, 2.0, 3.0), result.vertex_normals[2]);
    }

    #[test]
    fn given_an_obj_file_with_faces_with_normals_when_parsing_should_correctly_parse_out_each_one()
    {
        // Arrange
        let file_path = "tests/obj_files/faces_with_normals.obj";

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

        assert_eq!(2, group.num_of_children());

        let t1_points = group.get_child(0).unwrap().points();
        let t1_normals = group.get_child(0).unwrap().normals();

        let t2_points = group.get_child(1).unwrap().points();
        let t2_normals = group.get_child(1).unwrap().normals();

        assert_eq!(result.vertices[0], t1_points.0);
        assert_eq!(result.vertices[1], t1_points.1);
        assert_eq!(result.vertices[2], t1_points.2);
        assert_eq!(result.vertex_normals[2], t1_normals.0);
        assert_eq!(result.vertex_normals[0], t1_normals.1);
        assert_eq!(result.vertex_normals[1], t1_normals.2);

        assert_eq!(t1_points.0, t2_points.0);
        assert_eq!(t1_points.1, t2_points.1);
        assert_eq!(t1_points.2, t2_points.2);
        assert_eq!(t1_normals.0, t2_normals.0);
        assert_eq!(t1_normals.1, t2_normals.1);
        assert_eq!(t1_normals.2, t2_normals.2);
    }
}
