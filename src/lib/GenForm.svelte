<script lang="ts">
	import { faker } from '@faker-js/faker';
	import { invoke } from '@tauri-apps/api';
	import { RotateCcw, WandSparkles } from 'lucide-svelte';
	import { fetchUsers } from '../stores/UsersStore';
	import type { User } from '../types/User';

	let amount: string = '';
	let users: User[] = [];

	function clearForm() {
		amount = '';
		users = [];
	}

	async function generateFakeData() {
		await invoke('clear_users');
		users = [];
		for (let i = 0; i < parseInt(amount); i++) {
			users.push({
				id: i,
				email: faker.internet.email(),
				username: faker.internet.username(),
				password: faker.internet.password()
			});
		}
		await invoke('add_users', { users: users });
		await fetchUsers();
	}
</script>

<form class="w-full flex flex-col justify-center items-center gap-8 p-4">
	<label class="label w-full">
		<span>Number of records</span>
		<input
			bind:value={amount}
			class="input"
			type="text"
			placeholder="Number of generated records"
		/>
	</label>

	<div class="btn-group variant-filled w-full mt-10">
		<button on:click={clearForm} type="button">
			<span class="text-yellow-600"><RotateCcw /></span>
			<span class="text-yellow-600">Reset</span>
		</button>

		<button
			on:click={generateFakeData}
			type="submit"
			class="flex flex-row justify-center items-center flex-grow"
		>
			<span class="text-green-600"><WandSparkles /></span>
			<span class="text-green-600">Generate records</span>
		</button>
	</div>
</form>
