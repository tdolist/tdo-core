extern crate tdo_core;


#[cfg(test)]
mod storage {
    use tdo_core::{todo, list};

    #[test]
    fn save() {
        let mut tdo = list::Tdo::new();
        let mut li = list::TodoList::new("test");
        let tdo1 = todo::Todo::new(0, "First Entry");
        let tdo2 = todo::Todo::new(1, "Second Entry");

        li.list.push(tdo1.clone());
        li.list.push(tdo2);
        tdo.lists.push(li);


        let res = tdo.save("foo.json");
        assert_eq!(res.unwrap(), ());
    }

    #[test]
    fn load() {
        let mut tdo = list::Tdo::new();
        let mut li = list::TodoList::new("test");
        let tdo1 = todo::Todo::new(0, "First Entry");

        li.list.push(tdo1.clone());
        tdo.lists.push(li);


        let res = tdo.save("foo2.json");

        let new_tdo = list::Tdo::load("foo2.json").unwrap();
        assert_eq!(new_tdo.lists[0].list[0].name, "First Entry");
    }

}
