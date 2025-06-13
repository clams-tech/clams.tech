<script lang="ts">
	import Hero from '$lib/components/Hero.svelte';
	import WhyClams from '$lib/components/WhyClams.svelte';
	import Features from '$lib/components/Features.svelte';
	import MailingListSection from '$lib/components/MailingListSection.svelte';
</script>

<svelte:head>
	<title>Clams</title>
</svelte:head>

<div>
	<Hero />
	<div class="mx-4 border-b"></div>
	<WhyClams />
	<div class="mx-4 border-b"></div>
	<Features />
	<div class="mx-4 border-b"></div>
	<MailingListSection />
</div>
