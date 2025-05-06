<!-- Button.svelte -->
<script lang="ts">
	export let variant: 'black' | 'green' | 'red' = 'black';
	export let disabled = false;
	export let type: 'button' | 'submit' | 'reset' = 'button';
	export let href: string = '';
	export let target: string = '';
	export let rel: string = '';

	const classes =
		'inline-flex items-center justify-center rounded-xl px-4 py-3 transition-colors duration-200 disabled:cursor-not-allowed disabled:opacity-50 ' +
		(variant === 'black'
			? 'bg-black text-white hover:bg-black/90 active:bg-black/80'
			: variant === 'green'
				? 'bg-clams-green-normal hover:clams-green-normal-hover active:clams-green-normal-active text-white'
				: 'bg-clams-red-norma hover:bg-clams-red-normal-hover active:clams-red-normal-active text-white');
</script>

{#if href}
	<a {href} class={classes} aria-disabled={disabled} {target} {rel}>
		<p class="leading-none"><slot /></p>
	</a>
{:else}
	<button {type} {disabled} class={classes}>
		<p class="leading-none"><slot /></p>
	</button>
{/if}
