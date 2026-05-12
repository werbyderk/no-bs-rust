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

impl<F> Iterator for GraphMeIterator<F>
where
    F: FnMut(f64) -> f64,
{
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.x > self.graph_me.bounds.1 {
            return None;
        }
        let f = &mut self.graph_me.func;
        let x_eval = f(self.x + (self.graph_me.step * 0.5));
        let x_area = x_eval * self.graph_me.step;
        self.x += self.graph_me.step;
        Some(x_area)
    }
}

#[cfg(test)]
mod tests {
    use super::{GraphMe, GraphMeIterator};
    #[test]
    fn integral_1_5() {
        let graph_me = GraphMe {
            func: |x| (1.0 / 5.0) * x,
            step: 0.0000001,
            bounds: (0.0, 5.0),
        };

        let graph_me_iter = GraphMeIterator::new(graph_me);
        let graph_area: f64 = graph_me_iter.sum();
        dbg!(&graph_area);
        assert!((2.49..=2.51).contains(&graph_area));
    }
}
