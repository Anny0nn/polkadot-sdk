// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::Config;
use codec::{Decode, Encode};
use frame_support::DebugNoBound;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{DispatchInfoOf, SignedExtension},
	transaction_validity::{TransactionValidityError, UnknownTransaction},
};

/// Genesis hash check to provide replay protection between different networks.
///
/// # Transaction Validity
///
/// Note that while a transaction with invalid `genesis_hash` will fail to be decoded,
/// the extension does not affect any other fields of `TransactionValidity` directly.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, DebugNoBound)]
#[scale_info(skip_type_params(T))]
pub struct CheckMetadataHash<T> {
	_phantom: core::marker::PhantomData<T>,
	enable: bool,
}

impl<T> CheckMetadataHash<T> {
	/// Creates new `SignedExtension` to check metadata hash.
	pub fn new(enable: bool) -> Self {
		Self { _phantom: core::marker::PhantomData, enable }
	}
}

impl<T: Config + Send + Sync> SignedExtension for CheckMetadataHash<T> {
	type AccountId = T::AccountId;
	type Call = <T as Config>::RuntimeCall;
	type AdditionalSigned = Option<[u8; 32]>;
	type Pre = ();
	const IDENTIFIER: &'static str = "CheckMetadataHash";

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		if self.enable {
			match option_env!("RUNTIME_METADATA_HASH") {
				Some(hash) => Ok(Some(array_bytes::hex2array_unchecked(hash))),
				None => Err(UnknownTransaction::CannotLookup.into()),
			}
		} else {
			Ok(None)
		}
	}

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		self.validate(who, call, info, len).map(|_| ())
	}
}
