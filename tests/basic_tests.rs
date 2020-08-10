extern crate event_behavior_tree;
mod action;

mod decorator {
    use event_behavior_tree::helpers;
    use event_behavior_tree::{BehaviorTree, Status};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn repeater() {
        let x = Rc::new(RefCell::new(0));
        let x_clone = x.clone();
        let action = helpers::pure_action(Box::new(move || {
            x_clone.replace_with(|v| *v + 1);
            Status::Success
        }));
        let mut bt = BehaviorTree::new(helpers::repeater(action, 3));
        bt.run();
        assert_eq!(*x.borrow(), 3);
    }
}

mod sequence {
    use super::action;
    use event_behavior_tree::helpers::*;
    use event_behavior_tree::*;
    #[test]
    fn fail_if_one_fail() {
        let mut bt = BehaviorTree::new(sequence(vec![
            action::succeed(),
            action::fail(),
            action::panic(),
        ]));
        assert_eq!(bt.run(), Status::Failure);
    }

    #[test]
    fn succeed_if_all_succeed() {
        let mut bt = BehaviorTree::new(sequence(vec![action::succeed(), action::succeed()]));
        assert_eq!(bt.run(), Status::Success);
    }
}
