use fnv::FnvHashMap;
use indexmap::IndexMap;
use std::collections::VecDeque;

pub fn distribute<'a, T: 'a, ID>(
    data: &'a [T],
    mut spread: usize,
    id_func: impl Fn(&'a T) -> ID + Send + 'static,
) -> Vec<usize>
where
    ID: Eq + std::hash::Hash,
{
    let mut result = Vec::with_capacity(data.len());
    let mut queue_per_id: FnvHashMap<ID, VecDeque<usize>> = Default::default();
    let mut last_pos: IndexMap<ID, usize> = Default::default();
    let mut output_pos = 0;
    let mut data_pos = 0;
    let mut iterator_reached_end = false;

    loop {
        let item = loop {
            let mut result = None;
            let mut adjust_spread = false;
            let sorted_spreadable_ids = last_pos
                .iter()
                .filter(|(_id, &last_pos)| output_pos - last_pos >= spread)
                .map(|(id, _last_pos)| id);
            for id in sorted_spreadable_ids {
                match queue_per_id.get_mut(id) {
                    Some(queue) => {
                        if let Some(item) = queue.pop_front() {
                            if iterator_reached_end && queue.is_empty() {
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
                if adjust_spread {
                    spread = calculate_spread(&queue_per_id);
                }
                break result;
            }

            if iterator_reached_end {
                if queue_per_id.values().flatten().any(|_| true) {
                    panic!(
                        "Nothing can be returned even though the queue is not empty. This is a bug"
                    );
                } else {
                    break None;
                }
            }

            let current_data_pos = data_pos;
            data_pos += 1;

            match data.get(current_data_pos) {
                Some(item) => {
                    let id = (id_func)(item);
                    if !last_pos.contains_key(&id) {
                        break Some(current_data_pos);
                    } else {
                        queue_per_id
                            .entry(id)
                            .or_insert_with(|| VecDeque::with_capacity(100))
                            .push_back(current_data_pos);
                    }
                }
                None => {
                    spread = calculate_spread(&queue_per_id);
                    iterator_reached_end = true;
                }
            }
        };
        if let Some(output_idx) = item {
            let id = (id_func)(&data[output_idx]);
            last_pos.shift_remove(&id);
            last_pos.insert(id, output_pos);
            result.push(output_idx);
            output_pos += 1;
        } else {
            break;
        }
    }
    result
}

/// Distribute items that are themselves IDs
pub fn distribute_ids<T>(data: &[T], spread: usize) -> Vec<usize> where T: Eq + std::hash::Hash, {
    distribute(data, spread, |item| item)
}

fn calculate_spread<T, ID>(queue_per_id: &FnvHashMap<ID, VecDeque<T>>) -> usize {
    queue_per_id
        .iter()
        .filter(|(_id, queue)| !queue.is_empty())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute() {
        let data = vec![
            "1", "1", "1", "2", "3", "3", "2", "2", "3", "3", "2", "3", "2", "3", "3",
        ];
        let result = distribute(&data, 3, |item| item.parse::<usize>().unwrap());
        assert_eq!(
            result,
            vec![0, 3, 4, 1, 6, 5, 2, 7, 8, 10, 9, 12, 11, 13, 14]
        );
    }

    #[test]
    fn test_distribute2() {
        let data = vec!["Picture", "Post", "Video", "Video", "Picture", "Post", "Picture", "Picture", "Video"];
        let result = distribute_ids(&data, 3);
        let result_with_labels = result
            .iter()
            .map(|idx| data[*idx])
            .collect::<Vec<_>>();
        assert_eq!(result_with_labels, vec!["Picture", "Post", "Video", "Picture", "Post", "Video", "Picture", "Video", "Picture"]);
        assert_eq!(result, vec![0, 1, 2, 4, 5, 3, 6, 8, 7]);
    }
}
