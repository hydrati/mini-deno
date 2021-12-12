((window) => {

    const {
        SymbolFor
    } = window.__bootstrap.primordials;

    const core = Deno.core

    function opOsPhysicalCoreCount() {
        return core.opSync("op_os_physical_core_count");
    }

    function opOsTargetArch() {
        return core.opSync("op_os_target_arch");
    }

    function opOsTargetEnv() {
        return core.opSync("op_os_target_env");
    }

    function opOsTargetOs() {
        return core.opSync("op_os_target_os");
    }

    function opOsHostname() {
        return core.opSync("op_os_hostname");
    }

    function opOsVersion() {
        return core.opSync("op_os_version");
    }

    function opOsLongVersion() {
        return core.opSync("op_os_long_version");
    }

    function opOsKernelVersion() {
        return core.opSync("op_os_kernel_version")
    }

    function opOsEnvGet(key) {
        const val = core.opSync("op_os_env_get", key);
        if (typeof val !== "string") {
            return undefined;
        } else {
            return val;
        }
    }

    function opOsEnvSet(key, value) {
        return core.opSync("op_os_env_set", key, value);
    }

    function opOsEnvDelete(key) {
        return core.opSync("op_os_env_delete", key);
    }

    function opOsEnvHas(key) {
        return core.opSync("op_os_env_has", key);
    }

    function opOsEnvGetKv() {
        return core.opSync("op_os_env_get_kv");
    }

    function opOsEnvGetValues() {
        return core.opSync("op_os_env_get_values");
    }

    function opOsEnvGetKeys() {
        return core.opSync("op_os_env_get_keys");
    }

    function opOsEnvGetEntries() {
        return core.opSync("op_os_env_get_entries");
    }

    function opOsCurrentPpid() {
        return core.opSync("op_os_current_ppid");
    }

    function opOsCurrentPid() {
        return core.opSync("op_os_current_pid");
    }

    function opOsLoadavg() {
        return core.opSync("op_os_loadavg");
    }

    function opOsGlobalProcessorInfo() {
        return core.opSync("op_os_global_processor_info");
    }

    function opOsProcessorsInfo() {
        return core.opSync("op_os_processors_info");
    }

    function opOsTotalMemoryKB() {
        return core.opSync("op_os_total_memory");
    }

    function opOsFreeMemoryKB() {
        return core.opSync("op_os_free_memory");
    }

    function opOsAvailableMemoryKB() {
        return core.opSync("op_os_available_memory");
    }

    function opOsUsedMemoryKB() {
        return core.opSync("op_os_used_memory");
    }

    let exitHandler = null;

    function setExitHandler(fn) {
        exitHandler = fn;
    }

    function opOsExit(code = 0) {
        if (!window[SymbolFor("isUnloadDispatched")]) {
            // Invokes the `unload` hooks before exiting
            // ref: https://github.com/denoland/deno/issues/3603
            window.dispatchEvent(new Event("unload"));
        }

        if (typeof exitHandler === "function") {
            exitHandler(code);
            return;
        }

        core.opSync("op_os_exit", code);
        throw new Error("Code not reachable");
    }

    if (typeof window.__bootstrap.os === "undefined") {
        window.__bootstrap.os = {}
    }

    window.__bootstrap.os.ops = {
        opOsPhysicalCoreCount,
        opOsTargetArch,
        opOsTargetEnv,
        opOsTargetOs,
        opOsHostname,
        opOsVersion,
        opOsLongVersion,
        opOsKernelVersion,

        opOsEnvGet,
        opOsEnvGetKeys,
        opOsEnvGetValues,
        opOsEnvGetKv,
        opOsEnvSet,
        opOsEnvHas,

        opOsCurrentPid,
        opOsCurrentPpid,

        opOsLoadavg,
        opOsGlobalProcessorInfo,
        opOsProcessorsInfo,

        opOsTotalMemoryKB,
        opOsFreeMemoryKB,
        opOsAvailableMemoryKB,
        opOsUsedMemoryKB,

        opOsExit,
    }

    // debug
    // Deno.os = window.__bootstrap.os;

})(this);