extern crate tdo_core;


#[cfg(test)]
#[allow(unused_must_use)]
mod storage {
    use tdo_core::{todo, list};

    #[test]
    fn save() {
        let mut tdo = list::Tdo::new();
        tdo.add_list(list::TodoList::new("test"));
        tdo.add_todo("test", todo::Todo::new(0, "First Entry"));
        tdo.add_todo("test", todo::Todo::new(1, "Second Entry"));

        let res = tdo.save("foo.json").unwrap();
        assert_eq!(res, ());
    }

    #[test]
    fn load() {
        let mut tdo = list::Tdo::new();
        tdo.add_list(list::TodoList::new("test"));
        tdo.add_todo("test", todo::Todo::new(0, "First Entry"));

        tdo.save("foo2.json");

        let new_tdo = list::Tdo::load("foo2.json").unwrap();
        assert_eq!(new_tdo.lists[1].list[0].name, "First Entry");
    }

}
