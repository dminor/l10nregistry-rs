use super::ProblemSolver;
use std::ops::{Deref, DerefMut};

use futures::ready;
use std::future::Future;
use std::pin::Pin;

pub trait AsyncTester {
    type Result: Future<Output = Vec<bool>>;

    fn test_async(&self, query: Vec<(usize, usize)>) -> Self::Result;
}

pub struct ParallelProblemSolver<T>
where
    T: AsyncTester,
{
    solver: ProblemSolver,
    current_test: Option<(T::Result, Vec<usize>)>,
}

impl<T: AsyncTester> Deref for ParallelProblemSolver<T> {
    type Target = ProblemSolver;

    fn deref(&self) -> &Self::Target {
        &self.solver
    }
}

impl<T: AsyncTester> DerefMut for ParallelProblemSolver<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.solver
    }
}

impl<T: AsyncTester> ParallelProblemSolver<T> {
    pub fn new(width: usize, depth: usize) -> Self {
        Self {
            solver: ProblemSolver::new(width, depth),
            current_test: None,
        }
    }
}

impl<T: AsyncTester> ParallelProblemSolver<T> {
    pub fn try_generate_complete_candidate(&mut self) -> bool {
        while !self.is_complete() {
            while self.is_current_cell_missing() {
                if !self.try_advance_source() {
                    return false;
                }
            }
            if !self.try_advance_resource() {
                return false;
            }
        }
        true
    }

    fn try_generate_test_query(&mut self) -> Result<(Vec<(usize, usize)>, Vec<usize>), usize> {
        let mut test_cells = vec![];
        let query = self
            .solution
            .iter()
            .enumerate()
            .filter_map(|(res_idx, source_idx)| {
                let cell = self.cache[res_idx][*source_idx];
                match cell {
                    None => {
                        test_cells.push(res_idx);
                        Some(Ok((res_idx, *source_idx)))
                    }
                    Some(false) => Some(Err(res_idx)),
                    Some(true) => None,
                }
            })
            .collect::<Result<_, _>>()?;
        Ok((query, test_cells))
    }

    fn apply_test_result(
        &mut self,
        resources: Vec<bool>,
        testing_cells: &[usize],
    ) -> Result<(), usize> {
        for (missing_idx, res) in resources.iter().enumerate() {
            let res_idx = testing_cells[missing_idx];
            if *res {
                let source_idx = self.solution[res_idx];
                self.cache[res_idx][source_idx] = Some(true);
            } else {
                return Err(res_idx);
            }
        }
        Ok(())
    }

    pub fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        tester: &T,
    ) -> std::task::Poll<Option<Vec<usize>>>
    where
        <T as AsyncTester>::Result: Unpin,
    {
        if self.width == 0 || self.depth == 0 {
            return None.into();
        }

        'outer: loop {
            if let Some((test, testing_cells)) = &mut self.current_test {
                let pinned = Pin::new(test);
                let set = ready!(pinned.poll(cx));
                let testing_cells = testing_cells.clone();

                if let Err(res_idx) = self.apply_test_result(set, &testing_cells) {
                    self.idx = res_idx;
                    self.prune();
                    if !self.bail() {
                        return None.into();
                    }
                    self.current_test = None;
                    continue 'outer;
                } else {
                    self.current_test = None;
                    self.dirty = true;
                    return Some(self.solution.clone()).into();
                }
            } else {
                if self.dirty {
                    if !self.bail() {
                        return None.into();
                    }
                    self.dirty = false;
                }
                while self.try_generate_complete_candidate() {
                    match self.try_generate_test_query() {
                        Ok((query, testing_cells)) => {
                            self.current_test = Some((tester.test_async(query), testing_cells));
                            continue 'outer;
                        }
                        Err(res_idx) => {
                            self.idx = res_idx;
                            self.prune();
                            if !self.bail() {
                                return None.into();
                            }
                        }
                    }
                }
                return None.into();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem_solver() {
        // let keys = vec!["key1.ftl", "key2.ftl"];
        // let sources = vec!["source1", "source2"];
        // let args = ("foo",);

        // let ps = ProblemSolver::new(keys.len(), sources.len(), &foo);
    }
}
