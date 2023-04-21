type Fr = char;

/// A Sprite is an atomic, visual asset.
pub struct Sprite {
    /// A Sprite is made of at least one visual frame. Multiple frames will
    ///  be cycled periodically (e.g., to create ocean waves or a flapping
    ///  flag).
    frames: Vec<Fr>,
    period: usize,
    offset: usize,
}

//type Drawer<Fr, Cx> = dyn FnMut(Fr, Cx);

impl Sprite {
    pub fn new(frames: Vec<Fr>, period: usize, offset: usize) -> Sprite {
        Sprite {
            frames,
            period,
            offset,
        }
    }
    pub fn new_solid(frame: Fr) -> Sprite {
        Sprite {
            frames: vec![frame],
            period: 1,
            offset: 0,
        }
    }
    pub fn draw(&self, beat: usize) -> Fr {
        let i = (self.offset + beat / self.period) % self.frames.len();
        self.frames[i]
    }
    /*    /// A Sprite is drawn abstractly: the draw method takes a method for
    ///  drawing a frame to whatever medium (e.g., a character to the
    ///  terminal, a rectangle of pixels to a window, or who knows what
    ///  else). Context is passed as well and only means something to
    ///  the implementer of the passed Drawer.
    pub fn draw<Cx>(&self, draw: &Drawer<Fr, Cx>, context: Cx, beat: usize) {
        let i = (self.offset + beat/self.period) % self.frames.len();
        draw(self.frames[i], context)
    }*/
}
