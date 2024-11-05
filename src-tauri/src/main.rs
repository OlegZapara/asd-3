// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use once_cell::sync::Lazy;
use std::sync::RwLock;

use app::{BTree, User};
use tauri::command;

static TREE: Lazy<RwLock<BTree>> = Lazy::new(|| RwLock::new(BTree::new()));
static COUNTER: Lazy<RwLock<u32>> = Lazy::new(|| RwLock::new(0));

#[command]
fn get_users() -> Vec<User> {
    let tree = TREE.read().unwrap();
    tree.get_all_users()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SearchResult {
    user: User,
    comparisons: u32,
}

#[command]
fn search_user(id: u32) -> Option<SearchResult> {
    let mut tree = TREE.write().unwrap();
    let (user, comparisons) = tree.search(id);
    if let Some(user) = user {
        return Some(SearchResult {
            user: user.clone(),
            comparisons,
        });
    }
    None
}

#[command]
fn add_user(user: User) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
    let mut tree = TREE.write().unwrap();
    tree.insert(user);
}

#[command]
fn add_users(users: Vec<User>) {
    let mut counter = COUNTER.write().unwrap();
    let mut tree = TREE.write().unwrap();
    for user in users {
        *counter += 1;
        tree.insert(user);
    }
}

#[command]
fn remove_user(id: u32) {
    let mut tree = TREE.write().unwrap();
    tree.delete(id);
}

#[command]
fn update_user(user: User) {
    let mut tree = TREE.write().unwrap();
    tree.delete(user.id);
    tree.insert(user);
}

#[command]
fn get_next_id() -> u32 {
    let counter = COUNTER.read().unwrap();
    *counter + 1
}

#[command]
fn clear_users() {
    let mut tree = TREE.write().unwrap();
    *tree = BTree::new();
    let mut counter = COUNTER.write().unwrap();
    *counter = 0;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_users,
            search_user,
            add_user,
            add_users,
            remove_user,
            update_user,
            get_next_id,
            clear_users
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
