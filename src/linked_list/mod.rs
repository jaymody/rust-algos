pub mod singly;

trait LinkedList<T>
where
    Self: IntoIterator<Item = T>,
    for<'a> &'a Self: IntoIterator<Item = &'a T>,
    for<'a> &'a mut Self: IntoIterator<Item = &'a mut T>,
{
    fn push_front(&mut self, item: T) -> Result<(), String>;

    fn push_back(&mut self, item: T) -> Result<(), String>;

    fn pop_front(&mut self) -> Option<T>;

    fn pop_back(&mut self) -> Option<T>;

    fn peek_front(&self) -> Option<&T>;

    fn peek_back(&self) -> Option<&T>;

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;

    // fn push_at(&mut self, index: usize, item: T) -> Result<(), String>;

    // fn pop_at();

    // fn peek_at();

    // fn reverse(&mut self);
}
