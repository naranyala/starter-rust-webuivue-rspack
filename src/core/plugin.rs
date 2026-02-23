use webui_rs::webui;

pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn register(&self, window: &mut webui::Window);
    fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[allow(dead_code)]
pub trait PluginRegistry: Send + Sync {
    fn register_plugin(&self, plugin: Box<dyn Plugin>);
    fn get_plugin(&self, name: &str) -> Option<&dyn Plugin>;
    fn get_all_plugins(&self) -> Vec<&dyn Plugin>;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    #[allow(dead_code)]
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins
            .iter()
            .find(|p| p.name() == name)
            .map(|p| p.as_ref() as &dyn Plugin)
    }

    #[allow(dead_code)]
    pub fn get_all_plugins(&self) -> Vec<&dyn Plugin> {
        self.plugins
            .iter()
            .map(|p| p.as_ref() as &dyn Plugin)
            .collect()
    }

    pub fn init_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        for plugin in &self.plugins {
            plugin.init()?;
        }
        Ok(())
    }

    pub fn register_all(&self, window: &mut webui::Window) {
        for plugin in &self.plugins {
            plugin.register(window);
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
