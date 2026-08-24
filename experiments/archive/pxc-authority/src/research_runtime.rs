//! Small reusable harness primitives for fast, deterministic research loops.
//!
//! Permanent fixtures are shared immutably. Counterfactual branches allocate
//! only their episode workspace. Independent matrix cells may run in parallel,
//! but each cell remains single-threaded and results retain input order.

use std::ops::Deref;
use std::sync::Arc;
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HarnessMode {
    Micro,
    Gate,
    Definitive,
}

#[derive(Debug)]
pub struct Frozen<T>(Arc<T>);

impl<T> Frozen<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(value))
    }

    pub fn branch<E>(&self, episode: E) -> Branch<T, E> {
        Branch {
            permanent: Arc::clone(&self.0),
            episode,
        }
    }
}

impl<T> Clone for Frozen<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Deref for Frozen<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Branch<T, E> {
    permanent: Arc<T>,
    pub episode: E,
}

impl<T, E> Branch<T, E> {
    pub fn permanent(&self) -> &T {
        &self.permanent
    }
}

/// Runs independent deterministic cells concurrently and restores input order.
pub fn parallel_map_ordered<T, F>(cells: usize, worker: F) -> Vec<T>
where
    T: Send,
    F: Fn(usize) -> T + Sync,
{
    thread::scope(|scope| {
        let handles = (0..cells)
            .map(|index| {
                let worker = &worker;
                (index, scope.spawn(move || worker(index)))
            })
            .collect::<Vec<_>>();
        let mut completed = handles
            .into_iter()
            .map(|(index, handle)| (index, handle.join().expect("research matrix cell panicked")))
            .collect::<Vec<_>>();
        completed.sort_by_key(|(index, _)| *index);
        completed.into_iter().map(|(_, value)| value).collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_branches_share_permanent_state_and_isolate_episode_state() {
        let fixture = Frozen::new(vec![1, 2, 3]);
        let mut first = fixture.branch(vec![4]);
        let second = fixture.branch(vec![5]);
        first.episode.push(6);
        assert_eq!(first.permanent(), second.permanent());
        assert_eq!(first.episode, vec![4, 6]);
        assert_eq!(second.episode, vec![5]);
    }

    #[test]
    fn parallel_cells_return_in_input_order() {
        assert_eq!(
            parallel_map_ordered(8, |index| 7usize.saturating_sub(index)),
            vec![7, 6, 5, 4, 3, 2, 1, 0]
        );
    }
}
