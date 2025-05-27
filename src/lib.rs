//! # StrategyForge Plugin Template
//! 
//! This is a template for creating StrategyForge plugins.
//! Replace the content with your plugin's functionality.

use bevy::prelude::*;

/// The main plugin struct that will be loaded by the game
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "dev")]
        app.add_systems(Startup, setup_developer_tools);
        
        app.add_systems(Startup, setup_plugin)
           .add_systems(Update, update_system);
           
        #[cfg(feature = "dev")]
        app.register_type::<TemplateComponent>();
    }
}

// Example component
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TemplateComponent {
    pub value: f32,
}

fn setup_plugin(mut commands: Commands) {
    // Initialize plugin resources and entities here
    info!("TemplatePlugin initialized!");
}

fn update_system(time: Res<Time>, mut query: Query<&mut Transform, With<TemplateComponent>>) {
    // Example update system
    for mut transform in &mut query {
        transform.rotate_z(time.delta_seconds());
    }
}

#[cfg(feature = "dev")]
fn setup_developer_tools(mut commands: Commands) {
    // Add development-only systems and resources here
    info!("Developer tools enabled for TemplatePlugin");
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
