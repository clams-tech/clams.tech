<script lang="ts">
	import { scrollRef } from 'svelte-scrolling';

	export let title;
	export let scrollTo = 'none';
</script>

<div id={scrollTo} use:scrollRef={scrollTo} class="overflow-hidden">
	<div class="mx-auto max-w-5xl px-6 py-16 md:py-28 lg:px-8">
		<h2
			class="mb-8 text-center text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl md:mb-12 dark:text-white"
		>
			{title}
		</h2>
		<slot />
	</div>
</div>
