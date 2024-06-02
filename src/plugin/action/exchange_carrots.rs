use pyo3::*;

use crate::plugin::{ errors::CannotExchangeCarrotsError, game_state::GameState };

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct ExchangeCarrots {
    #[pyo3(get, set)]
    value: i32,
}

#[pymethods]
impl ExchangeCarrots {
    #[new]
    #[must_use]
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn perform(&self, state: &GameState) -> Result<(), PyErr> {
        if state.can_exchange_carrots(&state.get_current(), self.value)? {
            state.get_current().carrots += self.value;
            return Ok(());
        }
        Err(CannotExchangeCarrotsError::new_err("Cannot exhange carrots"))
    }
}