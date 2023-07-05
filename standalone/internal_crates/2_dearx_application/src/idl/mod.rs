use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    model_names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    name: String,
    transform_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModelTable {
    models: Vec<Model>,
}

#[derive(Serialize, Deserialize)]
pub struct Transform {
    name: String,
    translation: Float3,
    rotation: Float3,
    scale: Float3,
}

#[derive(Serialize, Deserialize)]
pub struct Float3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize)]
pub struct TransformTable {
    transforms: Vec<Transform>,
}

struct Serializer;
impl Serializer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize<T: Serialize>(&self, data_model: &T) -> String {
        let str = serde_json::to_string(data_model).unwrap();
        str
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(
        &self,
        data: &'a str,
    ) -> Result<T, serde_json::Error> {
        let result = serde_json::from_str(data);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::idl::Scene;

    use super::{ModelTable, Serializer};

    #[test]
    fn deserialize_scene() {
        let scene = Serializer::new()
            .deserialize::<Scene>(
                "
{
    \"model_names\": [\"A\", \"B\"]
}",
            )
            .unwrap();

        assert_eq!(scene.model_names[0], "A");
        assert_eq!(scene.model_names[1], "B");
    }

    #[test]
    fn deserialize_model_table() {
        let model_table = Serializer::new()
            .deserialize::<ModelTable>(
                "
{
    \"models\": [
        { \"name\": \"A\" },
        { \"name\": \"B\" }
]
}",
            )
            .unwrap();

        assert_eq!(model_table.models[0].name, "A");
        assert_eq!(model_table.models[1].name, "B");
    }
}
