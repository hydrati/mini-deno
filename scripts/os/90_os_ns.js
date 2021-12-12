((window) => {
    const {
        Symbol,
        SymbolToStringTag,
        TypeError,
        ObjectSetPrototypeOf
    } = window.__bootstrap.primordials;

    const illegalConstructorKey = Symbol("illegalConstructorKey");
    const os = window.__bootstrap.os;
    const ns = {
        env: os.environ.environ,
        physicalCoreCount: os.ops.opOsPhysicalCoreCount,
        arch: os.ops.opOsTargetArch,
        platform: os.ops.opOsTargetOs,
        hostname: os.ops.opOsHostname,
        version: os.ops.opOsVersion,
        longVersion: os.ops.opOsLongVersion,
        kernelVersion: os.ops.opOsKernelVersion,
        pid: os.ops.opOsCurrentPid,
        ppid: os.ops.opOsCurrentPpid,
        loadavg: os.ops.opOsLoadavg,
        cpu: os.ops.opOsGlobalProcessorInfo,
        cpus: os.ops.opOsProcessorsInfo,
        totalMemory: os.ops.opOsTotalMemoryKB,
        freeMemory: os.ops.opOsFreeMemoryKB,
        availableMemory: os.ops.opOsAvailableMemoryKB,
        usedMemory: os.ops.opOsUsedMemoryKB,
        exit: os.ops.opOsExit,
    };

    ObjectSetPrototypeOf(ns, null);

    window.__bootstrap.denoNs_os = ns;

})(this);