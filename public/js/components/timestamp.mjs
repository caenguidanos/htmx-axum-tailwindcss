import {setTimeout$} from "../utils/timers.mjs"

/**
 * @param {HTMLElement} root
 * @return {void}
 */
export function onMount(root) {
    setTimeout$("timestamp", fadeText(root), 250);
}

/**
 * @param {HTMLElement} root
 * @return {() => void}
 */
function fadeText(root) {
    return () => root
        .querySelector("#timestamp span")
        .classList.replace("text-green-600", "text-slate-600");
}