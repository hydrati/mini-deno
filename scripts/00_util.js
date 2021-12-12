// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.
"use strict";

((window) => {
    function writable(value) {
        return {
            value,
            writable: true,
            enumerable: true,
            configurable: true,
        };
    }

    function value(value) {
        return {
            value,
        };
    }

    function nonEnumerable(value) {
        return {
            value,
            writable: true,
            enumerable: false,
            configurable: true,
        };
    }

    function readOnly(value) {
        return {
            value,
            enumerable: true,
            writable: false,
            configurable: true,
        };
    }

    function getterOnly(getter) {
        return {
            get: getter,
            enumerable: true,
            configurable: true,
        };
    }

    function onceFunc(fn, thisArg) {
        let called = false;
        return function() {
            if (called) return null;
            return fn.apply(thisArg, arguments);
        }
    }

    window.__bootstrap.util = {
        writable,
        nonEnumerable,
        readOnly,
        getterOnly,
        value,
        onceFunc,
    };
})(this);