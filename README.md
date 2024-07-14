# Raytracer

Completed rust version of the first eleven chapters of "The Ray Tracer Challenge" by Jamis Buck.

Made the rendering loop parallel with simple 1D row-wise partitioning of the image to improve render times.

## Example Input

`cargo run --release tmp.ppm 1000 1000`

## Example Output

![alt text](https://raw.githubusercontent.com/lydiasamuel/raytracer/main/example_output.png)
