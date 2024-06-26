use engine::{Backend, Engine, EngineSettings, GameObject, Scene, ThrustlerError, Vertex};

fn main() -> engine::Result<(), ThrustlerError> {
    Ok(
        Engine::new_with_settings(
            EngineSettings {
                frames_per_second: 1,
                backend: Backend::Wgpu,
                ..EngineSettings::default()
            }
        )?
            .add_scene(Test {
                game_objects: vec![
                    GameObject::new(vec![
                        Vertex::new([-1.0, 1.0]),
                        Vertex::new([0.0, -1.0]),
                        Vertex::new([1.0, 1.0]),
                    ])
                ]
            })
            .start()?,
    )
}

struct Test {
    game_objects: Vec<GameObject>,
}

impl Scene for Test {
    fn on_start(&mut self) {}

    fn on_update(&mut self) {}

    fn on_destroy(&mut self) {}

    fn get_scene_objects(&self) -> &Vec<GameObject> {
        self.game_objects.as_ref()
    }
}