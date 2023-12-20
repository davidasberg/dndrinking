/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,rs}', 'index.html'],

	theme: {
		extend: {},
	},
	plugins: [
		// require('@tailwindcss/typography'),
		// require('@tailwindcss/forms'),
		require("daisyui")
	],
	daisyui: {
		themes: [
			{
				mytheme: {

					"primary": "#facc15",

					"secondary": "#f43f5e",

					"accent": "#0064ff",

					"neutral": "#0f0f0f",

					"base-100": "#222222",

					"info": "#00adcb",

					"success": "#548100",

					"warning": "#ea7e00",

					"error": "#e6364a",
				},
			},
		],
	},
};
