<script lang="ts">
	import type { PaginationSettings } from '@skeletonlabs/skeleton';
	import { Paginator } from '@skeletonlabs/skeleton';
	import { Table } from 'lucide-svelte';
	import { onDestroy, onMount } from 'svelte';
	import { fetchUsers, userStore } from '../stores/UsersStore';
	import type { User } from '../types/User';

	let users: User[] = [];
	let paginationSettings = {
		page: 0,
		limit: 10,
		size: users.length,
		amounts: []
	} satisfies PaginationSettings;

	const unsubscribe = userStore.subscribe((value) => {
		users = value;
		paginationSettings = {
			page: 0,
			limit: 10,
			size: users.length,
			amounts: []
		} satisfies PaginationSettings;
	});

	onMount(async () => {
		await fetchUsers();
	});

	onDestroy(() => {
		unsubscribe();
	});

	let search = '';

	$: filteredResults = users.filter((row) => {
		if (search === '') return true;
		if (!search.includes(':')) return true;
		let searchKey = search.split(':')[0];
		let searchValue = search.split(':')[1].trim();
		if (searchValue === '') return true;
		if (
			Object.keys(row)
				.map((x) => x.toLowerCase())
				.includes(searchKey.toLowerCase())
		) {
			return (row as any)[searchKey].toString().toLowerCase().includes(searchValue.toLowerCase());
		}
		return false;
	});

	$: paginatedSource = filteredResults
		.sort((a, b) => a.id - b.id)
		.slice(
			paginationSettings.page * paginationSettings.limit,
			paginationSettings.page * paginationSettings.limit + paginationSettings.limit
		);
</script>

<div class="table-container flex flex-col justify-center items-end gap-4">
	<div class="w-full px-10 text-xl font-bold flex flex-row gap-8 items-center justify-between">
		<span class="flex flex-row gap-2 flex-nowrap"><Table />Users table</span>
		<input bind:value={search} class="input w-auto flex-grow" placeholder="Search" />
		<span class="flex flex-row gap-2 flex-nowrap">
			Page {paginationSettings.page + 1} of {Math.ceil(users.length / paginationSettings.limit)}
		</span>
	</div>
	<table class="table table-hover">
		<thead>
			<tr>
				<th>ID</th>
				<th>Email</th>
				<th>Username</th>
				<th>Password</th>
			</tr>
		</thead>
		<tbody>
			{#each paginatedSource as row, i}
				<tr>
					<td>{row.id}</td>
					<td>{row.email}</td>
					<td>{row.username}</td>
					<td>{row.password}</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<Paginator
		bind:settings={paginationSettings}
		showFirstLastButtons={false}
		showPreviousNextButtons={true}
	/>
</div>
