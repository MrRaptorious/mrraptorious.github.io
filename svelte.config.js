import adapter from '@sveltejs/adapter-static';

const dev = process.argv.includes('dev');

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			pages: 'build/',
			assets: 'build/',   
			fallback: undefined,
		}),
		paths: {
			base: dev ? '' : '/mrraptorious.github.io',
		},
		prerender: {
			entries: dev ? [] : ['*']
		}
	}
};

export default config;
