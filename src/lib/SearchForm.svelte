<script lang="ts">
	import { Search, RotateCcw, Shuffle } from 'lucide-svelte';
	import { searchUser } from '../stores/UsersStore';
	import { invoke } from '@tauri-apps/api';
	import type { User } from '../types/User';
	let id: string = '';

	let activeUser: User | null = null;
	let comparisons = 0;

	async function searchRecord() {
		try {
			let idNumber: number = parseInt(id);
			const result = await searchUser(idNumber);
			activeUser = result.user;
			comparisons = result.comparisons;
		} catch (error) {
			console.error('Failed to search record:', error);
		}
	}

	function clearForm() {
		id = '';
		activeUser = null;
	}

	async function searchRandomId() {
		try {
			let users: User[] = await invoke('get_users');
			let randomIndex: number = Math.floor(Math.random() * users.length);
			id = users[randomIndex].id.toString();
			await searchRecord();
		} catch (error) {
			console.error('Failed to get random ID:', error);
		}
	}
</script>

<form class="w-full flex flex-col justify-center items-center gap-8 p-4">
	<label class="label w-full">
		<span>ID</span>
		<input bind:value={id} class="input" type="text" placeholder="Record ID" />
	</label>

	<div class="btn-group variant-filled w-full mt-10">
		<button on:click={clearForm} type="button">
			<span class="text-yellow-600"><RotateCcw /></span>
			<span class="text-yellow-600">Reset</span>
		</button>
		<button on:click={searchRandomId} type="button">
			<span class="text-blue-600"><Shuffle /></span>
			<span class="text-blue-600">Search random</span>
		</button>
		<button
			on:click={searchRecord}
			type="submit"
			class="flex flex-row justify-center items-center flex-grow"
		>
			<span class="text-green-600"><Search /></span>
			<span class="text-green-600">Search record</span>
		</button>
	</div>
</form>

{#if activeUser != null}
	<aside class="alert variant-ghost mt-4">
		<div class="alert-message">
			<h3 class="h3">ID {activeUser.id}: {activeUser.username}</h3>
			<div>Email: {activeUser.email}</div>
			<div>Password: {activeUser.password}</div>
			<div>Comparisons: {comparisons}</div>
		</div>
	</aside>
{/if}
