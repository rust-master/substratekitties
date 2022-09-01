use support::{decl_storage, decl_module, dispatch::Result, StorageValue};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Declare storage and getter functions here
        MyU32: u32;
        MyBool get(my_bool_getter): bool;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare public functions here

        fn my_function(origin, input_bool: bool) -> Result {
            let _sender = ensure_signed(origin)?;

            <MyBool<T>>::put(input_bool);
            
            Ok(())
        }
    }
}
