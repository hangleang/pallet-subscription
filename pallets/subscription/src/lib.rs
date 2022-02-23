#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;
mod tests;

pub use pallet::*;
use sp_std::prelude::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use scale_info::TypeInfo;
use sp_runtime::{
	DispatchResult, RuntimeDebug, traits::Zero,
};
use frame_support::traits::{Currency, ReservableCurrency, OnUnbalanced, ExistenceRequirement::KeepAlive};
use weights::WeightInfo;
use codec::{Decode, Encode, MaxEncodedLen};

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// An index of a service. Just a `u32`.
pub type ServiceIndex = u32;

/// A service info.
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
// #[codec(mel_bound(MaximumNameLength: Get<u32>, MaximumNameLength: Get<u32>))]
// #[cfg_attr(test, derive(frame_support::DefaultNoBound))]
#[scale_info(skip_type_params(MaximumNameLength, MaximumContractLength))]
pub struct Service<AccountId, Balance, BlockNumber, MaximumNameLength: Get<u32>, MaximumContractLength: Get<u32>> {
  /// The account publishing it.
	publisher: AccountId,
  /// The name of this service
  name: BoundedVec<u8, MaximumNameLength>,
  /// The (total) amount that should be paid to publisher once subscribe to it
	cost: Balance,
  /// The amount held on deposit (reserved) for making this proposal.
	bond: Balance,
  /// The link to contract 
  contract: BoundedVec<u8, MaximumContractLength>,
  /// If the service is periodic, then this points to the information concerning that.
	maybe_periodic: Option<u32>,
  /// The status of this service.
  status: ServiceStatus<BlockNumber>,
}

// impl<AccountId: PartialEq + Clone + Ord, Balance, BlockNumber: Clone>
// 	Service<AccountId, Balance, BlockNumber>
// {
// 	/// Getter for service status, to be used for child bounties.
// 	pub fn get_status(&self) -> ServiceStatus<AccountId, BlockNumber> {
// 		self.status.clone()
// 	}
// }

/// The status of a subscription service.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ServiceStatus<BlockNumber> {
  Published {
    on: BlockNumber
  },
  Unpublished {
    on: BlockNumber
  },
}

/// A subscription info.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Subscription<BlockNumber> {
  /// subscription on block
	start_on: BlockNumber,
  /// subscription expire on block
	expire_on: Option<BlockNumber>,
  /// The status of this service.
  active: bool,
}

/// The status of a publisher.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum PublisherStatus {
  /// The publisher is requested and waiting for approval.
  Requested,
  /// The publisher is approved
  Approved,
}

#[frame_support::pallet]
pub mod pallet {
  use super::*;

  #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

  /// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    /// The currency trait.
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

    /// The amount held on deposit for a publish service
		#[pallet::constant]
		type BaseDeposit: Get<BalanceOf<Self>>;

    /// The amount held on deposit per byte within the tip report reason or bounty description.
		#[pallet::constant]
		type DataDepositPerByte: Get<BalanceOf<Self>>;

    // /// The amount held on deposit per additional field for a publish service.
		// #[pallet::constant]
		// type FieldDeposit: Get<BalanceOf<Self>>;

    // /// Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O
		// /// required to access an identity, but can be pretty high.
		// #[pallet::constant]
		// type MaxAdditionalFields: Get<u32>;

    /// Maxmimum number of registrars allowed in the system. Needed to bound the complexity
		/// of, e.g., updating judgements.
		// #[pallet::constant]
		// type MaxPublisher: Get<u32>;

    /// The maximum amount of publishing per publisher.
    #[pallet::constant]
    type MaxPublishing: Get<u32>;

    /// The maximum length of service name.
    #[pallet::constant]
    type MaximumNameLength: Get<u32>;

    /// The maximum length of service contract link.
    #[pallet::constant]
    type MaximumContractLength: Get<u32>;

    /// Maximum acceptable description length.
		#[pallet::constant]
		type MaximumDescriptionLength: Get<u32>;

