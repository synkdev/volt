# Volt
Volt is an extremely fast and efficient GPU-accelerated UI library written completely in pure, safe Rust. It is powered by WGPU which means that most of the rendering is done on the GPU, making Volt fast. It uses [Vello](https://github.com/linebender/vello) for rendering with WGPU. Unlike other traditional vector graphics renderers that use vertex and fragment shaders, Vello is powered by compute shaders which allows for lightning fast computation.