# Path Tracing
This is a simple path tracer that can render one or more spheres and one complex, multi-triangled object in a scence with realistic lighting. The object and spheres can each have a different color, emission strength and color if it is a light source, and smoothness.

## How to use
1. Upload the complex object as a .stl file in the objects folder
2. In prepare_data/data_formatter.js, update the input path to your .stl file and the output path to the name you want the resulting .bin file to have
3. Run `cd prepare_data` then `node data_formatter.js` in the terminal
4. Update the input and output file names in prepare_data/src/main.rs, and adjust the max_depth for the BVH creation if necessary
5. Run `cargo run --release`
6. Run `cd ../` in the terminal
7. Modify the file names being read in main.rs, and add or remove any desired spheres
8. In shader.wgsl, update any necessary information at the top of the file, and update the material information for the complex object in the ray_triangle() function
9. Run `cargo run --release`

## Example Scenes
![Dragon With Red Light](https://github.com/Snowplou/Path-Tracing/blob/main/public/ReadMe1.png?raw=true)
![Dragon With Red Light, Alternate View](https://github.com/Snowplou/Path-Tracing/blob/main/public/ReadMe2.png?raw=true)
![Blue Reflective Teapot](https://github.com/Snowplou/Path-Tracing/blob/main/public/ReadMe3.png?raw=true)
