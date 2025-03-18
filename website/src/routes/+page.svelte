<script lang="ts">
	import Hero from '$lib/components/app/Hero.svelte';
	import HowItWorks from '$lib/components/app/HowItWorks.svelte';
	import Features from '$lib/components/app/Features.svelte';
	import Connections from '$lib/components/app/Connections.svelte';
	import Pricing from '$lib/components/app/Pricing.svelte';
	import FAQ from '$lib/components/app/FAQ.svelte';
	import MailingList from '$lib/components/app/MailingList.svelte';
</script>

<svelte:head>
	<title>Clams</title>
</svelte:head>

<Hero />
<Features />
<HowItWorks />
<Pricing />
<Connections />
<FAQ />
<MailingList />
