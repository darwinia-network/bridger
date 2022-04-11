use relay_substrate_client::Chain;

use std::marker::PhantomData;

#[derive(Clone)]
pub struct NothingStrategy<T: Chain> {
    _marker: PhantomData<T>,
}

impl<T: Chain> NothingStrategy<T> {
    pub fn new() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}
