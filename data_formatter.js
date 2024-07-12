const input_path = "objects/knight.stl"
const output_path = "objects/knight.bin"

const fs = require('fs')
const scale = 0.1

// Read the input file
console.log(`Reading input file: ${input_path}`)
const input = fs.readFileSync(input_path).toString()

// Convert the input file to a 1d array of floats that store the vertices of the triangles
console.log("Parisng data")
let vertices = input.split("\n").filter(line => line.includes("vertex")).map(line => line.split("vertex ")[1].split(" ").map(Number)).flat().map(n => n * scale)
let normals = input.split("\n").filter(line => line.includes("facet normal")).map(line => line.split("facet normal ")[1].split(" ").map(Number)).flat()

// Combine the vertices and normals into one array. Normals go after the 3 vertices
console.log("Combining vertices and normals")
let combined = []
for (let i = 0; i < vertices.length; i += 9) {
    combined.push(...vertices.slice(i, i + 9))
    combined.push(...normals.slice(i / 9 * 3, i / 9 * 3 + 3))
}

// Convert to binary
console.log("Converting to binary")
const buffer = Buffer.alloc(combined.length * 4)
combined.forEach((n, i) => buffer.writeFloatLE(n, i * 4))

// Write the binary data to the output file
console.log(`Writing binary data to: ${output_path}`)
fs.writeFileSync(output_path, buffer)

console.log("Done!")
console.log("Amount of triangles: ", vertices.length / 9)