<script lang="ts">
	import '../app.postcss';
	import { AppBar } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';
</script>

<AppBar>
	<svelte:fragment slot="lead"><img src="favicon.png" alt="" /></svelte:fragment>
	<p class="text-2xl font-bold">Лабораторна робота №3</p>
	<svelte:fragment slot="trail"><LightSwitch /></svelte:fragment>
</AppBar>
<main class="py-6 px-2">
	<slot />
</main>
