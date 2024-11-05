<script lang="ts">
	import { Pencil, RotateCcw, Shuffle } from 'lucide-svelte';
	import { updateUser } from '../stores/UsersStore';
	import type { User } from '../types/User';
	import { invoke } from '@tauri-apps/api';
	import { faker } from '@faker-js/faker';

	let id: string = '';
	let email: string = '';
	let username: string = '';
	let password: string = '';

	async function updateRecord() {
		try {
			let idNumber: number = parseInt(id);
			let record = { id: idNumber, email, username, password };
			await updateUser(record);
		} catch (error) {
			console.error('Failed to update record:', error);
		}
	}

	async function editRandom() {
		try {
			let users: User[] = await invoke('get_users');
			let randomIndex: number = Math.floor(Math.random() * users.length);
			id = users[randomIndex].id.toString();
			email = faker.internet.email();
			username = faker.internet.username();
			password = faker.internet.password();
			await updateRecord();
		} catch (error) {
			console.error('Failed to get random ID:', error);
		}
	}

	function clearForm() {
		id = '';
		email = '';
		username = '';
		password = '';
	}
</script>

<form class="w-full flex flex-col justify-center items-center gap-8 p-4">
	<label class="label w-full">
		<span>ID</span>
		<input bind:value={id} class="input" type="text" placeholder="Record ID" />
	</label>
	<label class="label w-full">
		<span>Email</span>
		<input bind:value={email} class="input" type="email" placeholder="Email" />
	</label>

	<label class="label w-full">
		<span>Username</span>
		<input bind:value={username} class="input" type="text" placeholder="Email" />
	</label>

	<label class="label w-full">
		<span>Password</span>
		<input bind:value={password} class="input" type="text" placeholder="Password" />
	</label>

	<div class="btn-group variant-filled w-full mt-10">
		<button on:click={clearForm} type="button">
			<span class="text-yellow-600"><RotateCcw /></span>
			<span class="text-yellow-600">Reset</span>
		</button>
		<button on:click={editRandom} type="button">
			<span class="text-blue-600"><Shuffle /></span>
			<span class="text-blue-600">Edit random</span>
		</button>
		<button
			on:click={updateRecord}
			type="submit"
			class="flex flex-row justify-center items-center flex-grow"
		>
			<span class="text-green-600"><Pencil /></span>
			<span class="text-green-600">Update record</span>
		</button>
	</div>
</form>
