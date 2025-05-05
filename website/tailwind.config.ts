import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	darkMode: 'class',
	theme: {
		extend: {
			colors: {
				green: {
					light: '#edf0eb',
					'light-hover': '#e3e9e1',
					'light-active': '#c6d2c2',
					DEFAULT: '#466d39',
					hover: '#3f6233',
					active: '#38572e',
					dark: '#35522b',
					'dark-hover': '#2a4122',
					'dark-active': '#1f311a',
					darker: '#192614'
				},
				red: {
					light: '#f6eced',
					'light-hover': '#f2e3e3',
					'light-active': '#e4c5c6',
					DEFAULT: '#a84546',
					hover: '#973e3f',
					active: '#863738',
					dark: '#7e3435',
					'dark-hover': '#65292a',
					'dark-active': '#4c1f1f',
					darker: '#3b1819'
				},
				grey: {
					light: '#fefdfd',
					'light-hover': '#fdfcfb',
					'light-active': '#fbfaf8',
					DEFAULT: '#f1eee7',
					hover: '#d9d6d0',
					active: '#c1beb9',
					dark: '#b5b3ad',
					'dark-hover': '#918f8b',
					'dark-active': '#6c6b68',
					darker: '#545351'
				},
				// Semantic color mappings
				primary: {
					bg: {
						DEFAULT: '#f1eee7', // grey.DEFAULT
						hover: '#d9d6d0', // grey.hover
						active: '#c1beb9' // grey.active
					},
					text: {
						DEFAULT: '#212420' // black
						// hover: '#3f6233', // green.hover
						// active: '#38572e' // green.active
					},
					border: '#212420'
				},
				error: {
					text: {
						DEFAULT: '#a84546', // red.DEFAULT
						hover: '#973e3f', // red.hover
						active: '#863738' // red.active
					}
				},
				black: '#212420',
				white: '#ffffff',
				transparent: 'transparent'
			},
			borderColor: {
				DEFAULT: '#212420' // Override default border color
			}
		}
	},
	plugins: [],
	variants: {
		width: ['responsive', 'hover', 'focus']
	}
} satisfies Config;
