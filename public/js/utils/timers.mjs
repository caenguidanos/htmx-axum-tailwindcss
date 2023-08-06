/**
 * @param {string} key Unique key
 * @param {() => void} cb Callback
 * @param {number} ms Milliseconds
 */
export function setTimeout$(key, cb, ms) {
    globalThis.__TIMERS_ID ??= {};

    if (key in globalThis.__TIMERS_ID) {
        globalThis.clearTimeout(globalThis.__TIMERS_ID[key]);
    }

    globalThis.__TIMERS_ID[key] = globalThis.setTimeout(cb, ms);
}
