<script lang="ts">
	import Controls from '$lib/Controls.svelte';
	import Data from '$lib/Data.svelte';
</script>

<div class="h-full mx-auto grid grid-cols-8 gap-4">
	<div class="col-span-3">
		<Controls />
	</div>
	<div class="col-span-5">
		<Data />
	</div>
</div>
