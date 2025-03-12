<script lang="ts">
	import { browser } from '$app/environment';
	import { page } from '$app/state';
	import Navigation from '$lib/components/shared/Navigation.svelte';
	import Footer from '$lib/components/shared/Footer.svelte';
	import { loadTranslations, locale } from '$lib/i18n/translations';
	import '../app.css';

	if (browser) {
		initialise();
	}

	async function initialise() {
		const defaultLocale = 'en';
		const initLocale = locale.get() || defaultLocale;
		await loadTranslations(initLocale);
	}
</script>

<Navigation />
<slot />
<Footer isAppRoute={page.url.pathname.includes('app')} />
