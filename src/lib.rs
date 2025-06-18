use bevy::prelude::*;
// In sf-plugin-template/src/lib.rs
/// Trait that plugins must implement
pub trait GamePlugin: Plugin + dyn_clone::DynClone {
    /// Get plugin name for debugging and identification
    fn name(&self) -> &'static str;
    
    /// Get plugin version
    fn version(&self) -> &'static str;
}

// Implement clone for trait objects
dyn_clone::clone_trait_object!(GamePlugin);

/// Example plugin implementation
#[derive(Default, Clone)]
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_system)
           .add_systems(Startup, setup_plugin);
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
    commands.spawn((
        TemplateComponent { value: 0.0 },
        Name::new("TemplateEntity"),
    ));
}

fn update_system(query: Query<&TemplateComponent>) {
    for _component in &query {
        // Do something with component
    }
}

// Opaque pointer type for FFI safety
#[repr(C)]
pub struct PluginHandle {
    plugin: Box<dyn GamePlugin>,
}

impl PluginHandle {
    /// Create a new plugin handle
    pub fn new(plugin: Box<dyn GamePlugin>) -> Self {
        Self { plugin }
    }
    
    /// Get a reference to the plugin
    pub fn get_plugin(&self) -> &dyn GamePlugin {
        &*self.plugin
    }
    
    /// Get a mutable reference to the plugin
    pub fn get_plugin_mut(&mut self) -> &mut dyn GamePlugin {
        &mut *self.plugin
    }
    
    /// Convert the handle into the inner plugin
    pub fn into_plugin(self) -> Box<dyn GamePlugin> {
        self.plugin
    }
}

/// Creates a new plugin instance and returns an opaque handle to it
/// 
/// # Safety
/// The returned pointer must be freed with `destroy_plugin` when no longer needed
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut PluginHandle {
    let plugin = Box::new(TemplatePlugin::default());
    let handle = Box::new(PluginHandle {
        plugin: plugin as Box<dyn GamePlugin>,
    });
    Box::into_raw(handle)
}


#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_plugin_initialization() {
        let mut app = App::new();
        app.add_plugins(TemplatePlugin);
        
        // Test plugin initialization logic
    }
}
