use std::iter::Iterator;

pub struct UniquePermutations<'a, V> {
    stack: Vec<(Box<[&'a V]>, usize)>,
    len: usize,
}

impl<'a, V> UniquePermutations<'a, V> {
    fn new<'b>(items: &'b [V], len: usize) -> UniquePermutations<'a, V>
    where
        'b: 'a,
    {
        let top: Box<_> = items.iter().collect();
        let stack = vec![(top, 0)];

        UniquePermutations { stack, len }
    }

    fn next_stack_item(&self) -> Option<(Box<[&'a V]>, usize)> {
        self.stack.last().and_then(|(items, current)| {
            if *current >= items.len() {
                return None;
            }

            let n: Box<_> = items
                .iter()
                .enumerate()
                .filter(|(idx, _)| idx != current)
                .map(|(_, v)| *v)
                .collect();

            Some((n, 0))
        })
    }

    fn generate_next_stack(&mut self) {
        loop {
            let last = self.stack.last();
            if last.is_some() && last.unwrap().1 >= last.unwrap().0.len() {
                self.stack.pop();
                if let Some((_, curr)) = self.stack.last_mut() {
                    *curr += 1;
                }
            }
            if self.stack.is_empty() || self.stack.len() == self.len {
                break;
            }

            match self.next_stack_item() {
                None => {
                    self.stack.pop();
                    if let Some((_, curr)) = self.stack.last_mut() {
                        *curr += 1;
                    }
                }
                Some(next_stack) => {
                    self.stack.push(next_stack);
                }
            }
        }
    }
}

impl<'a, V> Iterator for UniquePermutations<'a, V> {
    type Item = Vec<&'a V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.generate_next_stack();

        if self.stack.is_empty() {
            return None;
        }

        let next_item: Vec<_> = self
            .stack
            .iter()
            .map(|(items, current)| items.get(*current).cloned())
            .flatten()
            .collect();

        if let Some((_, current)) = self.stack.last_mut() {
            *current += 1
        }

        Some(next_item)
    }
}

pub trait Permutations<'a> {
    type Item;
    fn unique_permutations(&'a self) -> UniquePermutations<'a, Self::Item>;
}

impl<'a, V> Permutations<'a> for Vec<V> {
    type Item = V;

    fn unique_permutations(&'a self) -> UniquePermutations<'a, Self::Item> {
        UniquePermutations::new(self, self.len())
    }
}

impl<'a, V> Permutations<'a> for &'a [V] {
    type Item = V;

    fn unique_permutations(&'a self) -> UniquePermutations<'a, Self::Item> {
        UniquePermutations::new(self, self.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_size_one() {
        let data: Vec<i32> = vec![0];
        let result: Vec<_> = data.unique_permutations().collect();

        assert_eq!(result, [[&0]]);
    }

    #[test]
    fn test_permutations_size_two() {
        let data: Vec<i32> = vec![0, 1];
        let result: Vec<_> = data.unique_permutations().collect();

        assert_eq!(result, [[&0, &1], [&1, &0]]);
    }

    #[test]
    fn test_permutations_size_three() {
        let data: Vec<i32> = vec![0, 1, 2];
        let result: Vec<_> = data.unique_permutations().collect();

        assert_eq!(
            result,
            [
                [&0, &1, &2],
                [&0, &2, &1],
                [&1, &0, &2],
                [&1, &2, &0],
                [&2, &0, &1],
                [&2, &1, &0]
            ]
        );
    }
}
