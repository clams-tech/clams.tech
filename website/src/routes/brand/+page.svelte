<script lang="ts">
	import clamsLogoSVG from '$lib/icons/clams-logo.svg';
	import clamsLogoPNG from '$lib/icons/clams-logo.png';
	import clamsLogomarkSVG from '$lib/icons/clams-logomark.svg';
	import clamsLogomarkPNG from '$lib/icons/clams-logomark.png';
	import clamsLogoWhiteSVG from '$lib/icons/clams-logo-white.svg';
</script>

<section class="px-4 py-16">
	<div class="mt-1 mb-16 border-b"></div>
	<div class="m-auto flex max-w-5xl flex-col gap-8">
		<h1 class="text-6xl">Brand Assets</h1>

		<!-- Downloads -->
		<div class="grid gap-8 md:grid-cols-2">
			<!-- Logo -->
			<div class="flex flex-col gap-4">
				<div>
					<p class="text-2xl font-bold">Logo</p>
					<p class="mt-2">Our primary brand logo.</p>
				</div>
				<div class="flex flex-col items-center rounded-md border bg-white p-6">
					<div class="flex h-24 w-full items-center justify-center">
						<img src={clamsLogoSVG} alt="Logo" class="h-24" />
					</div>
					<div class="mt-6 flex gap-4">
						<a
							href={clamsLogoSVG}
							download={clamsLogoSVG}
							class="flex items-center gap-2 rounded-md border px-4 py-2 text-sm transition-colors"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
								<polyline points="7 10 12 15 17 10"></polyline>
								<line x1="12" y1="15" x2="12" y2="3"></line>
							</svg>
							SVG
						</a>
						<a
							href={clamsLogoPNG}
							download={clamsLogoPNG}
							class="flex items-center gap-2 rounded-md border bg-white px-4 py-2 text-sm transition-colors"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
								<polyline points="7 10 12 15 17 10"></polyline>
								<line x1="12" y1="15" x2="12" y2="3"></line>
							</svg>
							PNG
						</a>
					</div>
				</div>
			</div>

			<!-- Logomark -->
			<div class="flex flex-col gap-4">
				<div>
					<p class="text-2xl font-bold">Logomark</p>
					<p class="mt-2">Our logo for compact spaces.</p>
				</div>
				<div class="flex flex-col items-center rounded-md border bg-white p-6">
					<div class="flex h-24 w-full items-center justify-center">
						<img src={clamsLogomarkSVG} alt="Clams Logomark" class="h-24" />
					</div>
					<div class="mt-6 flex gap-4">
						<a
							href={clamsLogomarkSVG}
							download={clamsLogomarkSVG}
							class="flex items-center gap-2 rounded-md border bg-white px-4 py-2 text-sm transition-colors"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
								<polyline points="7 10 12 15 17 10"></polyline>
								<line x1="12" y1="15" x2="12" y2="3"></line>
							</svg>
							SVG
						</a>
						<a
							href={clamsLogomarkPNG}
							download={clamsLogomarkPNG}
							class="flex items-center gap-2 rounded-md border px-4 py-2 text-sm transition-colors"
						>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="16"
								height="16"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							>
								<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
								<polyline points="7 10 12 15 17 10"></polyline>
								<line x1="12" y1="15" x2="12" y2="3"></line>
							</svg>
							PNG
						</a>
					</div>
				</div>
			</div>
		</div>

		<!-- Brand Colors -->
		<div>
			<p class="text-2xl font-bold">Brand Colors</p>
			<p class="mt-2">Our official color palette for consistent brand representation.</p>

			<div class="mt-6 grid gap-6 md:grid-cols-4">
				<!-- Black -->
				<div class="flex flex-col overflow-hidden rounded-md border">
					<div class="h-24 bg-[#212420]"></div>
					<div class="bg-white p-4">
						<p class="font-semibold">Off-Black</p>
						<p class="mt-1 text-sm">#212420</p>
					</div>
				</div>

				<!-- Off-White -->
				<div class="flex flex-col overflow-hidden rounded-md border">
					<div class="h-24 bg-[#F1EEE7]"></div>
					<div class="bg-white p-4">
						<p class="font-semibold">Off-White</p>
						<p class="mt-1 text-sm">#F1EEE7</p>
					</div>
				</div>

				<!-- Red -->
				<div class="flex flex-col overflow-hidden rounded-md border">
					<div class="h-24 bg-[#8F0709]"></div>
					<div class="bg-white p-4">
						<p class="font-semibold">Red</p>
						<p class="mt-1 text-sm">#8F0709</p>
					</div>
				</div>

				<!-- Green -->
				<div class="flex flex-col overflow-hidden rounded-md border">
					<div class="h-24 bg-[#235811]"></div>
					<div class="bg-white p-4">
						<p class="font-semibold">Green</p>
						<p class="mt-1 text-sm">#235811</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Logo Color Usage -->
		<div>
			<p class="text-2xl font-bold">Logo Color Usage</p>
			<p class="mt-2">
				Our logo should only ever be a single color: Black or Off-White. Always maintain high
				contrast with the background.
			</p>

			<div class="mt-6 grid gap-8 md:grid-cols-2">
				<!-- Light Background Example -->
				<div class="flex flex-col overflow-hidden rounded-md border">
					<div class="flex h-36 items-center justify-center bg-[#F1EEE7] p-4">
						<img src={clamsLogoSVG} alt="Logo on light background" class="h-16 filter" />
					</div>
					<div class="bg-white p-4">
						<p class="font-semibold">Light Background</p>
						<p class="mt-1 text-sm">Use Black logo on light backgrounds</p>
					</div>
				</div>

				<!-- Dark Background Example -->
				<div class="flex flex-col overflow-hidden rounded-md border">
					<div class="flex h-36 items-center justify-center bg-[#212420] p-4">
						<img src={clamsLogoWhiteSVG} alt="Logo on dark background" class="h-16" />
					</div>
					<div class="bg-white p-4">
						<p class="font-semibold">Dark Background</p>
						<p class="mt-1 text-sm">Use Off-White logo on dark backgrounds</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Usage Guidelines -->
		<div>
			<p class="text-2xl font-bold">Usage Guidelines</p>
			<div class="mt-4 grid gap-8 md:grid-cols-2">
				<div>
					<p class="font-semibold">Do:</p>
					<ul class="mt-2 list-disc pl-5">
						<li>Use the logo with adequate spacing</li>
						<li>Maintain aspect ratio when scaling</li>
						<li>Use on backgrounds that ensure visibility</li>
						<li>Always use either Black or Off-White for the logo</li>
					</ul>
				</div>
				<div>
					<p class="font-semibold">Don't:</p>
					<ul class="mt-2 list-disc pl-5">
						<li>Make size smaller than than 31px</li>
						<li>Distort or modify the logo</li>
						<li>Change the colors beyond Black and Off-White</li>
						<li>Apply effects like shadows or outlines</li>
					</ul>
				</div>
			</div>
		</div>
	</div>
</section>
