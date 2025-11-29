

export const index = 5;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/genesis/_page.svelte.js')).default;
export const imports = ["_app/immutable/nodes/5.DlZTQqxY.js","_app/immutable/chunks/t7hwNuoe.js","_app/immutable/chunks/knXiOFn4.js","_app/immutable/chunks/D6YF6ztN.js","_app/immutable/chunks/DlQNAQKj.js"];
export const stylesheets = ["_app/immutable/assets/5.BqzMmYRY.css"];
export const fonts = [];
