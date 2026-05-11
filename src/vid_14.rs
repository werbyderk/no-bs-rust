#![allow(dead_code)]
use std::rc::Rc;

struct File(String);

struct Folder {
    name: String,
    files: Vec<File>,
}

// ** START EDITS HERE **

struct User {
    pub name: String,
    // Fill out folders definition. It should be a Vec of Folders, and more than one User can reference
    // the same Folder. Hint: Use Rc
    pub folders: None, // <- Replace None with the proper definition
}

fn populate_users() -> (User, User) {
    // Return 2 users (Bob, Alice)
    // Bob should hold folder_a, folder_b with some (or optionally no) files in it
    // Files don't need to be shared across Folders, but Folders should be shareable between Users
    // Alice should hold folder_a as well
}

// ** END EDITS HERE **

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn populate_users() {
        let (bob, alice) = super::populate_users();
        assert_eq!(bob.name, "Bob".to_string());
        assert_eq!(alice.name, "Alice".to_string());

        assert_eq!(Rc::strong_count(&bob.folders[0]), 2);
        assert_eq!(Rc::strong_count(&bob.folders[1]), 1);

        assert_eq!(Rc::strong_count(&alice.folders[0]), 2);
    }
}
