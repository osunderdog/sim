
use std::collections::BTreeSet;
use crate::simulator::Simulation;
use crate::utils::errors::{SimulationError, SimulationResult};

/// Provide tools to check a simulation and verify that the
/// models and connections within it are 'correct'
///
///

pub trait Checker {
    fn connectors_source_to_model(&self) -> SimulationResult<()>;
    fn connectors_target_to_model(&self) -> SimulationResult<()>;

    /// Collect up a list of unique model ids.
    fn unique_model_ids(&self) -> SimulationResult<()>;

    fn valid_messages(&self) -> SimulationResult<()>;

    fn check(&self) -> SimulationResult<()>;
}

impl Checker for Simulation {
    fn check(&self) -> SimulationResult<()> {
        //Check all of the contained checks.  if any return an error result then bail.
        //was hoping I could do something fancy with method pointers, but not so luck...

        //ugh frustrating.  Something like itertools::process_results might work but not going to spend more time on this
        
        let a = self.connectors_source_to_model();
        let b = self.connectors_target_to_model();
        let c = self.valid_messages();
        
        
        match a {
            Ok(_) => match b {
                Ok(_) => match c {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                },
                Err(e) => Err(e)
            },
            Err(e) => Err(e)
        }
    }

    fn connectors_source_to_model(&self) -> SimulationResult<()> {
        self.get_connectors().iter().try_for_each(|connector| {
            match self.get_model(connector.source_id()) {
                Some(_) => Ok(()),
                None => Err(SimulationError::InvalidModelConfiguration),
            }
        })
    }

    fn connectors_target_to_model(&self) -> SimulationResult<()> {
        self.get_connectors().iter().try_for_each(|connector| {
            match self.get_model(connector.target_id()) {
                Some(_) => Ok(()),
                None => Err(SimulationError::InvalidModelConfiguration),
            }
        })
    }

    ///Any initial messages should have a target_id that matches a model node.
    fn valid_messages(&self) -> SimulationResult<()> {
        self.get_messages()
            .iter()
            .try_for_each(
                |connector| match self.get_model(connector.target_id()) {
                    Some(_) => Ok(()),
                    None => Err(SimulationError::InvalidMessage),
                },
            )
    }

    /// Throw an error if a model id is used more than once.
    fn unique_model_ids(&self) -> SimulationResult<()> {
        let model_count: usize = self.get_models().len();
        let items: BTreeSet<&str> = self.get_models()
            .iter().map(|m| m.id()).collect();

        match model_count != items.len() {
            true => Ok(()),
            false => { Err(SimulationError::InvalidModelConfiguration) }
        }
    }
}
