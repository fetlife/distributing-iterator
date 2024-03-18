use fnv::FnvHashMap;
use indexmap::IndexMap;
use std::collections::VecDeque;

pub struct DistributionIterator<T, ID> {
    data: VecDeque<T>,
    pos: usize,
    original_size: usize,
    spread: usize,
    queue_per_id: Option<FnvHashMap<ID, VecDeque<T>>>,
    last_pos: IndexMap<ID, usize>,
    iterator_reached_end: bool,
    id_func: Box<dyn Fn(&T) -> ID + Send>,
}

impl<T, ID> DistributionIterator<T, ID>
where
    ID: Eq + std::hash::Hash,
{
    pub fn new(
        data: VecDeque<T>,
        spread: usize,
        id_func: impl Fn(&T) -> ID + Send + 'static,
    ) -> Self {
        let original_size = data.len();
        Self {
            data,
            original_size,
            spread,
            pos: 0,
            id_func: Box::new(id_func),
            queue_per_id: Some(FnvHashMap::default()),
            last_pos: IndexMap::default(),
            iterator_reached_end: false,
        }
    }

    pub fn take_next(&mut self) -> Option<T> {
        let item = self.get_next_item();
        match item {
            Some(item) => {
                let id = (self.id_func)(&item);
                self.last_pos.shift_remove(&id); // remove from sorted Hash
                self.last_pos.insert(id, self.pos); // insert at last pos
                self.pos += 1;
                Some(item)
            }
            None => None,
        }
    }

    fn get_next_item(&mut self) -> Option<T> {
        let mut queue_per_id = self.queue_per_id.take().unwrap();
        let mut adjust_spread = false;

        let result = loop {
            let mut result = None;
            for id in self.sorted_spreadable_ids() {
                match queue_per_id.get_mut(id) {
                    Some(queue) => {
                        if let Some(item) = queue.pop_front() {
                            if self.iterator_reached_end && queue.is_empty() {
                                queue_per_id.remove(id);
                                adjust_spread = true
                            }
                            result = Some(item);
                            break;
                        }
                    }
                    None => continue,
                }
            }
            if result.is_some() {
                break result;
            }

            if self.iterator_reached_end {
                if queue_per_id.values().flatten().any(|_| true) {
                    panic!(
                        "Nothing can be returned even though the queue is not empty. This is a bug"
                    );
                } else {
                    break None;
                }
            }

            match self.data.pop_front() {
                Some(item) => {
                    let id = (self.id_func)(&item);
                    if !self.last_pos.contains_key(&id) {
                        break Some(item);
                    } else {
                        // queue_per_id.entry(id).or_default().push_back(item);
                        queue_per_id
                            .entry(id)
                            .or_insert_with(|| VecDeque::with_capacity(100))
                            .push_back(item);
                    }
                }
                None => {
                    self.spread = Self::calculate_spread(&queue_per_id);
                    self.iterator_reached_end = true;
                }
            }
        };
        if adjust_spread {
            self.spread = Self::calculate_spread(&queue_per_id);
        }
        self.queue_per_id = Some(queue_per_id);
        result
    }

    fn sorted_spreadable_ids(&self) -> impl Iterator<Item = &ID> {
        self.last_pos
            .iter()
            .filter(|(_id, &last_pos)| self.pos - last_pos >= self.spread)
            .map(|(id, _last_pos)| id)
    }

    fn calculate_spread(queue_per_id: &FnvHashMap<ID, VecDeque<T>>) -> usize {
        queue_per_id
            .iter()
            .filter(|(_id, queue)| !queue.is_empty())
            .count()
    }
}

impl<T, ID> Iterator for DistributionIterator<T, ID>
where
    T: std::fmt::Debug,
    ID: Eq + std::hash::Hash + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.take_next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.original_size - self.pos;
        (len, Some(len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Item {
        id: u64,
    }

    #[test]
    fn test_distributing_iterator() {
        let data = vec![
            Item { id: 1 },
            Item { id: 1 },
            Item { id: 1 },
            Item { id: 1 },
            Item { id: 1 },
            Item { id: 1 },
            Item { id: 1 },
            Item { id: 2 },
            Item { id: 3 },
            Item { id: 3 },
            Item { id: 2 },
            Item { id: 2 },
            Item { id: 2 },
            Item { id: 3 },
            Item { id: 3 },
        ];
        let iterator = DistributionIterator::new(data.into(), 3, |item| item.id);
        let data: Vec<_> = iterator.collect();
        assert_eq!(
            data,
            vec![
                Item { id: 1 },
                Item { id: 2 },
                Item { id: 3 },
                Item { id: 1 },
                Item { id: 2 },
                Item { id: 3 },
                Item { id: 1 },
                Item { id: 2 },
                Item { id: 3 },
                Item { id: 1 },
                Item { id: 2 },
                Item { id: 3 },
                Item { id: 1 },
                Item { id: 1 },
                Item { id: 1 },
            ]
        );
    }
}
