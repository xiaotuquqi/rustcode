#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{decl_module, decl_storage, decl_event, 
	ensure, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp-std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {

	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}


decl_storage! {


	trait Store for Module<T: Trait> as TemplateModule {


		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> =>(T::AccountId, T::BlcokNumber);
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {

		ClaimRevoked(AccountId , Vec<u8>),
		ClaimCreated(AccountId , Vec<u8>),
	}
);


decl_error! {
	pub enum Error for Module<T: Trait> {
		ClaimNotExist,
		ProofAlreadyExist,
		NotClaimOwner,
}


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		type Error = Error<T>;


		fn deposit_event() = default;


		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) ->dispatch::DispatchResult{
				let sender = ensure_signed(origin)?;
				ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
				Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Module::<T>::block_number()));
				Self::deposit_event(RawEvent::ClaimCreated(sender , claim));
			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) ->dispatch::DispatchResult{
				let sender = ensure_signed(origin)?;
				ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
				let (owner, _block_number) = Proofs::<T>::get(&claim);
				ensure!( owner == sender ,Error::<T>::NotClaimOwner); 
				Proofs::<T>::remove(&claim);
				Self::deposit_event(RawEvent::ClaimRevoked(sender , claim));
			Ok(())
		}






	}
}
