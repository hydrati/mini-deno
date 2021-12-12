((window) => {
    const {
        ObjectFreeze,
        ObjectDefineProperty,
        SymbolToStringTag,
    } = window.__bootstrap.primordials;

    const version = {
        neptune: "0.0.1-testonly",
        v8: "9.7.106.16",
        core: "0.110.0"
    };

    ObjectDefineProperty(version, SymbolToStringTag, {
        value: "Versions",
        enumerable: false,
        configurable: false,
        writable: false,
    })

    window.__bootstrap.version = ObjectFreeze(version);
})(this);