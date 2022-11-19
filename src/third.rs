use std::rc::Rc;
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

pub struct Iter<'a,T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct Node<T>  {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None
        }
    }

    pub fn prepend(&self, elem: T) -> List<T> { 
        List { 
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
       List {
            head: match self.head.as_ref() {
                Some(node) => { 
                    node.next.clone()
                },
                None => None
            }
       }
    }

    pub fn head(&self)->Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
}

impl<'a, T> List<T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref()
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head  {
            if let Ok(mut node) = Rc::try_unwrap(node) { 
                head = node.next.take();
            }else {
                break
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn basics() {
        let list = List::new();

        assert_eq!(list.head(), None);
        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));
        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter(); 
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}