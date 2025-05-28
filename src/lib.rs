use bevy::prelude::*;

/// Trait that plugins must implement
pub trait GamePlugin: Plugin {
    /// Get plugin name for debugging and identification
    fn name(&self) -> &'static str;
    
    /// Get plugin version
    fn version(&self) -> &'static str;
}

/// Marker component for menu items
#[derive(Component)]
pub struct MenuItem {
    /// The plugin that owns this menu item
    pub plugin_name: String,
    /// Whether this menu item is currently selected
    pub selected: bool,
}

/// Menu content container component
#[derive(Component)]
pub struct MenuContent;

/// Trait for plugins that can add menu items
pub trait MenuItemPlugin: Send + Sync + 'static {
    /// Get the name of this menu item for display
    fn menu_name(&self) -> &'static str;
    
    /// Add a menu item button to the specified parent entity
    fn add_menu_item(&self, world: &mut World, parent: Entity);
    
    /// Handle when this menu item is selected
    fn on_selected(&self, world: &mut World, content_entity: Entity);
    
    /// Clone the plugin
    fn clone_box(&self) -> Box<dyn MenuItemPlugin>;
}

// Allow Box<dyn MenuItemPlugin> to be cloned
impl Clone for Box<dyn MenuItemPlugin> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

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

impl MenuItemPlugin for TemplatePlugin {
    fn menu_name(&self) -> &'static str {
        "Template"
    }
    
    fn add_menu_item(&self, world: &mut World, parent: Entity) {
        // Add menu item button to the menu
        let mut entity = world.entity_mut(parent);
        entity.with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.25, 0.25, 0.25).into(),
                    ..default()
                },
                MenuItem {
                    plugin_name: self.menu_name().to_string(),
                    selected: false,
                }
            )).with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        self.menu_name(),
                        TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..default()
                        }
                    )
                );
            });
        });
    }
    
    fn on_selected(&self, world: &mut World, content_entity: Entity) {
        // Display template content when this menu item is selected
        let mut entity = world.entity_mut(content_entity);
        entity.despawn_descendants();
        entity.with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|parent| {
                // Title
                parent.spawn(
                    TextBundle::from_section(
                        "Template Plugin",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        }
                    ).with_style(Style {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    })
                );
                
                // Content
                parent.spawn(
                    TextBundle::from_section(
                        "This is a template plugin for StrategyForge.",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        }
                    )
                );
            });
        });
    }
    
    fn clone_box(&self) -> Box<dyn MenuItemPlugin> {
        Box::new(self.clone())
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
    for component in &query {
        // Do something with component
    }
}

// Opaque pointer type for FFI safety
#[repr(C)]
pub struct PluginHandle {
    plugin: Box<dyn GamePlugin>,
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

/// Destroys a plugin instance created with `create_plugin`
/// 
/// # Safety
/// The handle must be a valid pointer returned by `create_plugin`
/// and must not be used after this call
#[no_mangle]
pub unsafe extern "C" fn destroy_plugin(handle: *mut PluginHandle) {
    if !handle.is_null() {
        let _ = Box::from_raw(handle);
    }
}

// Legacy function for backward compatibility
#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut std::ffi::c_void {
    create_plugin() as *mut std::ffi::c_void
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
