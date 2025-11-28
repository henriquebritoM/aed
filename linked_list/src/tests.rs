 #[cfg(test)]
    mod tests {
        // use crate::linked_list::LinkedList;
        use crate::linked_list_simple::LinkedList;

        #[test] 
        fn new() {
            let ll: LinkedList<i32> = LinkedList::new();

            assert_eq!(ll.len(), 0);
        }

        #[test] 
        fn push_first() {
            let expected = vec![&1];
            let mut ll: LinkedList<i32> = LinkedList::new();
            ll.push(1);

            assert_eq!(ll.len(), 1);
            assert_eq!(ll.list(), expected);
        }

        #[test] 
        fn push_many() {
            let expected = vec![&1, &2, &3, &4];
            let mut ll: LinkedList<i32> = LinkedList::new();
            ll.push(1);
            ll.push(2);
            ll.push(3);
            ll.push(4);

            assert_eq!(ll.len(), 4);
            assert_eq!(ll.list(), expected);
        }

        // #[test] 
        // fn remove_head() {
        //     let expected: Vec<&i32> = Vec::new();
        //     let mut ll: LinkedList<i32> = LinkedList::new();
        //     ll.push(1);

        //     ll.remove(1);

        //     assert_eq!(ll.len(), 0);
        //     assert_eq!(ll.list(), expected);
        // }

        // #[test] 
        // fn remove_last() {
        //     let expected: Vec<&i32> = vec![&1, &2, &3];
        //     let mut ll: LinkedList<i32> = LinkedList::new();
        //     ll.push(1);
        //     ll.push(2);
        //     ll.push(3);
        //     ll.push(4);

        //     ll.remove(4);

        //     assert_eq!(ll.len(), 3);
        //     assert_eq!(ll.list(), expected);
        // }

        // #[test] 
        // fn remove_midle() {
        //     let expected: Vec<&i32> = vec![&1, &2, &4];
        //     let mut ll: LinkedList<i32> = LinkedList::new();
        //     ll.push(1);
        //     ll.push(2);
        //     ll.push(3);
        //     ll.push(4);

        //     ll.remove(3);

        //     assert_eq!(ll.len(), 3);
        //     assert_eq!(ll.list(), expected);
        // }

        // #[test] 
        // fn remove_many() {
        //     let expected: Vec<&i32> = vec![&2];
        //     let mut ll: LinkedList<i32> = LinkedList::new();
        //     ll.push(1);
        //     ll.push(2);
        //     ll.push(3);
        //     ll.push(4);

        //     ll.remove(3);
        //     ll.remove(1);
        //     ll.remove(4);

        //     assert_eq!(ll.len(), 1);
        //     assert_eq!(ll.list(), expected);
        // }

        // #[test] 
        // fn remove_not_found() {
        //     let expected: Vec<&i32> = vec![&1, &2, &3, &4];
        //     let mut ll: LinkedList<i32> = LinkedList::new();
        //     ll.push(1);
        //     ll.push(2);
        //     ll.push(3);
        //     ll.push(4);

        //     ll.remove(5);

        //     assert_eq!(ll.len(), 4);
        //     assert_eq!(ll.list(), expected);
        // }

    }