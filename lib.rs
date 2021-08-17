#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod counter {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Counter {
        /// Stores a single 32 bit signed integer value on the storage.
        value: i32,
    }

    impl Counter {
        /// Constructor that initializes the value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the value to 0(the default for i32).
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one increments the stored value by one.
        /// Saturating addition is used to prevent overflow.
        #[ink(message)]
        pub fn increment(&mut self) {
            self.value = self.value.saturating_add(1);
        }

        /// Decrements the stored value by one.
        /// Saturating substraction is used to prevent underflow.
        #[ink(message)]
        pub fn decrement(&mut self) {
            self.value = self.value.saturating_sub(1);
        }

        /// Modifies stored value by a provided to a call.
        /// Saturating addition is used to prevent overflow and underflow.
        #[ink(message)]
        pub fn modify_by(&mut self, by: i32) {
            self.value = self.value.saturating_add(by);
        }

        /// Simply returns the current value in the storage.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test the value after a default constructor instantiation
        #[ink::test]
        fn default_works() {
            let counter = Counter::default();
            assert_eq!(counter.get(), 0);
        }

        #[ink::test]
        fn increment_works() {
            let mut counter = Counter::new(42);
            counter.increment();
            assert_eq!(counter.get(), 43);
        }

        #[ink::test]
        fn increment_does_not_overflow() {
            let mut counter = Counter::new(i32::MAX);
            counter.increment();
            assert_eq!(counter.get(), i32::MAX);
        }

        #[ink::test]
        fn decrement_works() {
            let mut counter = Counter::new(42);
            counter.decrement();
            assert_eq!(counter.get(), 41);
        }

        #[ink::test]
        fn decrement_does_not_overflow() {
            let mut counter = Counter::new(i32::MIN);
            counter.decrement();
            assert_eq!(counter.get(), i32::MIN);
        }

        #[ink::test]
        fn modify_by_works() {
            let mut counter = Counter::new(42);
            counter.modify_by(10);
            assert_eq!(counter.get(), 52);
        }

        #[ink::test]
        fn modify_by_does_not_underflow() {
            let mut counter = Counter::new(-42);
            counter.modify_by(i32::MIN);
            assert_eq!(counter.get(), i32::MIN);
        }
    }
}
