<script lang="ts">
	import { Plus, RotateCcw, Shuffle } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { addUser } from '../stores/UsersStore';
	import { faker } from '@faker-js/faker';

	let email: string = '';
	let username: string = '';
	let password: string = '';

	async function addRecord() {
		try {
			let id: number = await invoke('get_next_id');
			let record = { id, email, username, password };
			await addUser(record);
		} catch (error) {
			console.error('Failed to add record:', error);
		}
	}

	async function addFakeData() {
		email = faker.internet.email();
		username = faker.internet.username();
		password = faker.internet.password();
		await addRecord();
	}

	function clearForm() {
		email = '';
		username = '';
		password = '';
	}
</script>

<form class="w-full flex flex-col justify-center items-center gap-8 p-4">
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
		<button on:click={addFakeData} type="button">
			<span class="text-blue-600"><Shuffle /></span>
			<span class="text-blue-600">Add random</span>
		</button>
		<button
			on:click={addRecord}
			type="submit"
			class="flex flex-row justify-center items-center flex-grow"
		>
			<span class="text-green-600"><Plus /></span>
			<span class="text-green-600">Add record</span>
		</button>
	</div>
</form>
