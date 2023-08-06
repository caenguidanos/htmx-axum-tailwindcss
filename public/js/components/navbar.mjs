/**
 * @param {HTMLElement} root
 * @return {void}
 */
export function onMount(root) {
    root.querySelectorAll("a").forEach(disableNavbarLink);
}

/**
 * @param {HTMLAnchorElement} node
 * @return {void}
 */
function disableNavbarLink(node) {
    if (globalThis.location.href === node.href) {
        node.style.pointerEvents = "none";
        node.style.cursor = "not-allowed";

        node.classList.remove("hover:bg-blue-200");
        node.classList.remove("hover:text-blue-700");
        node.classList.remove("active:bg-blue-300");

        node.classList.add("text-slate-600");
    }
}