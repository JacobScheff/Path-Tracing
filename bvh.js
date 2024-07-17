const input_path = "objects/knight.bin"
const output_path = "objects/knight_bvh.bin"

const fs = require('fs')

// Bounding Box class
class BoundingBox {
    constructor(min_x, min_y, min_z, max_x, max_y, max_z) {
        this.min_x = min_x
        this.min_y = min_y
        this.min_z = min_z
        this.max_x = max_x
        this.max_y = max_y
        this.max_z = max_z
        // this.center = [(min_x + max_x) / 2, (min_y + max_y) / 2, (min_z + max_z) / 2]
    }

    // Grow the bounding box to include the given point
    grow_to_include_point(point) {
        this.min_x = Math.min(this.min_x, point[0])
        this.min_y = Math.min(this.min_y, point[1])
        this.min_z = Math.min(this.min_z, point[2])
        this.max_x = Math.max(this.max_x, point[0])
        this.max_y = Math.max(this.max_y, point[1])
        this.max_z = Math.max(this.max_z, point[2])
        // this.center = [(this.min_x + this.max_x) / 2, (this.min_y + this.max_y) / 2, (this.min_z + this.max_z) / 2]
    }
    
    // Grow the bounding box to include the given triangle
    grow_to_include_triangle(triangle) {
        this.grow_to_include_point(triangle[0])
        this.grow_to_include_point(triangle[1])
        this.grow_to_include_point(triangle[2])
    }
    
}

// Read the input file and convert to a 1d array of floats
console.log(`Reading input file: ${input_path}`)
const input = fs.readFileSync(input_path)
let vertices = []
for (let i = 0; i < input.length; i += 4) {
    vertices.push(input.readFloatLE(i))
}

// Convert to 3d array of triangles->vertices->coordinates
console.log("Converting to 3d array")
let triangles = []
for (let i = 0; i < vertices.length; i += 9) {
    triangles.push([
        [vertices[i], vertices[i + 1], vertices[i + 2]],
        [vertices[i + 3], vertices[i + 4], vertices[i + 5]],
        [vertices[i + 6], vertices[i + 7], vertices[i + 8]]
    ])
}

// Create the BVH
console.log("Creating BVH")
