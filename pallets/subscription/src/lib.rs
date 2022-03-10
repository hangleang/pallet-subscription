#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;
mod tests;

pub use pallet::*;
use sp_std::prelude::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use scale_info::TypeInfo;
use sp_std::fmt::Debug;
use sp_runtime::{
	DispatchResult, RuntimeDebug, traits::{Zero, BadOrigin},
};
use frame_support::{
  BoundedVec, CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound,
  traits::{Currency, ReservableCurrency, OnUnbalanced, ExistenceRequirement::AllowDeath}
};
use weights::WeightInfo;
use codec::{Decode, Encode, MaxEncodedLen};

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

// type BalanceOf<T> = pallet_treasury::BalanceOf<T>;
// type PositiveImbalanceOf<T> = pallet_treasury::PositiveImbalanceOf<T>;

/// An index of a service. Just a `u32`.
pub type ServiceIndex = u32;

/// A service info.
#[derive(Encode, Decode, CloneNoBound, PartialEqNoBound, Eq, TypeInfo, MaxEncodedLen, RuntimeDebugNoBound)]
#[codec(mel_bound())]
// #[cfg_attr(test, derive(frame_support::DefaultNoBound))]
#[scale_info(skip_type_params(MaximumNameLength, MaximumContractLength))]
pub struct Service<
  AccountId: Encode + Decode + MaxEncodedLen + Clone + Debug + Eq + PartialEq, 
  Balance: Encode + Decode + MaxEncodedLen + Clone + Debug + Eq + PartialEq, 
  BlockNumber: MaxEncodedLen + Clone + Copy + Debug + Eq + PartialEq, 
  MaximumNameLength: Get<u32>, 
  MaximumContractLength: Get<u32>
> {
  /// The account publishing it.
	publisher: AccountId,
  /// The name of this service
  name: BoundedVec<u8, MaximumNameLength>,
  /// The amount that need be paid to publisher once subscribe.
	fee: Balance,
  /// The amount held on deposit (reserved) for making this proposal.
	bond: Balance,
  /// The link to contract 
  contract: BoundedVec<u8, MaximumContractLength>,
  /// If the service is periodic, then this points to the information concerning that.
	maybe_periodic: Option<u32>,
  /// Subscriber may need approval or not, in order to subscribe to the service.
  need_approval: bool,
  /// The status of this service.
  status: ServiceStatus<BlockNumber>,
}

// impl<
//   AccountId: PartialEq + Clone + Ord, 
//   Balance, 
//   BlockNumber: Clone, 
//   MaximumNameLength: Get<u32>, 
//   MaximumContractLength: Get<u32>
// >
// 	Service<AccountId, Balance, BlockNumber, MaximumNameLength, MaximumContractLength>
// {
// 	/// Getter for service status, to be used for child bounties.
// 	pub fn get_status(&self) -> ServiceStatus<BlockNumber> {
// 		self.status.clone()
// 	}
// }

/// The status of a subscription service.
#[derive(Encode, Decode, CloneNoBound, PartialEqNoBound, Eq, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
pub enum ServiceStatus<BlockNumber: MaxEncodedLen + Copy + Clone + Debug + Eq + PartialEq> {
  /// The service has been proposed.
  Proposed,
  /// The proposed service has been approved by ApproveOrigin.
  Published,
  /// The publisher's service requested to unpublish the service, all the active payments need to settle back to subscribers.
  PendingUnpublished {
    /// The service can be completely unpublish at this block.
    settle_due: BlockNumber
  },
  /// All the active payments has been settled back to subscribers.
  PaymentSettled {
    /// The service can be completely unpublish at this block.
    settle_due: BlockNumber
  },
  /// The service has been unpublished.
  Unpublished,
}

