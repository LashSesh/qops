
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	export interface AppTypes {
		RouteId(): "/" | "/algorithm" | "/circuit" | "/genesis" | "/hypercube" | "/resonance" | "/settings" | "/slots" | "/topology";
		RouteParams(): {
			
		};
		LayoutParams(): {
			"/": Record<string, never>;
			"/algorithm": Record<string, never>;
			"/circuit": Record<string, never>;
			"/genesis": Record<string, never>;
			"/hypercube": Record<string, never>;
			"/resonance": Record<string, never>;
			"/settings": Record<string, never>;
			"/slots": Record<string, never>;
			"/topology": Record<string, never>
		};
		Pathname(): "/" | "/algorithm" | "/algorithm/" | "/circuit" | "/circuit/" | "/genesis" | "/genesis/" | "/hypercube" | "/hypercube/" | "/resonance" | "/resonance/" | "/settings" | "/settings/" | "/slots" | "/slots/" | "/topology" | "/topology/";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): string & {};
	}
}