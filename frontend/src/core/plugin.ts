export interface Plugin {
  name: string;
  version: string;
  initialize(): void;
  register(): void;
}

export interface PluginRegistry {
  register(plugin: Plugin): void;
  getPlugin(name: string): Plugin | undefined;
  getAllPlugins(): Plugin[];
  initializeAll(): void;
  registerAll(): void;
}

class PluginManager implements PluginRegistry {
  private plugins: Map<string, Plugin> = new Map();

  register(plugin: Plugin): void {
    this.plugins.set(plugin.name, plugin);
  }

  getPlugin(name: string): Plugin | undefined {
    return this.plugins.get(name);
  }

  getAllPlugins(): Plugin[] {
    return Array.from(this.plugins.values());
  }

  initializeAll(): void {
    for (const plugin of this.plugins.values()) {
      plugin.initialize();
    }
  }

  registerAll(): void {
    for (const plugin of this.plugins.values()) {
      plugin.register();
    }
  }
}

export const pluginManager = new PluginManager();
