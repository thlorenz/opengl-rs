# Playground

Implementing some examples to be viewed in
[glslViewer](https://github.com/patriciogonzalezvivo/glslViewer).

Using [`.obj`](https://en.wikipedia.org/wiki/Wavefront_.obj_file) files to input vertices.

The advantage is that this allows quick experimentation as the window reloads each time a
shader is edited.

## Triangle Example

Run as follows: `cd play/01_triangle`

```sh
# Show the triangle applying glslViewer default shaders
glslViewer triangle.obj

# Show the triangle applying frag shader only
glslViewer triangle.obj triangle.frag

# Show the triangle applying both shaders
glslViewer triangle.obj triangle.frag
```

Note that unfortunately the shaders have to be adapted to remove version declarations and to be compatible
with older OpenGL versions.

Omitting vertex shaders provides 3D space which allows you to rotate around the triangle.
