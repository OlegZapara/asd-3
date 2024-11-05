<script lang="ts">
	import { TabGroup, Tab } from '@skeletonlabs/skeleton';
	import AddForm from './AddForm.svelte';
	import UpdateForm from './UpdateForm.svelte';
	import DeleteForm from './DeleteForm.svelte';
	import SearchForm from './SearchForm.svelte';
	import GenForm from './GenForm.svelte';

	let tabSet: number = 0;
</script>

<TabGroup justify="justify-center">
	<Tab bind:group={tabSet} name="tab1" value={0}>Add row</Tab>
	<Tab bind:group={tabSet} name="tab2" value={1}>Edit row</Tab>
	<Tab bind:group={tabSet} name="tab3" value={2}>Delete row</Tab>
	<Tab bind:group={tabSet} name="tab4" value={3}>Search</Tab>
	<Tab bind:group={tabSet} name="tab5" value={4}>Generate</Tab>
	<svelte:fragment slot="panel">
		{#if tabSet === 0}
			<AddForm />
		{:else if tabSet === 1}
			<UpdateForm />
		{:else if tabSet === 2}
			<DeleteForm />
		{:else if tabSet === 3}
			<SearchForm />
		{:else if tabSet === 4}
			<GenForm />
		{/if}
	</svelte:fragment>
</TabGroup>
