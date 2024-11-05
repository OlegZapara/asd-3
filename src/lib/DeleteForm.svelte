<script lang="ts">
	import { Trash, RotateCcw, Shuffle } from 'lucide-svelte';
	import { deleteUser } from '../stores/UsersStore';
	import { invoke } from '@tauri-apps/api';
	import type { User } from '../types/User';
	let id: string = '';

	async function deleteRecord() {
		try {
			let idNumber: number = parseInt(id);
			await deleteUser(idNumber);
		} catch (error) {
			console.error('Failed to delete record:', error);
		}
	}

	function clearForm() {
		id = '';
	}

	async function deleteRandomId() {
		try {
			let users: User[] = await invoke('get_users');
			let randomIndex: number = Math.floor(Math.random() * users.length);
			id = users[randomIndex].id.toString();
			await deleteRecord();
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
		<button on:click={deleteRandomId} type="button">
			<span class="text-blue-600"><Shuffle /></span>
			<span class="text-blue-600">Delete random</span>
		</button>
		<button
			on:click={deleteRecord}
			type="submit"
			class="flex flex-row justify-center items-center flex-grow"
		>
			<span class="text-red-600"><Trash /></span>
			<span class="text-red-600">Delete record</span>
		</button>
	</div>
</form>
