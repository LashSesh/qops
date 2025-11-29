

export const index = 10;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/topology/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/10.D0IXE3Eo.js","_app/immutable/chunks/t7hwNuoe.js","_app/immutable/chunks/knXiOFn4.js","_app/immutable/chunks/D6YF6ztN.js","_app/immutable/chunks/DlQNAQKj.js"];
export const stylesheets = [];
export const fonts = [];
