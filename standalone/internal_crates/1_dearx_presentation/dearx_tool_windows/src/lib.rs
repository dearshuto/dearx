mod main_window_view_model;
mod object_tree_view_model;
mod property_window_view_model;

pub use main_window_view_model::MainWindowViewModel;
pub use object_tree_view_model::ObjectTreeViewModel;
pub use property_window_view_model::PropertyWindowViewModel;

#[derive(Default)]
pub struct ServiceProvider;
