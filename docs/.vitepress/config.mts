import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
	title: "🛶 HostDinghy",
	description: "Self-hosting",
	themeConfig: {
		// https://vitepress.dev/reference/default-theme-config
		nav: [
			{ text: "Home", link: "/" },
			{ text: "Compose", link: "/compose" },
		],

		sidebar: [
			{
				text: "Compose",
				items: [
					{ text: "Compose", link: "/compose" },
					// { text: "Runtime API Examples", link: "/api-examples" },
				],
			},
		],

		socialLinks: [
			{
				icon: "github",
				link: "https://github.com/hostdinghy/hostdinghy",
			},
		],
	},
});