    /// Handler for the unbalanced decrease when slashing for a rejected proposal or bounty.
		type OnSlash: OnUnbalanced<NegativeImbalanceOf<Self>>;

    /// The origin which may forcibly approve or revoke approval publisher. Root can always do this.
		type ApproveOrigin: EnsureOrigin<Self::Origin>;

    /// The origin which may forcibly unpublish service. Root can always do this.
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

  #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
    /// New published service.
    ServicePublished{ service_id: ServiceIndex },
    /// A service has been taken down.
    ServiceUnpublished{ service_id: ServiceIndex },
    /// New subscription to a service.
    ServiceSubscribed{ service_id: ServiceIndex, subscriber: T::AccountId },
    /// A subscription was cancelled.
    ServiceUnsubscribed{ service_id: ServiceIndex, subscriber: T::AccountId },
    /// A subscription has been renew.
    SubscriptionRenew{ service_id: ServiceIndex, subscriber: T::AccountId, expire_on: T::BlockNumber },
    /// An account has been requested for approved publisher
    RequestApprovedPublished{ account_id: T::AccountId },
    /// A publisher has been approved.
    PublisherApproved{ publisher: T::AccountId },
    /// A approved publisher was revoked.
    PublisherRovokeApproval{ publisher: T::AccountId },
  }

  #[pallet::error]
  pub enum Error<T> {
    ServiceAlreadyPublished,
    ServiceNotFound,
    ServiceAlreadyUnpublished,
    NotServicePublisher,
    // request publishing
    AlreadyRequestForApproval,
    NotRequestForApproval,
    AlreadyApprovedPublisher,
    AlreadyApprovedOrRequested,
    // publish service
    NotApprovedPublisher,
    NameTooLong,
    DescriptionTooLong,
    InsufficientPublisherBalance,
    // subscribe service
    AlreadySubscribed,
    SubscriptionNotFound,
    SubscriptionInactive,
    NotPeriodicService,
  }

  /// Number of service that have been published.
	#[pallet::storage]
	#[pallet::getter(fn service_count)]
	pub type ServiceCount<T: Config> = StorageValue<_, ServiceIndex, ValueQuery>;

  /// Services that have been published.
	#[pallet::storage]
	#[pallet::getter(fn services)]
	pub type Services<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ServiceIndex,
		Service<T::AccountId, BalanceOf<T>, T::BlockNumber, T::MaximumNameLength, T::MaximumContractLength>,
    OptionQuery
	>;

  /// The description of each service.
	#[pallet::storage]
	#[pallet::getter(fn service_descriptions)]
	pub type ServiceDescriptions<T: Config> =
		StorageMap<_, Twox64Concat, ServiceIndex, BoundedVec<u8, T::MaximumDescriptionLength>>;

  /// Subscription that has been established.
	#[pallet::storage]
	#[pallet::getter(fn subscriptions)]
	pub type Subscriptions<T: Config> = StorageDoubleMap<
    _, 
    Twox64Concat, 
    ServiceIndex, 
    Twox64Concat,
    T::AccountId, 
    Subscription<T::BlockNumber>, 
    OptionQuery
  >;

  /// This indicates whether an account is approved publisher or not
	#[pallet::storage]
	#[pallet::getter(fn approved_publisher)]
	pub type ApprovedPublisher<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, PublisherStatus, OptionQuery>;

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(<T as Config>::WeightInfo::request_approved_publisher())]
    pub fn request_approved_publisher(
      origin: OriginFor<T>
    ) -> DispatchResult {
      let sender = ensure_signed(origin)?;

      // Get status of the sender.
      match ApprovedPublisher::<T>::get(&sender) {
        None => {
          // set the status of the sender to requested approval
          ApprovedPublisher::<T>::insert(&sender, PublisherStatus::Requested);

          Self::deposit_event(Event::RequestApprovedPublished{ account_id: sender });
          Ok(())
        },
        Some(_) => Err(Error::<T>::AlreadyApprovedOrRequested)?
      }
    }

    #[pallet::weight(<T as Config>::WeightInfo::approve_publisher())]
    pub fn approve_publisher(
      origin: OriginFor<T>,
      account_id: T::AccountId
    ) -> DispatchResult {
      T::ApproveOrigin::ensure_origin(origin)?;

      // Get status of the sender.
      let publisher_status = ApprovedPublisher::<T>::get(&account_id).ok_or(Error::<T>::NotRequestForApproval)?;
      ensure!(publisher_status == PublisherStatus::Requested, Error::<T>::AlreadyApprovedPublisher);

      // set the status of the sender to requested approval
      ApprovedPublisher::<T>::insert(&account_id, PublisherStatus::Approved);

      Self::deposit_event(Event::PublisherApproved{ publisher: account_id });
			Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::revoke_publisher())]
    pub fn revoke_publisher(
      origin: OriginFor<T>,
      account_id: T::AccountId
    ) -> DispatchResult {
      T::ApproveOrigin::ensure_origin(origin)?;

      // Get status of the sender.
      let publisher_status = ApprovedPublisher::<T>::get(&account_id).ok_or(Error::<T>::NotRequestForApproval)?;
      ensure!(publisher_status == PublisherStatus::Approved, Error::<T>::NotApprovedPublisher);

      // remove the account from approved publisher.
      ApprovedPublisher::<T>::remove(&account_id);

      Self::deposit_event(Event::PublisherRovokeApproval{ publisher: account_id });
			Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::publish_service(name.len() as u32))]
    pub fn publish_service(
			origin: OriginFor<T>,
			cost: BalanceOf<T>,
			name: Vec<u8>,
      description: Vec<u8>,
		) -> DispatchResult {
      let publisher = ensure_signed(origin)?;
      ensure!(ApprovedPublisher::<T>::contains_key(&publisher), Error::<T>::NotApprovedPublisher);

      let bounded_name : BoundedVec<_, T::MaximumNameLength> = name.clone().try_into().map_err(|()| Error::<T>::NameTooLong)?;
      let bounded_description : BoundedVec<_, T::MaximumDescriptionLength> = description.clone().try_into().map_err(|()| Error::<T>::DescriptionTooLong)?;
      let service_id = Self::service_count();

      // reserve deposit for new service
      let bond = T::BaseDeposit::get() + T::DataDepositPerByte::get() * ((bounded_name.len() + bounded_description.len()) as u32).into();
      T::Currency::reserve(&publisher, bond).map_err(|_| Error::<T>::InsufficientPublisherBalance)?;
      ServiceCount::<T>::put(service_id + 1);

      let service = Service {
        publisher,
        cost,
        bond,
        name: bounded_name,
        contract: "https://contract-link".as_bytes().to_vec().try_into().unwrap(),
        maybe_periodic: None,
        status: ServiceStatus::Published{ on: <frame_system::Pallet<T>>::block_number() },
      };

      Services::<T>::insert(service_id, &service);
      ServiceDescriptions::<T>::insert(service_id, bounded_description);

      Self::deposit_event(Event::<T>::ServicePublished { service_id });
      Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::subscribe_service())]
    pub fn subscribe_service(
      origin: OriginFor<T>,
      service_id: ServiceIndex,
    ) -> DispatchResult {
      let subscriber = ensure_signed(origin)?;

      match Services::<T>::get(&service_id) {
        None => Err(Error::<T>::ServiceNotFound)?,
        Some(service) => {
          // ensure!(!Subscriptions::<T>::contains_key(&service_id, &subscriber), Error::<T>::AlreadySubscribed);

          if let Some(subscription) = Subscriptions::<T>::get(&service_id, &subscriber) {
            ensure!(!subscription.active, Error::<T>::AlreadySubscribed);
          }
          
          T::Currency::transfer(&subscriber, &service.publisher, service.cost, KeepAlive)?;

          let start_on = <frame_system::Pallet<T>>::block_number();
          let expire_on = if let Some(period) = service.maybe_periodic {
            Some(start_on + period.into())
          } else {
            None
          };

          let subscription = Subscription {
            start_on,
            expire_on,
            active: true,
          };
          Subscriptions::<T>::insert(&service_id, subscriber.clone(), subscription);

          Self::deposit_event(Event::<T>::ServiceSubscribed{ service_id, subscriber });
          Ok(())
        }
      }
    }

    #[pallet::weight(<T as Config>::WeightInfo::unsubscribe_service())]
    pub fn unsubscribe_service(
      origin: OriginFor<T>,
      service_id: ServiceIndex,
    ) -> DispatchResult {
      let subscriber = ensure_signed(origin)?;

      ensure!(Services::<T>::contains_key(&service_id), Error::<T>::ServiceNotFound);
      Subscriptions::<T>::try_mutate_exists(&service_id, subscriber.clone(), |maybe_sub| -> DispatchResult {
        let mut subscription = maybe_sub.as_mut().ok_or(Error::<T>::SubscriptionNotFound)?;

        ensure!(subscription.active, Error::<T>::SubscriptionInactive);
        subscription.active = false;

        Ok(())
      })?;

      Self::deposit_event(Event::<T>::ServiceUnsubscribed{ service_id, subscriber });
      Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::renew_subscription())]
    pub fn renew_subscription(
      origin: OriginFor<T>,
      service_id: ServiceIndex,
    ) -> DispatchResult {
      let subscriber = ensure_signed(origin)?;

      match Services::<T>::get(&service_id) {
        None => Err(Error::<T>::ServiceNotFound)?,
        Some(service) => match service.maybe_periodic {
          None => Err(Error::<T>::NotPeriodicService)?,
          Some(period) => {
            let start_on = <frame_system::Pallet<T>>::block_number();
            let expire_on = start_on + period.into();

            Subscriptions::<T>::try_mutate_exists(&service_id, subscriber.clone(), |maybe_sub| -> DispatchResult {
              let mut subscription = maybe_sub.as_mut().ok_or(Error::<T>::SubscriptionNotFound)?;
      
              ensure!(subscription.active, Error::<T>::SubscriptionInactive);
              subscription.expire_on = Some(expire_on);
  
              Ok(())
            })?;
            Self::deposit_event(Event::<T>::SubscriptionRenew{ service_id, subscriber, expire_on });
          }
        }
      }

      Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::unpublished_service())]
    pub fn unpublished_service(
      origin: OriginFor<T>,
      service_id: ServiceIndex,
    ) -> DispatchResult {
      let maybe_publisher = ensure_signed(origin.clone())
				.map(Some)
				.or_else(|_| T::ForceOrigin::ensure_origin(origin).map(|_| None))?;

      Services::<T>::try_mutate_exists(&service_id, |maybe_service| -> DispatchResult {
        let mut service = maybe_service.as_mut().ok_or(Error::<T>::ServiceNotFound)?;

        if let Some(publisher) = maybe_publisher {
          ensure!(publisher == service.publisher, Error::<T>::NotServicePublisher);
        }

        match service.status {
          ServiceStatus::Unpublished { .. } => return Err(Error::<T>::ServiceAlreadyUnpublished.into()),
          ServiceStatus::Published { .. } => {
            ServiceDescriptions::<T>::remove(&service_id);
            let imbalance = T::Currency::slash_reserved(&service.publisher, service.bond).0;
            T::OnSlash::on_unbalanced(imbalance);

            service.bond = Zero::zero();
            service.status = ServiceStatus::Unpublished{ on: <frame_system::Pallet<T>>::block_number() };

            Self::deposit_event(Event::<T>::ServiceUnpublished{ service_id });
            Ok(())
          }
        }
      })?;

      Ok(())
    }
  }
}
