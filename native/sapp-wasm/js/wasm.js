
class Wasm {
    
    constructor() {
        this.importObject = null;
        this.plugins = null;
        this.exports = null;
        this.memory = null;
    }
    
    load(path, plugins = null) {
        this.importObject = {
            env: {...(window.wasmImports ?? {})}
        };
        
        const request = fetch(path);
        
        this.plugins = plugins;
        this.registerPlugins(this.plugins);

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            WebAssembly.instantiateStreaming(request, this.importObject)
                .then(this.initialiseObject.bind(this));
            return;
        }

        request
            .then((x) => {
                return x.arrayBuffer();
            })
            .then((bytes) => {
                return WebAssembly.instantiate(bytes, this.importObject);
            })
            .then(this.initialiseObject.bind(this));
    }
    
    initPlugins(plugins) {
        if (plugins == null)
            return;

        plugins.forEach(plugin => {
            if (plugin.on_init == null)
                return;
            plugin.on_init();
        });
    }

    registerPlugins(plugins) {
        if (plugins == null)
            return;

        plugins.forEach(plugin => {
            if (plugin.on_init == null)
                return;
            plugin.register_plugin(this.importObject);
        });
    }
    
    initialiseObject(object) {
        this.memory = object.instance.exports.memory;
        this.exports = object.instance.exports;

        this.initPlugins(this.plugins);
        object.instance.exports.main();
    }
    
}

window.wasm = new Wasm();