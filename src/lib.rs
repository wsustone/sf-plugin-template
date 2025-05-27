//! # StrategyForge Plugin Template
//! 
//! This is a template for creating StrategyForge plugins.

use bevy::prelude::*;
use dyn_clone::DynClone;

/// Trait that plugins must implement
pub trait GamePlugin: Plugin + DynClone + Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
}

dyn_clone::clone_trait_object!(GamePlugin);

/// Example plugin implementation
#[derive(Default, Clone)]
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_plugin)
           .add_systems(Update, update_system);
    }
}

impl GamePlugin for TemplatePlugin {
    fn name(&self) -> &'static str {
        "TemplatePlugin"
    }

    fn version(&self) -> &'static str {
        "0.1.0"
    }
}

#[derive(Component)]
pub struct TemplateComponent {
    pub value: f32,
}

fn setup_plugin(mut commands: Commands) {
    info!("TemplatePlugin initialized!");
    commands.spawn(TemplateComponent { value: 1.0 });
}

fn update_system(query: Query<&TemplateComponent>) {
    for component in &query {
        trace!("Template component value: {}", component.value);
    }
}

// Required for dynamic loading
#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut dyn GamePlugin {
    Box::into_raw(Box::new(TemplatePlugin::default()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_plugin_initialization() {
        let mut app = App::new();
        app.add_plugins(TemplatePlugin);
        
        // Test that the plugin initializes without panicking
        app.update();
    }
}
