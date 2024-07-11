const input_path = "objects/knight.stl"
const output_path = "objects/knight.bin"

const fs = require('fs')

// Read the input file
console.log(`Reading input file: ${input_path}`)
const input = fs.readFileSync(input_path).toString()

// Convert the input file to a 1d array of floats that store the vertices of the triangles
console.log("Parisng data")
const vertices = input.split("\n").filter(line => line.includes("vertex")).map(line => line.split("vertex ")[1].split(" ").map(Number)).flat()

// Convert to binary
console.log("Converting to binary")
const buffer = Buffer.alloc(vertices.length * 4)
vertices.forEach((vertex, i) => buffer.writeFloatLE(vertex, i * 4))

// Write the binary data to the output file
console.log(`Writing binary data to: ${output_path}`)
fs.writeFileSync(output_path, buffer)

console.log("Done!")