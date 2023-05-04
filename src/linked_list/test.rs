use super::*;

#[test]
fn test_singly() {
    let mut list = SinglyLinkedList::new();
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.is_empty(), true);
    assert_eq!(list.size(), 0);

    // push front pop front
    list.push_front(1).unwrap();
    list.push_front(2).unwrap();
    assert_eq!(list.size(), 2);
    assert_eq!(list.is_empty(), false);
    list.push_front(3).unwrap();

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.size(), 1);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.size(), 0);
    assert_eq!(list.is_empty(), true);

    // push back pop front
    list.push_back(1).unwrap();
    list.push_back(2).unwrap();
    list.push_back(3).unwrap();

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), None);

    // push front pop back
    list.push_front(1).unwrap();
    list.push_front(2).unwrap();
    list.push_front(3).unwrap();

    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), None);

    // push back pop back
    list.push_back(1).unwrap();
    list.push_back(2).unwrap();
    list.push_back(3).unwrap();

    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);

    // peek front and back
    assert_eq!(list.peek_front(), None);
    assert_eq!(list.peek_back(), None);

    list.push_back(1).unwrap();
    assert_eq!(list.peek_front(), Some(&1));
    assert_eq!(list.peek_back(), Some(&1));

    list.push_back(2).unwrap();
    list.push_back(3).unwrap();

    assert_eq!(list.peek_front(), Some(&1));
    assert_eq!(list.peek_back(), Some(&3));

    // test iters
    for x in &mut list {
        *x = *x + 10;
    }

    for (i, x) in (&list).into_iter().enumerate() {
        assert_eq!(x, &(i + 11))
    }

    for (i, x) in list.into_iter().enumerate() {
        assert_eq!(x, i + 11)
    }
}
