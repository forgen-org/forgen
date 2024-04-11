use std::{future::Future, pin::Pin, sync::Arc};

pub struct Fsm<S> {
    actions: Vec<Box<dyn Fn(Arc<S>) -> Pin<Box<dyn Future<Output = Option<S>>>>>>,
}

impl<S> Fsm<S> {
    pub fn new() -> Self {
        Self { actions: vec![] }
    }

    pub fn register<'a, F, T>(&mut self, action: F)
    where
        F: Fn(Arc<S>) -> T + 'static,
        T: Future<Output = Option<S>> + 'static,
    {
        self.actions
            .push(Box::new(move |state| Box::pin(action(state))));
    }

    pub async fn next(&self, state: S) -> Option<S> {
        let initial = Arc::new(state);
        let mut state = None;
        for action in &self.actions {
            let next = action(initial.clone()).await;
            if next.is_some() {
                state = next;
            }
        }
        state
    }

    pub async fn run(&self, init: S) {
        let mut state = init;
        while let Some(next_state) = self.next(state).await {
            state = next_state;
        }
    }
}
