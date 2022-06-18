pub trait IPropertyWindowViewModel {
    fn get_translation(&self) -> (f32, f32, f32);
}

pub struct PropertyWindow<TViewModel: IPropertyWindowViewModel> {
    #[allow(dead_code)]
    view_model: TViewModel,
}

impl<TViewModel: IPropertyWindowViewModel> PropertyWindow<TViewModel> {
    pub fn new(view_model: TViewModel) -> Self {
        Self { view_model }
    }

    pub fn process_frame(&mut self, context: &egui::CtxRef) {
        epi::egui::Window::new("Property Window").show(context, |ui| {
            ui.label("Hello World");
            ui.label("I am PropertyWindow");
        });
    }
}
