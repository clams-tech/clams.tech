<script lang="ts">
	import ContentContainer from '$lib/components/shared/ContentContainer.svelte';
</script>

<ContentContainer title="Hmmm... This page doesn’t exist!">
	<div class="h-screen flex-col items-center text-center">
		<p class="text-xl tracking-tight text-gray-900 dark:text-white">
			Maybe the link is broken, or the page moved.
		</p>
		<div class="mt-8">
			<a
				href="/"
				class="text-md rounded-md bg-indigo-600 px-3.5 py-2.5 font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
				>Go Home</a
			>
		</div>
	</div>
</ContentContainer>
