#![allow(dead_code)]

struct GraphMe<F>
where
    F: FnMut(f64) -> f64,
{
    step: f64,
    bounds: (f64, f64),
    func: F,
}

struct GraphMeIterator<F>
where
    F: FnMut(f64) -> f64,
{
    graph_me: GraphMe<F>,
    x: f64,
}

impl<F> GraphMeIterator<F>
where
    F: FnMut(f64) -> f64,
{
    fn new(graph_me: GraphMe<F>) -> GraphMeIterator<F> {
        let x = graph_me.bounds.0;
        GraphMeIterator {
            graph_me: graph_me,
            x: x,
        }
    }
}

// ** START EDITS HERE **

// Implement the Iterator trait for GraphMeIterator
// next() method should:
// 1. return None if self.x is out of bounds
// 2. evaluate self.graph_me.func(self.x + (self.graph_me.step * 0.5)) * self.graph_me.step
//    - hint: we've allowed self.graph_me.func to mutate its environment. if we try calling func directly,
//      we get an immutable ref to it. how can we fix that?
// 3. increment self.x by self.graph_me.step
// 4. return the calculated area for the last self.x value

// ** END EDITS HERE **

#[cfg(test)]
mod tests {
    use super::{GraphMe, GraphMeIterator};
    #[test]
    fn integral_1_5() {
        let mut total_steps = 0;
        let step = 0.0000001;
        let graph_me = GraphMe {
            // a closure that is allowed to mutate its environment
            func: |x| {
                total_steps += 1;
                (1.0 / 5.0) * x
            },
            step: step,
            bounds: (0.0, 5.0),
        };

        let graph_me_iter = GraphMeIterator::new(graph_me);
        let graph_area: f64 = graph_me_iter.sum();

        // the integral[0,5] of f(x) = (1/5)*x should evaluate to 2.5
        assert!((2.49..=2.51).contains(&graph_area));
        // and we should have called our closure `(5 / step) + 1` times
        assert_eq!(total_steps as f64, (5.0 / step) + 1.0);
    }
}
