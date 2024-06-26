/// This trait is used to define the length of the target.
/// # Example
/// ```rust
/// use refined_type::rule::LengthDefinition;
/// let target = "12345";
/// assert_eq!(target.length(), 5);
/// ```
pub trait LengthDefinition {
    fn length(&self) -> usize;
}

impl LengthDefinition for str {
    fn length(&self) -> usize {
        self.chars().count()
    }
}

impl LengthDefinition for &str {
    fn length(&self) -> usize {
        self.chars().count()
    }
}

impl LengthDefinition for String {
    fn length(&self) -> usize {
        self.chars().count()
    }
}

impl<T> LengthDefinition for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> LengthDefinition for [T] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<K, V> LengthDefinition for std::collections::HashMap<K, V> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<K, V> LengthDefinition for std::collections::BTreeMap<K, V> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> LengthDefinition for std::collections::HashSet<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> LengthDefinition for std::collections::BTreeSet<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> LengthDefinition for std::collections::LinkedList<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> LengthDefinition for std::collections::VecDeque<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> LengthDefinition for std::collections::BinaryHeap<T> {
    fn length(&self) -> usize {
        self.len()
    }
}
