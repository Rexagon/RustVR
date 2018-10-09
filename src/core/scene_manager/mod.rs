pub trait Scene {
    fn on_init(&mut self) {}
    fn on_close(&mut self) {}

    fn on_enter(&mut self) {}
    fn on_leave(&mut self) {}

    fn on_update(&mut self, _dt: f64) {}
}

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: vec![]
        }
    }

    pub fn has_scenes(&self) -> bool {
        self.scenes.len() > 0
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene>) {
        if let Some(last_scene) = self.scenes.last_mut() {
            last_scene.on_leave();
        }

        self.scenes.push(scene);

        if let Some(last_scene) = self.scenes.last_mut() {
            last_scene.on_init();
            last_scene.on_enter();
        }
    }

    pub fn remove_scene(&mut self) {
        if let Some(last_scene) = self.scenes.last_mut() {
            last_scene.on_leave();
            last_scene.on_close();
        }

        self.scenes.pop();

        if let Some(last_scene) = self.scenes.last_mut() {
            last_scene.on_enter();
        }
    }

    pub fn clear(&mut self) {
        while let Some(mut scene) = self.scenes.pop() {
            scene.on_leave();
            scene.on_close();
        }
    }
}