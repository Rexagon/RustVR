pub trait Scene {
    fn on_init(&mut self) {}
    fn on_close(&mut self) {}

    fn on_enter(&mut self) {}
    fn on_leave(&mut self) {}

    fn on_update(&mut self, _dt: f64) {}
}

pub struct SceneManager {
    scenes: Vec<Box<Scene>>
}

impl SceneManager {
    pub fn new<T: 'static + Scene>(first_scene: Box<T>) -> SceneManager {
        let mut scene_manager = SceneManager {
            scenes: vec![]
        };

        scene_manager.add_scene(first_scene);

        scene_manager
    }

    pub fn add_scene<T: 'static + Scene>(&mut self, scene: Box<T>) {
        if let Some(last_scene) = self.scenes.last_mut() {
            last_scene.on_leave();
        }

        self.scenes.push(scene);

        if let Some(last_scene) = self.scenes.last_mut() {
            last_scene.on_init();
            last_scene.on_enter();
        }
    }

    pub fn remove_scene<T: 'static + Scene>(&mut self) {
        if let Some(last_scene) = self.scenes.last_mut() {
            last_scene.on_leave();
            last_scene.on_close();
        }

        self.scenes.pop();

        if let Some(last_scene) = self.scenes.last_mut() {
            last_scene.on_enter();
        }
    }
}