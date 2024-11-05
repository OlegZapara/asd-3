import { writable } from 'svelte/store';
import type { User } from '../types/User';
import { invoke } from '@tauri-apps/api';

export const userStore = writable<User[]>([]);

export async function fetchUsers() {
	const users: User[] = await invoke('get_users');
	userStore.set(users);
}

type SearchResult = {
	user: User;
	comparisons: number;
};
export async function searchUser(id: number) {
	const result: SearchResult = await invoke('search_user', { id: id });
	return result;
}

export async function addUser(user: User) {
	await invoke('add_user', { user: user });
	fetchUsers();
}

export async function deleteUser(id: number) {
	await invoke('remove_user', { id: id });
	fetchUsers();
}

export async function updateUser(user: User) {
	await invoke('update_user', { user: user });
	fetchUsers();
}
