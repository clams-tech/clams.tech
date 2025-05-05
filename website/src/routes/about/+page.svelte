<script lang="ts">
	import MailingListForm from '$lib/components/shared/MailingListForm.svelte';
</script>

<!-- About -->
<section class="px-4 pb-16 pt-28">
	<div class="m-auto flex max-w-5xl flex-col gap-8">
		<h1 class="text-6xl">About Clams</h1>
		<div class="flex flex-col gap-2">
			<p>Hey!</p>
			<p>
				We're Aaron and John. Thanks to Bitcoin, we're not just excited about the future – we're on
				a mission to shape it. Inspired by the optimism and talent found in this community, we
				decided to use our skills to build Clams. Why stop at having the best money? We also need
				the best tools to help Bitcoiners manage their finances in a self-sovereign way. We are
				committed to keeping the app accessible to as many people as possible, which is why our core
				features will always be free. To ensure the long-term sustainability and continuous
				improvement of Clams, we're exploring options for a paid version. More details on that soon.
				We are our first customers, and we hope you will join us.
			</p>
			<p>Cheers,</p>
			<p>Aaron & John</p>
		</div>
		<div>
			<p class="text-2xl">Our Vision</p>
			<p>Set the global standard for Bitcoin accounting.</p>
		</div>
		<div>
			<p class="text-2xl">Our Mission</p>
			<p>
				Simplify Bitcoin accounting with trusted, private, best-in-class software for individuals,
				businesses, and finance professionals.
			</p>
		</div>

		<div class="grid grid-cols-2 gap-8">
			<div>
				<p class="text-2xl">Stay Updated</p>
				<p>
					Join our mailing list for the latest Clams updates. As with everything we do, your privacy
					is our priority— there will be no data sharing. Easy unsubscribe at any time.
				</p>
			</div>
			<MailingListForm />
		</div>
	</div>
</section>
