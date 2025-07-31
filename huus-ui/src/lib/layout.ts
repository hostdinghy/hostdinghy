import type { Request, Router } from 'chuchi';

export function withLayout(
	loadLayoutComp: (req: Request) => Promise<any>,
	loadComp: (req: Request) => Promise<any>,
): (req: Request) => Promise<any> {
	return async (req: Request) => {
		const comp = await loadComp(req);
		const layoutComp = await loadLayoutComp(req);

		return {
			async loadProps(...args: any[]) {
				const [layoutProps, pageProps] = await Promise.all([
					layoutComp?.loadProps?.(...args) ?? {},
					comp?.loadProps?.(...args) ?? {},
				]);

				return { ...layoutProps, ...pageProps };
			},
			layout(...args: any[]) {
				return layoutComp.default(...args);
			},
			default(...args: any[]) {
				return comp.default(...args);
			},
		};
	};
}

export function layoutGroup(
	router: Router,
	path: string | RegExp,
	loadLayoutComp: (req: Request) => Promise<any>,
	loadGroup: (groupRouter: Pick<Router, 'register'>) => void,
) {
	const groupRouter = {
		register(
			subPath: string | RegExp,
			loadComp: (req: Request) => Promise<any>,
		) {
			return router.register(
				joinRoutePaths(path, subPath),
				withLayout(loadLayoutComp, loadComp),
			);
		},
	};

	loadGroup(groupRouter);
}

function joinRoutePaths(a: RegExp | string, b: RegExp | string) {
	if (typeof a === 'string' && typeof b === 'string') {
		return a + b;
	}

	const sourceA = a instanceof RegExp ? a.source.replace(/\$$/, '') : a;
	const flagsA = a instanceof RegExp ? a.flags : '';

	const sourceB = b instanceof RegExp ? b.source : b;
	const flagsB = b instanceof RegExp ? b.flags : '';

	const combinedSource = sourceA + sourceB;

	const combinedFlags = Array.from(new Set((flagsA + flagsB).split(''))).join(
		'',
	);

	return new RegExp(combinedSource, combinedFlags);
}
