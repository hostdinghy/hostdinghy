import { Router as ChuchiRouter, Route as ChuchiRoute } from 'chuchi';
import type { Component as SvelteComponent } from 'svelte';
import { type LoadPropsFn } from './LoadProps';

// todo maybe we could make better types here?
// more generics?

export type Component = SvelteComponent<any, any, any>;
export type ComponentModule = {
	layout?: Component;
	default: Component;
	requiresRights?: string | string[];
	loadProps?: LoadPropsFn<any, any>;
};

export type Router = ChuchiRouter<ComponentModule>;

export type Route = ChuchiRoute<ComponentModule>;
