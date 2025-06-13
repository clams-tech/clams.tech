<script lang="ts">
	import MailingListForm from '$lib/components/shared/MailingListForm.svelte';
</script>

<section class="px-4 py-16">
	<div class="m-auto flex max-w-5xl flex-col gap-8">
		<div class="flex flex-col gap-4">
			<h1 class="text-center text-6xl">Never miss an update</h1>
			<p class="text-center">Join our mailing list to get the latest updates delivered to your inbox.</p>
		</div>
		<div class="mx-auto w-full max-w-lg">
			<MailingListForm />
		</div>
	</div>
</section>