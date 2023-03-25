use dearx_macro::Immutable;
use dearx_workspace::DocumentId;
use im::HashMap;
use std::sync::Arc;

use crate::{GameObject, GameObjectId, ModelContent};

#[derive(Default, Immutable)]
pub struct DearxProject {
    pub game_object: Arc<HashMap<GameObjectId, GameObject>>,
    pub selections: Arc<im::Vector<DocumentId>>,
    pub model_contents: Arc<Vec<ModelContent>>,

    // 動作確認用のデータ。将来的に消す。
    pub color: [f32; 3],
    pub vertives: Vec<f32>,
    pub indices: Vec<u32>,
}

impl DearxProject {
    pub fn new() -> Self {
        Self {
            game_object: Default::default(),
            selections: Default::default(),
            model_contents: Default::default(),
            color: Default::default(),
            vertives: Default::default(),
            indices: Default::default(),
        }
    }
}
