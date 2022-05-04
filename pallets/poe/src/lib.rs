// 两种编译形式
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

/// 存证模块
#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*, StorageMap, Blake2_256, Blake2_128Concat
    };
	use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    //配置
    #[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    //存储
    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    // 存储单元
	#[pallet::storage]
	#[pallet::getter(fn Proofs)]
	pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId,T::BlockNumber)
    >;

    //存证操作相关的事件
    #[pallet::event]
    #[pallet::metadata(T::AccountId="AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        ClaimCreat(T::AccountId, Vec<u8>),
        ClaimRevoke(T::AccountId, Vec<u8>),
	}

	// 异常类型
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimNotExist,
        NotClaimOwner,
	}

    // 类似生命周期环绕函数的调用
    #[pallet::hooks]
    impl <T: Config> Hooks<BlockNumber<T>> for Pallet<T> {
        
    }

    //可调用函数
    #[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn creat_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            
            let who = ensure_signed(origin)?;
            ensure!(!Proof::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            // (T::AccountId,T::BlockNumber)
            Proofs::<T>::insert(&claim, (who.clone(),frame_system::Pallet::<T>::block_number()));
        
            // 发送事件
			Self::deposit_event(Event::ClaimCreat(who, &claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            
            let who = ensure_signed(origin)?;
            ensure!(!Proof::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);


            let (owner,_) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
        
            //确定所有权
            ensure!(owner==who, Error::<T>::NotClaimOwner);

            //撤销存证
            Proofs::<T>::remove(&claim);

            // 发送事件
			Self::deposit_event(Event::ClaimRevoke(who, claim));

            Ok(().into())
        }
            
    }
}

