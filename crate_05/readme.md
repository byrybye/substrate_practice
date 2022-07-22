程序代码

```#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

//  frame_support::pallet 宏
#[frame_support::pallet]
pub mod pallet {
	// import crate
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	// 引入需要的内容
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	//定义config
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	//定义pallet
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	//定义存储
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,                        // 存证的哈希值
		(T::AccountId, T::BlockNumber), // 值时两个元素的tuple，第一个是AccountId, 第二个存储区块高度。
	>;

	//定义事件
	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),                 //创建
		ClaimRevoked(T::AccountId, Vec<u8>),                 //撤销
		ClaimTransferd(T::AccountId, Vec<u8>, T::AccountId), //转移
	}

	//定义错误
	#[pallet::error]
	pub enum Error<T> {
		ClaimAlreadyExist, //已经存在
		ClaimNotExist,     //不存在
		NotClaimOwner,     //所有者错误
	}

	//hook 实现
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	//实现方法
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//创建
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			//获取操作人
			let sender = ensure_signed(origin)?;
			//验证数据不存在
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ClaimAlreadyExist);
			//插入
			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);
			//触发事件
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
		}

		//撤销
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			//验证操作人
			let sender = ensure_signed(origin)?;
			//获取claim的所有者
			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			//验证操作人是所有者
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			//删除
			Proofs::<T>::remove(&claim);
			//触发事件
			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			Ok(().into())
		}

		//转移
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
			receiver: T::AccountId,
		) -> DispatchResultWithPostInfo {
			//验证操作者
			let sender = ensure_signed(origin)?;
			//获取所有者
			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			//验证操作人是所有者
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			//修改
			Proofs::<T>::mutate(&claim, |v| {
				*v = Some((receiver.clone(), frame_system::Pallet::<T>::block_number()))
			});
			//触发时间
			Self::deposit_event(Event::ClaimTransferd(sender, claim, receiver));
			Ok(().into())
		}
	}
}


```
程序运行命令：
./node-template --dev --tmp

程序运行截图1
![image](https://github.com/byrybye/substrate_practice/blob/main/crate_05/screen1.png)

程序运行截图2
![image](https://github.com/byrybye/substrate_practice/blob/main/crate_05/screen2.png)

操作记录截图

创建
![image](https://github.com/byrybye/substrate_practice/blob/main/crate_05/create.png)

转移
![image](https://github.com/byrybye/substrate_practice/blob/main/crate_05/transfer.png)

撤销
![image](https://github.com/byrybye/substrate_practice/blob/main/crate_05/revoke.png)

操作日志（一次完整的操作记录从创建、转移到撤销）
![image](https://github.com/byrybye/substrate_practice/blob/main/crate_05/operation_log.png)

