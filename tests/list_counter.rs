extern crate tdo_core;


#[cfg(test)]
mod counter {
    use tdo_core::{tdo, list, todo};

    #[test]
    fn highest_id_single_list() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        assert_eq!(tdo.get_highest_id(), 1);
    }

    #[test]
    fn highest_id_multiple_list() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        tdo.add_todo(None, todo::Todo::new(2, "Random third Entry")).unwrap();
        tdo.add_todo(None, todo::Todo::new(3, "Funny fourth Entry")).unwrap();

        assert_eq!(tdo.get_highest_id(), 3);
    }

    #[test]
    fn remove_number_between() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        tdo.add_todo(None, todo::Todo::new(2, "Random third Entry")).unwrap();
        tdo.add_todo(None, todo::Todo::new(3, "Funny fourth Entry")).unwrap();

        tdo.remove_id(2).unwrap();
        assert_eq!(tdo.get_highest_id(), 3);
    }

    #[test]
    fn remove_highest_id() {
        let mut tdo = tdo::Tdo::new();
        tdo.add_list(list::TodoList::new("test")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(0, "First Entry")).unwrap();
        tdo.add_todo(Some("test"), todo::Todo::new(1, "Second Entry")).unwrap();

        tdo.add_todo(None, todo::Todo::new(2, "Random third Entry")).unwrap();
        tdo.add_todo(None, todo::Todo::new(3, "Funny fourth Entry")).unwrap();

        tdo.remove_id(3).unwrap();
        assert_eq!(tdo.get_highest_id(), 2);
    }
}
