import type { Request } from 'chuchi';
import type { ComponentModule, Router } from '.';

export function withLayout(
	loadLayoutComp: (req: Request) => Promise<ComponentModule>,
	loadComp: (req: Request) => Promise<ComponentModule>,
): (req: Request) => Promise<ComponentModule> {
	return async req => {
		const [layoutComp, comp] = await Promise.all([
			loadLayoutComp(req),
			loadComp(req),
		]);

		return {
			async loadProps(pageProps, lp) {
				// we cannot load the components in parallel because the component
				// might depend on the props from the layout

				const layoutProps =
					(await layoutComp.loadProps?.(pageProps, lp)) ?? {};

				// merge new Props
				pageProps = { ...pageProps, ...layoutProps };
				const nProps = (await comp?.loadProps?.(pageProps, lp)) ?? {};

				return {
					...pageProps,
					...nProps,
				};
			},
			layout: layoutComp.default,
			default: comp.default,
		};
	};
}

export function layoutGroup(
	router: Router,
	path: string | RegExp,
	loadLayoutComp: (req: Request) => Promise<ComponentModule>,
	loadGroup: (groupRouter: Pick<Router, 'register'>) => void,
) {
	loadGroup({
		register(subPath, loadComp) {
			return router.register(
				joinRoutePaths(path, subPath),
				withLayout(loadLayoutComp, loadComp),
			);
		},
	});
}

function joinRoutePaths(a: string, b: string): string;
function joinRoutePaths(a: RegExp | string, b: RegExp | string): RegExp;
function joinRoutePaths(
	a: RegExp | string,
	b: RegExp | string,
): string | RegExp {
	if (typeof a === 'string' && typeof b === 'string') {
		return a + b;
	}

	const sourceA = a instanceof RegExp ? a.source.replace(/\$$/, '') : a;
	const flagsA = a instanceof RegExp ? a.flags : '';

	const sourceB = b instanceof RegExp ? b.source : b;
	const flagsB = b instanceof RegExp ? b.flags : '';

	const combinedSource = sourceA + sourceB;

	console.log('cmobind', combinedSource);

	const combinedFlags = Array.from(new Set((flagsA + flagsB).split(''))).join(
		'',
	);

	return new RegExp(combinedSource, combinedFlags);
}
