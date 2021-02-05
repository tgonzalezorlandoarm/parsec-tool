// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Lists the supported opcodes for a given provider.

use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::ProviderOpts;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::auth::Authentication;
use parsec_client::core::interface::operations::list_opcodes;
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::interface::requests::ProviderID;
use parsec_client::core::operation_client::OperationClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Lists the supported opcodes for a given provider.
#[derive(Debug, StructOpt)]
pub struct ListOpcodes {
    #[structopt(flatten)]
    provider_opts: ProviderOpts,
}

impl TryFrom<&ListOpcodes> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(list_opcodes_subcommand: &ListOpcodes) -> Result<Self, Self::Error> {
        // Trivially converted to a `NativeOperation`.
        Ok(NativeOperation::ListOpcodes(list_opcodes::Operation {
            provider_id: list_opcodes_subcommand.provider_opts.provider()?,
        }))
    }
}

impl ParsecToolSubcommand<'_> for ListOpcodes {
    /// Lists the supported opcodes for a given provider.
    fn run(&self, _matches: &ParsecToolApp) -> Result<(), ParsecToolError> {
        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            // We still use the core provider because listing opcodes is a core operation. Note the
            // distinction between the provider we're _using_ and the provider we're querying.
            ProviderID::Core,
            &Authentication::None,
        )?;

        if let NativeResult::ListOpcodes(result) = native_result {
            info!(
                "Available opcodes for provider {:?}:",
                self.provider_opts.provider()?
            );
            for provider_opcode in result.opcodes {
                eprint_colored!(Blue, "*");
                eprintln!(" {:?}", provider_opcode);
            }
            Ok(())
        } else {
            Err(ParsecToolError::UnexpectedNativeResult(native_result))
        }
    }
}
