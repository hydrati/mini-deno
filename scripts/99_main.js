((window) => {

    const {
        ObjectAssign,
        ObjectDefineProperty,
        ObjectDefineProperties,
        ObjectFreeze,
        ObjectSetPrototypeOf,
        globalThis,
    } = window.__bootstrap.primordials;

    const core = Deno.core;

    const {
        Console,
    } = window.__bootstrap.console;

    const {
        Window,
        windowConstructorDescriptor,
    } = window.__bootstrap.globalInterfaces;

    const {
        atob,
        btoa
    } = window.__bootstrap.base64;

    const {
        URL,
        URLSearchParams,
    } = window.__bootstrap.url;

    const {
        URLPattern,
    } = window.__bootstrap.urlPattern;

    const {
        Blob,
        File,
    } = window.__bootstrap.file;

    const {
        TextEncoder,
        TextDecoder,
        TextEncoderStream,
        TextDecoderStream,
    } = window.__bootstrap.encoding;

    const {
        EventTarget
    } = window.__bootstrap.eventTarget;

    const {
        DOMException
    } = window.__bootstrap.domException;


    const util = window.__bootstrap.util;

    const {
        clearInterval,
        setInterval,
        clearTimeout,
        setTimeout,
        sleepSync,
        handleTimerMacrotask,
    } = window.__bootstrap.timers;

    const {
        PerformanceEntry,
        PerformanceMark,
        PerformanceMeasure,
        Performance,
        performance,
    } = window.__bootstrap.performance;

    const {
        Deferred,
        ByteLengthQueuingStrategy,
        CountQueuingStrategy,
        ReadableStream,
        ReadableStreamDefaultReader,
        TransformStream,
        WritableStream,
        WritableStreamDefaultWriter,
        WritableStreamDefaultController,
        ReadableByteStreamController,
        ReadableStreamBYOBReader,
        ReadableStreamBYOBRequest,
        ReadableStreamDefaultController,
        TransformStreamDefaultController,
    } = window.__bootstrap.streams;

    const denoNs_os = window.__bootstrap.denoNs_os;
    const stdio_console = new Console(core.print);
    const windowObj = ObjectSetPrototypeOf(globalThis, Window.prototype);
    const versionObj = window.__bootstrap.version;

    function rawPrintln(...v) {
        Deno.core.print(v.join(" ") + "\n");
    }

    function sumSync(...a) {
        return Deno.core.opSync("op_sum_sync", a);
    }

    function sumAsync(...a) {
        return Deno.core.opAsync("op_sum_async", a);
    }

    ObjectDefineProperties(globalThis, {
        console: util.readOnly(stdio_console),
        Neptune: util.readOnly({
            rawPrintln,
            version: versionObj.neptune,
            versions: versionObj,
            __bootstrap: ObjectSetPrototypeOf({}, Object.assign({
                [Symbol.toStringTag]: "BootstrapNamespace"
            }, window.__bootstrap))
        }),
        window: util.readOnly(windowObj)
    })

    // keep deno ns ref
    const deno_ns = window.Deno;

    function _get_ns() {
        return deno_ns;
    }

    ObjectDefineProperties(Deno, {
        timers: util.readOnly({
            sleepSync,
            setTimeout,
            setInterval,
            clearTimeout,
            clearInterval,
            __activateTimerMacrotask: util.onceFunc(() => core.setMacrotaskCallback(handleTimerMacrotask)),
            [Symbol.toStringTag]: "Timers"
        }),
        os: util.readOnly(denoNs_os),
        exit: util.readOnly(denoNs_os.exit),
        version: util.readOnly(versionObj.neptune),
        versions: util.readOnly(versionObj)
    });

    ObjectDefineProperties(globalThis, {
        atob: util.writable(atob),
        btoa: util.writable(btoa),
        URL: util.writable(URL),
        URLSearchParams: util.writable(URLSearchParams),
        URLPattern: util.writable(URLPattern),
        Blob: util.writable(Blob),
        File: util.writable(File),
        TextEncoder: util.writable(TextEncoder),
        TextDecoder: util.writable(TextDecoder),
        TextEncoderStream: util.writable(TextEncoderStream),
        TextDecoderStream: util.writable(TextDecoderStream),
        EventTarget: util.writable(EventTarget),
        DOMException: util.writable(DOMException),
        clearTimeout: util.writable(clearTimeout),
        setInterval: util.writable(setInterval),
        clearInterval: util.writable(clearInterval),
        setTimeout: util.writable(setTimeout),
        Console: util.writable(Console),
        Window: windowConstructorDescriptor,

        Deferred: util.writable(Deferred),
        ByteLengthQueuingStrategy: util.writable(ByteLengthQueuingStrategy),
        CountQueuingStrategy: util.writable(CountQueuingStrategy),
        ReadableStream: util.writable(ReadableStream),
        ReadableStreamDefaultReader: util.writable(ReadableStreamDefaultReader),
        TransformStream: util.writable(TransformStream),
        WritableStream: util.writable(WritableStream),
        WritableStreamDefaultWriter: util.writable(WritableStreamDefaultWriter),
        WritableStreamDefaultController: util.writable(WritableStreamDefaultController),
        ReadableByteStreamController: util.writable(ReadableByteStreamController),
        ReadableStreamBYOBReader: util.writable(ReadableStreamBYOBReader),
        ReadableStreamBYOBRequest: util.writable(ReadableStreamBYOBRequest),
        ReadableStreamDefaultController: util.writable(ReadableStreamDefaultController),
        TransformStreamDefaultController: util.writable(TransformStreamDefaultController),

        performance: util.writable(performance),
        Performance: util.writable(Performance),
        PerformanceMeasure: util.writable(PerformanceMeasure),
        PerformanceMark: util.writable(PerformanceMark),
        PerformanceEntry: util.writable(PerformanceEntry),
    });

    // ObjectDefineProperty(globalThis, "Deno", util.nonEnumerable(window.Deno));

    delete globalThis.__bootstrap

})(this);