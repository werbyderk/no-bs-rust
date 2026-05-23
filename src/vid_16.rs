#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// A SettingsMenu should own the button it is showing...
struct SettingsMenu {
    is_submitted: RefCell<bool>,
    button: Rc<Button>,
}

// ** START EDITS HERE **

// ...And a Button should have a reference back to its parent component
// but should NOT own it. Otherwise, we'd have a reference cycle!
struct Button {
    text: String,
    // Define settings_menu. It should be a RefCell that holds a Weak reference to SettingsMenu
    settings_menu: RefCell<Weak<SettingsMenu>>,
}

impl Button {
    fn new(text: &str) -> Self {
        Button {
            text: text.to_string(),
            settings_menu: RefCell::new(Weak::new()),
        }
    }
    fn submit(&self) {
        // Upgrade the weak reference to settings_menu and set is_submitted to true
        if let Some(sm) = self.settings_menu.borrow().upgrade() {
            *sm.is_submitted.borrow_mut() = true;
        }
    }
}

// ** END EDITS HERE **

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::vid_16::{Button, SettingsMenu};

    #[test]
    fn can_submit() {
        let button = Rc::new(Button::new("submit me"));
        let sm = Rc::new(SettingsMenu {
            is_submitted: RefCell::new(false),
            button: Rc::clone(&button),
        });

        *button.settings_menu.borrow_mut() = Rc::downgrade(&sm);
        button.submit();
        assert_eq!(*sm.is_submitted.borrow(), true);
    }
}
