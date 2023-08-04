/**
 * @param {string} key Unique key
 * @param {() => void} cb Callback
 * @param {number} ms Milliseconds
 */
export function setTimeout$(key, cb, ms) {
	globalThis.__SSR_TIMERS ??= {};
	if (key in globalThis.__SSR_TIMERS) {
		globalThis.clearTimeout(globalThis.__SSR_TIMERS[key]);
	}
	globalThis.__SSR_TIMERS[key] = globalThis.setTimeout(cb, ms);
}
