const T: usize = 100;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct BTreeNode {
    pub keys: Vec<u32>,
    pub values: Vec<User>,
    pub children: Vec<BTreeNode>,
    pub leaf: bool,
}

impl BTreeNode {
    pub fn new(leaf: bool) -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            leaf,
        }
    }

    // Search for a user by ID and count comparisons
    pub fn search(&mut self, id: u32, comparisons: &mut u32) -> Option<&mut User> {
        let mut i = 0;
        while i < self.keys.len() && id > self.keys[i] {
            *comparisons += 1;
            i += 1;
        }
        *comparisons += 1; // For the final comparison in the while loop
        if i < self.keys.len() && self.keys[i] == id {
            Some(&mut self.values[i])
        } else if self.leaf {
            None
        } else {
            self.children[i].search(id, comparisons)
        }
    }

    pub fn split_child(&mut self, i: usize) {
        let degree = T;
        let mut new_node = BTreeNode::new(self.children[i].leaf);

        new_node.keys = self.children[i].keys.split_off(degree - 1);
        new_node.values = self.children[i].values.split_off(degree - 1);

        if !self.children[i].leaf {
            new_node.children = self.children[i].children.split_off(degree);
        }

        let key = self.children[i].keys.pop().unwrap();
        let value = self.children[i].values.pop().unwrap();
        self.keys.insert(i, key);
        self.values.insert(i, value);
        self.children.insert(i + 1, new_node);
    }

    pub fn insert_non_full(&mut self, id: u32, user: User) {
        let mut i = self.keys.len();

        if self.leaf {
            while i > 0 && self.keys[i - 1] > id {
                i -= 1;
            }
            self.keys.insert(i, id);
            self.values.insert(i, user);
        } else {
            while i > 0 && self.keys[i - 1] > id {
                i -= 1;
            }
            if self.children[i].keys.len() == 2 * T - 1 {
                self.split_child(i);
                if self.keys[i] < id {
                    i += 1;
                }
            }
            self.children[i].insert_non_full(id, user);
        }
    }

    pub fn delete(&mut self, id: u32) -> bool {
        let mut i = 0;
        while i < self.keys.len() && self.keys[i] < id {
            i += 1;
        }

        if i < self.keys.len() && self.keys[i] == id {
            self.keys.remove(i);
            self.values.remove(i);
            true
        } else if self.leaf {
            false
        } else {
            self.children[i].delete(id)
        }
    }
}

#[derive(Debug)]
pub struct BTree {
    pub root: BTreeNode,
}

impl BTree {
    pub fn new() -> Self {
        Self {
            root: BTreeNode::new(true),
        }
    }

    pub fn search(&mut self, id: u32) -> (Option<&mut User>, u32) {
        let mut comparisons = 0;
        let result = self.root.search(id, &mut comparisons);
        (result, comparisons)
    }

    pub fn get_all_users(&self) -> Vec<User> {
        let mut users = Vec::new();
        self.collect_users(&self.root, &mut users);
        users
    }

    fn collect_users(&self, node: &BTreeNode, users: &mut Vec<User>) {
        for user in &node.values {
            users.push(user.clone());
        }
        for child in &node.children {
            self.collect_users(child, users);
        }
    }

    pub fn insert(&mut self, user: User) {
        let id = user.id;
        if self.root.keys.len() == 2 * T - 1 {
            let mut new_root = BTreeNode::new(false);
            new_root
                .children
                .push(std::mem::replace(&mut self.root, BTreeNode::new(true)));
            new_root.split_child(0);
            self.root = new_root;
        }
        self.root.insert_non_full(id, user);
    }

    pub fn delete(&mut self, id: u32) -> bool {
        self.root.delete(id)
    }

    pub fn update(&mut self, id: u32, new_user: User) -> bool {
        if let Some(user) = self.root.search(id, &mut 0) {
            *user = new_user;
            true
        } else {
            false
        }
    }
}
