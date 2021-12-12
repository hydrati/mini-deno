((window) => {
    const {
        Symbol,
        SymbolToStringTag,
        TypeError,
        SymbolIterator,
    } = window.__bootstrap.primordials;

    const {
        opOsEnvGet,
        opOsEnvGetEntries,
        opOsEnvGetValues,
        opOsEnvGetKeys,
        opOsEnvGetKv,
        opOsEnvSet,
        opOsEnvHas,
        opOsEnvDelete
    } = window.__bootstrap.os.ops;

    const illegalConstructorKey = Symbol("illegalConstructorKey");

    class Environ {
        constructor(key = null) {
            if (key !== illegalConstructorKey) {
                throw new TypeError("Illegal constructor.");
            }
        }

        get[SymbolToStringTag]() {
            return "Environ";
        }

        [SymbolIterator]() {
            return this.entries()[SymbolIterator]();
        }

        record() {
            return opOsEnvGetKv();
        }

        entries() {
            return opOsEnvGetEntries();
        }

        keys() {
            return opOsEnvGetKeys();
        }

        values() {
            return opOsEnvGetValues();
        }

        has(key) {
            return opOsEnvHas(key);
        }

        set(key, value) {
            opOsEnvSet(key, value)
        }

        get(key) {
            return opOsEnvGet(key);
        }

        delete(key) {
            opOsEnvDelete(key);
        }
    }

    const environ = new Environ(illegalConstructorKey);

    if (typeof window.__bootstrap.os === "undefined") {
        window.__bootstrap.os = {}
    }

    window.__bootstrap.os.environ = {
        Environ,
        environ,
    }
})(this);