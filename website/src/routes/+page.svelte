<script lang="ts">
	import Hero from '$lib/components/Hero.svelte';
	import WhyClams from '$lib/components/WhyClams.svelte';
	import Features from '$lib/components/Features.svelte';
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
</div>