/// A subscription info.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Subscription<BlockNumber> {
  /// subscription on block
	start_on: BlockNumber,
  /// subscription expire on block
	expire_on: Option<BlockNumber>,
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

    /// The amount held on deposit to be a publisher
    type PublisherDeposit: Get<BalanceOf<Self>>;

    /// The amount held on deposit for a publish service
		#[pallet::constant]
		type BaseDeposit: Get<BalanceOf<Self>>;

    /// The amount held on deposit per byte within the tip report reason or bounty description.
		#[pallet::constant]
		type DataDepositPerByte: Get<BalanceOf<Self>>;

    /// Payment settlement duration in blocks.
		#[pallet::constant]
		type PaymentSettlePeriod: Get<Self::BlockNumber>;

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

    /// The maximum amount of subscriber per service.
    #[pallet::constant]
    type MaxSubscriber: Get<u32>;

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
		type RejectOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

  #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
    /// New published service.
    ServiceProposed{ service_id: ServiceIndex },
    /// A service has been taken down.
    ServiceUnpublished{ service_id: ServiceIndex },
    /// New subscription to a service.
    ServiceSubscribed{ service_id: ServiceIndex, subscriber: T::AccountId },
    /// A subscription was cancelled.
    ServiceUnsubscribed{ service_id: ServiceIndex, subscriber: T::AccountId },
    /// A subscription has been renew.
    SubscriptionRenew{ service_id: ServiceIndex, subscriber: T::AccountId, expire_on: T::BlockNumber },
    /// A subscription has been requested.
    SubscriptionRequested{ service_id: ServiceIndex, subscriber: T::AccountId},
    /// An account has been requested for approved publisher
    RequestApprovedPublished{ account_id: T::AccountId },
    /// A publisher has been approved.
    PublisherApproved{ publisher: T::AccountId },
    /// A publisher has been rejected.
    PublisherRejected{ publisher: T::AccountId },
    /// A approved publisher was revoked.
    PublisherRovokeApproval{ publisher: T::AccountId },
    /// A publisher's service has been requested to unpublish the service.
    UnpublishServiceRequested{ service_id: ServiceIndex },
    /// The payments has been settled to selected subscribers, if `selected` is empty, which means all.
    PaymentSettled{ service_id: ServiceIndex, selected: Vec<T::AccountId> },
  }

  #[pallet::error]
  pub enum Error<T> {
    ServiceAlreadyPublished,
    ServiceNotFound,
    NotServicePublisher,
    UnexpectedStatus,
    TooManyQueued,
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
    InsufficientSubscriberBalance,
    SettleDueReached,
    PaymentNotSettled,
    Premature,
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

  /// List of AccountId has been subscribe to the service
  #[pallet::storage]
  #[pallet::getter(fn service_subscribers)]
  pub type ServiceSubscribers<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ServiceIndex,
		BoundedVec<T::AccountId, T::MaxSubscriber>,
    ValueQuery
	>;

  /// This indicates whether a subscription is requested by subscriber or not.
	#[pallet::storage]
	#[pallet::getter(fn requested_subscription)]
	pub type RequestedSubscription<T: Config> = StorageDoubleMap<
    _, 
    Twox64Concat, 
    ServiceIndex, 
    Twox64Concat,
    T::AccountId, 
    bool, 
    ValueQuery
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
          T::Currency::reserve(&sender, T::PublisherDeposit::get()).map_err(|_| Error::<T>::InsufficientPublisherBalance)?;

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
    pub fn reject_publisher(
      origin: OriginFor<T>,
      account_id: T::AccountId
    ) -> DispatchResult {
      T::RejectOrigin::ensure_origin(origin)?;

      // Get status of the sender.
      let publisher_status = ApprovedPublisher::<T>::get(&account_id).ok_or(Error::<T>::NotRequestForApproval)?;
      ensure!(publisher_status == PublisherStatus::Requested, Error::<T>::AlreadyApprovedPublisher);

      T::Currency::unreserve(&account_id, T::PublisherDeposit::get());
      // remove the account from approved publisher.
      ApprovedPublisher::<T>::remove(&account_id);

      Self::deposit_event(Event::PublisherRovokeApproval{ publisher: account_id });
			Ok(())
    }    

    #[pallet::weight(<T as Config>::WeightInfo::revoke_publisher())]
    pub fn revoke_publisher(
      origin: OriginFor<T>,
      account_id: T::AccountId
    ) -> DispatchResult {
      T::RejectOrigin::ensure_origin(origin)?;

      // Get status of the sender.
      let publisher_status = ApprovedPublisher::<T>::get(&account_id).ok_or(Error::<T>::NotRequestForApproval)?;
      ensure!(publisher_status == PublisherStatus::Approved, Error::<T>::NotApprovedPublisher);

      let imbalance = T::Currency::slash_reserved(&account_id, T::PublisherDeposit::get()).0;
      T::OnSlash::on_unbalanced(imbalance);
      // remove the account from approved publisher.
      ApprovedPublisher::<T>::remove(&account_id);

      Self::deposit_event(Event::PublisherRovokeApproval{ publisher: account_id });
			Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::propose_service(name.len() as u32))]
    pub fn propose_service(
			origin: OriginFor<T>,
			name: Vec<u8>,
      description: Vec<u8>,
      need_approval: bool,
			fee: BalanceOf<T>,
      maybe_periodic: Option<u32>,
		) -> DispatchResult {
      let publisher = ensure_signed(origin)?;

      let publisher_status = ApprovedPublisher::<T>::get(&publisher).ok_or(Error::<T>::NotRequestForApproval)?;
      ensure!(publisher_status == PublisherStatus::Approved, Error::<T>::NotApprovedPublisher);

      let bounded_name : BoundedVec<_, T::MaximumNameLength> = name.clone().try_into().map_err(|()| Error::<T>::NameTooLong)?;
      let bounded_description : BoundedVec<_, T::MaximumDescriptionLength> = description.clone().try_into().map_err(|()| Error::<T>::DescriptionTooLong)?;
      let service_id = Self::service_count();

      // reserve deposit for new service
      let bond = T::BaseDeposit::get() + T::DataDepositPerByte::get() * ((bounded_name.len() + bounded_description.len()) as u32).into();
      T::Currency::reserve(&publisher, bond).map_err(|_| Error::<T>::InsufficientPublisherBalance)?;
      ServiceCount::<T>::put(service_id + 1);

      let service = Service {
        publisher,
        fee,
        bond,
        name: bounded_name,
        contract: b"https://contract-link".to_vec().try_into().unwrap(),
        maybe_periodic,
        need_approval,
        status: ServiceStatus::Proposed,
      };

      Services::<T>::insert(service_id, &service);
      ServiceDescriptions::<T>::insert(service_id, bounded_description);

      Self::deposit_event(Event::<T>::ServiceProposed { service_id });
      Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::approve_service())]
    pub fn approve_service(
      origin: OriginFor<T>,
      service_id: ServiceIndex,
    ) -> DispatchResult {
      T::ApproveOrigin::ensure_origin(origin)?;

      Services::<T>::try_mutate_exists(service_id, |maybe_service| -> DispatchResult {
				let mut service = maybe_service.as_mut().ok_or(Error::<T>::ServiceNotFound)?;
				ensure!(service.status == ServiceStatus::Proposed, Error::<T>::UnexpectedStatus);

				service.status = ServiceStatus::Published;
				Ok(())
			})?;
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
          ensure!(service.status == ServiceStatus::Published, Error::<T>::UnexpectedStatus);
          ensure!(!Subscriptions::<T>::contains_key(&service_id, &subscriber), Error::<T>::AlreadySubscribed);

          if service.need_approval {
            RequestedSubscription::<T>::insert(&service_id, subscriber.clone(), true);

            Self::deposit_event(Event::<T>::SubscriptionRequested{ service_id, subscriber });
          } else {
            T::Currency::transfer(&subscriber, &service.publisher, service.fee, AllowDeath).map_err(|_| Error::<T>::InsufficientSubscriberBalance)?;

            let start_on = <frame_system::Pallet<T>>::block_number();
            let expire_on = if let Some(period) = service.maybe_periodic {
              Some(start_on + period.into())
            } else {
              None
            };
  
            let subscription = Subscription {
              start_on,
              expire_on,
            };
            Subscriptions::<T>::insert(&service_id, subscriber.clone(), subscription);
            ServiceSubscribers::<T>::try_mutate(&service_id, |subscribers| -> DispatchResult {
              subscribers.try_push(subscriber.clone()).map_err(|()| Error::<T>::TooManyQueued)?;

              Ok(())
            })?;
  
            Self::deposit_event(Event::<T>::ServiceSubscribed{ service_id, subscriber });
          }

          Ok(())
        }
      }
    }

    #[pallet::weight(<T as Config>::WeightInfo::subscribe_service())]
    pub fn approve_subscription(
      origin: OriginFor<T>,
      service_id: ServiceIndex,
      account_id: T::AccountId,
    ) -> DispatchResult {
      let publisher = ensure_signed(origin)?;

      match Services::<T>::get(&service_id) {
        None => return Err(Error::<T>::ServiceNotFound.into()),
        Some(service) => {
          ensure!(service.status == ServiceStatus::Published, Error::<T>::UnexpectedStatus);
          ensure!(service.publisher == publisher, Error::<T>::NotServicePublisher);
          ensure!(service.need_approval, Error::<T>::UnexpectedStatus);

          ensure!(!Subscriptions::<T>::contains_key(&service_id, &account_id), Error::<T>::AlreadySubscribed);
          ensure!(RequestedSubscription::<T>::contains_key(&service_id, &account_id), Error::<T>::NotRequestForApproval);

          T::Currency::transfer(&account_id, &publisher, service.fee, AllowDeath).map_err(|_| Error::<T>::InsufficientSubscriberBalance)?;

          let start_on = <frame_system::Pallet<T>>::block_number();
          let expire_on = if let Some(period) = service.maybe_periodic {
            Some(start_on + period.into())
          } else {
            None
          };

          let subscription = Subscription {
            start_on,
            expire_on,
          };

          RequestedSubscription::<T>::remove(&service_id, &account_id);
          Subscriptions::<T>::insert(&service_id, account_id.clone(), subscription);
          ServiceSubscribers::<T>::try_mutate(&service_id, |subscribers| -> DispatchResult {
            subscribers.try_push(account_id.clone()).map_err(|()| Error::<T>::TooManyQueued)?;

            Ok(())
          })?;

          Self::deposit_event(Event::<T>::ServiceSubscribed{ service_id, subscriber: account_id });
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

      match Services::<T>::get(&service_id) {
        None => return Err(Error::<T>::ServiceNotFound.into()),
        Some(service) => {
          ensure!(service.status == ServiceStatus::Published, Error::<T>::UnexpectedStatus);
          ensure!(Subscriptions::<T>::contains_key(&service_id, subscriber.clone()), Error::<T>::SubscriptionNotFound);

          Subscriptions::<T>::remove(&service_id, subscriber.clone());
          ServiceSubscribers::<T>::mutate(&service_id,  |subscribers| {
            subscribers.retain(|sub| *sub != subscriber.clone());
          });
    
          Self::deposit_event(Event::<T>::ServiceUnsubscribed{ service_id, subscriber });
          Ok(())
        }
      }
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
            T::Currency::transfer(&subscriber, &service.publisher, service.fee, AllowDeath).map_err(|_| Error::<T>::InsufficientSubscriberBalance)?;

            let start_on = <frame_system::Pallet<T>>::block_number();
            let expire_on = start_on + period.into();

            Subscriptions::<T>::try_mutate_exists(&service_id, subscriber.clone(), |maybe_sub| -> DispatchResult {
              let mut subscription = maybe_sub.as_mut().ok_or(Error::<T>::SubscriptionNotFound)?;
              subscription.expire_on = Some(expire_on);
  
              Ok(())
            })?;

            Self::deposit_event(Event::<T>::SubscriptionRenew{ service_id, subscriber, expire_on });
            Ok(())
          }
        }
      }
    }

    #[pallet::weight(<T as Config>::WeightInfo::unpublish_service())]
    pub fn request_unpublish_service(
      origin: OriginFor<T>,
      service_id: ServiceIndex
    ) -> DispatchResult {
      let publisher = ensure_signed(origin)?;

      Services::<T>::try_mutate_exists(&service_id, |maybe_service| -> DispatchResult {
        let mut service = maybe_service.as_mut().ok_or(Error::<T>::ServiceNotFound)?;
        ensure!(publisher == service.publisher, Error::<T>::NotServicePublisher);

        match service.status {
          ServiceStatus::Published => {
            let settle_due = <frame_system::Pallet<T>>::block_number() + T::PaymentSettlePeriod::get();
            service.status = ServiceStatus::PendingUnpublished{ settle_due };

            Ok(())
          },
          _ => return Err(Error::<T>::UnexpectedStatus.into())
        }
      })?;

      Self::deposit_event(Event::<T>::UnpublishServiceRequested{ service_id });
      Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::unpublish_service())]
    pub fn settle_payments(
      origin: OriginFor<T>,
      service_id: ServiceIndex,
      selected: Vec<T::AccountId>,
    ) -> DispatchResult {
      let publisher = ensure_signed(origin)?;

      Services::<T>::try_mutate_exists(&service_id, |maybe_service| -> DispatchResult {
        let mut service = maybe_service.as_mut().ok_or(Error::<T>::ServiceNotFound)?;

        match service.status {
          ServiceStatus::PendingUnpublished { settle_due } => {
            ensure!(publisher == service.publisher, Error::<T>::NotServicePublisher);
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block < settle_due, Error::<T>::SettleDueReached);

            let mut inactive_subscribers: Vec<T::AccountId> = Vec::new();
            ServiceSubscribers::<T>::mutate(&service_id, |v| {
              v.retain(|sub| {
                let mut keep = true;

                if selected.is_empty() || selected.contains(&sub.clone()) {
                  keep = false;
                } else {
                  match Subscriptions::<T>::get(&service_id, sub.clone()) {
                    Some(subscription) => match subscription.expire_on {
                      Some(expire_on) => keep = current_block < expire_on,
                      None => keep = false,
                    },
                    None => {}
                  }
                }

                if !keep {
                  inactive_subscribers.push(sub.clone());
                }
                keep
              });
            });

            if ServiceSubscribers::<T>::get(&service_id).is_empty() {
              service.status = ServiceStatus::PaymentSettled{ settle_due };
            }

            inactive_subscribers.iter().try_for_each(|sub| {
              Subscriptions::<T>::try_mutate_exists(&service_id, sub.clone(), |maybe_sub| -> DispatchResult {
                let subscription = maybe_sub.take().ok_or(Error::<T>::SubscriptionNotFound)?;

                if let Some(expire_on) = subscription.expire_on {
                  if current_block < expire_on {
                    T::Currency::transfer(&publisher, &sub, service.fee, AllowDeath).map_err(|_| Error::<T>::InsufficientPublisherBalance)?;
                  }
                } else {
                  T::Currency::transfer(&publisher, &sub, service.fee, AllowDeath).map_err(|_| Error::<T>::InsufficientPublisherBalance)?;
                }

                *maybe_sub = None;
                Ok(())
              })
            })
          },
          _ => Err(Error::<T>::UnexpectedStatus)?
        }
      })?;

      Self::deposit_event(Event::<T>::PaymentSettled{ service_id, selected });
      Ok(())
    }

    #[pallet::weight(<T as Config>::WeightInfo::unpublish_service())]
    pub fn unpublish_service(
      origin: OriginFor<T>,
      service_id: ServiceIndex,
    ) -> DispatchResult {
      let maybe_publisher = ensure_signed(origin.clone())
				.map(Some)
				.or_else(|_| T::RejectOrigin::ensure_origin(origin).map(|_| None))?;

      Services::<T>::try_mutate_exists(&service_id, |maybe_service| -> DispatchResult {
        let mut service = maybe_service.as_mut().ok_or(Error::<T>::ServiceNotFound)?;

        let slash_publisher = |publisher: &T::AccountId, publisher_deposit: &mut BalanceOf<T>| {
					let imbalance = T::Currency::slash_reserved(publisher, *publisher_deposit).0;
					T::OnSlash::on_unbalanced(imbalance);
          *publisher_deposit = Zero::zero();
				};

        match service.status {
          ServiceStatus::PaymentSettled{ settle_due } => {
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block >= settle_due, Error::<T>::Premature);
            ensure!(maybe_publisher.map_or(true, |sender| sender == service.publisher), Error::<T>::NotServicePublisher);
            ensure!(ServiceSubscribers::<T>::get(&service_id).len() == 0, Error::<T>::PaymentNotSettled);
            
            let err_amount = T::Currency::unreserve(&service.publisher, service.bond);
            debug_assert!(err_amount.is_zero());
          },
          _ => {
            ensure!(maybe_publisher.is_none(), BadOrigin);
            slash_publisher(&service.publisher, &mut service.bond)
          },
        }

        service.status = ServiceStatus::Unpublished;
        Ok(())
      })?;

      // ServiceDescriptions::<T>::remove(&service_id);
      Self::deposit_event(Event::<T>::ServiceUnpublished{ service_id });
      Ok(())
    }
  }
}