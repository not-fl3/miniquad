set -ex

glslangValidator -V quad.vert -o quad.vert.spv --invert-y
glslangValidator -V quad.frag -o quad.frag.spv --invert-y

spirv-cross quad.vert.spv --es --version 100 --output quad_100.vert
spirv-cross quad.frag.spv --es --version 100 --output quad_100.frag

spirv-cross quad.vert.spv --msl --output quad.vert.metal --rename-entry-point main vertex_function vert
spirv-cross quad.frag.spv --msl --output quad.frag.metal --rename-entry-point main fragment_function frag

xcrun -sdk macosx metal -c ./*.metal -mmacosx-version-min=10.11 -MO -gline-tables-only
xcrun -sdk macosx metallib ./*.air -o quad.metallib
rm ./*.air
