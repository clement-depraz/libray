# libray
School Rust project to implement a ray-tracing library

It uses a previous library to handle and save ppm files.

It also uses the rayon package to parallelize ray computing trough threads and improve global speed.

The lib is accessible through the description of a scene send in a json file.

An example is available, to try it, just run:
`cd examples && cargo run src/test.json out.ppm`

The first time it will build the library but when executed again, it will just compute the scene.
