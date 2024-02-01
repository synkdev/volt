pub struct WindowOptions {
    pub size: (u32, u32),
    pub min_size: (u32, u32),
    pub max_size: (u32, u32),
    pub position: (i32, i32),
    pub resizable: bool,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub blur: bool,
    pub decorations: bool,
    pub window_icon: &'static str,
    pub active: bool,
}
