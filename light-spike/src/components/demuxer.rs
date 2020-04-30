use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{io::*, scheduler::*, verifier::*};
use crate::prelude::*;

#[derive(Clone, Debug, Error, PartialEq, Eq, Serialize, Deserialize)]
pub enum DemuxerError {
    #[error("scheduler error")]
    Scheduler(SchedulerError),
    #[error("verifier error")]
    Verifier(VerifierError),
    #[error("I/O error")]
    Io(IoError),
}

pub struct Demuxer {
    state: State,
    scheduler: Scheduler,
    verifier: Verifier,
    io: Io,
}

impl Demuxer {
    pub fn new(state: State, scheduler: Scheduler, verifier: Verifier, io: Io) -> Self {
        Self {
            state,
            scheduler,
            verifier,
            io,
        }
    }

    pub fn verify_height(
        &mut self,
        height: Height,
        trusted_state: TrustedState,
        options: VerificationOptions,
    ) -> Result<Vec<LightBlock>, DemuxerError> {
        let input = SchedulerInput::VerifyHeight {
            height,
            trusted_state,
            options,
        };

        let result = self.run_scheduler(input)?;

        match result {
            SchedulerOutput::TrustedStates(trusted_states) => {
                self.state.add_trusted_states(trusted_states.clone());
                Ok(trusted_states)
            }
        }
    }

    pub fn verify_light_block(
        &mut self,
        light_block: LightBlock,
        trusted_state: TrustedState,
        options: VerificationOptions,
    ) -> Result<Vec<LightBlock>, DemuxerError> {
        let input = SchedulerInput::VerifyLightBlock {
            light_block,
            trusted_state,
            options,
        };

        let result = self.run_scheduler(input)?;

        match result {
            SchedulerOutput::TrustedStates(trusted_states) => {
                self.state.add_trusted_states(trusted_states.clone());
                Ok(trusted_states)
            }
        }
    }

    pub fn validate_light_block(
        &mut self,
        light_block: LightBlock,
        trusted_state: TrustedState,
        options: VerificationOptions,
    ) -> Result<LightBlock, DemuxerError> {
        let input = VerifierInput::VerifyLightBlock {
            light_block,
            trusted_state,
            options,
        };

        let result = self
            .verifier
            .process(input)
            .map_err(|e| DemuxerError::Verifier(e))?;

        match result {
            VerifierOutput::ValidLightBlock(valid_light_block) => {
                self.state.add_valid_light_block(valid_light_block.clone());
                Ok(valid_light_block)
            }
        }
    }

    pub fn fetch_light_block(&mut self, height: Height) -> Result<LightBlock, DemuxerError> {
        let input = IoInput::FetchLightBlock(height);

        let result = self.io.process(input).map_err(|e| DemuxerError::Io(e))?;

        match result {
            IoOutput::FetchedLightBlock(lb) => {
                self.state.add_fetched_light_block(lb.clone());
                Ok(lb)
            }
        }
    }

    fn handle_request(
        &mut self,
        request: SchedulerRequest,
    ) -> Result<SchedulerResponse, DemuxerError> {
        match request {
            SchedulerRequest::GetLightBlock(height) => self
                .fetch_light_block(height)
                .map(|lb| SchedulerResponse::LightBlock(lb)),

            SchedulerRequest::VerifyLightBlock {
                light_block,
                trusted_state,
                options,
            } => match self.verify_light_block(light_block, trusted_state, options) {
                Ok(ts) => Ok(SchedulerResponse::Verified(Ok(ts))),
                Err(DemuxerError::Verifier(err)) => Ok(SchedulerResponse::Verified(Err(err))),
                Err(err) => Err(err),
            },

            SchedulerRequest::ValidateLightBlock {
                light_block,
                trusted_state,
                options,
            } => match self.validate_light_block(light_block, trusted_state, options) {
                Ok(ts) => Ok(SchedulerResponse::Validated(Ok(ts))),
                Err(DemuxerError::Verifier(err)) => Ok(SchedulerResponse::Validated(Err(err))),
                Err(err) => Err(err),
            },
        }
    }

    fn run_scheduler(&mut self, input: SchedulerInput) -> Result<SchedulerOutput, DemuxerError> {
        let scheduler = Gen::new(|co| {
            let handler = &self.scheduler;
            handler(self.state.trusted_store_reader(), input, co)
        });

        let result = drain(scheduler, SchedulerResponse::Init, move |req| {
            self.handle_request(req)
        })?;

        result.map_err(|e| DemuxerError::Scheduler(e))
    }
}
