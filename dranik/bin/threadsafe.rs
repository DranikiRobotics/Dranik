//! Thread-safe values.
//!
//! This module contains thread-safe values that can be used in a
//! multi-threaded environment.

#![allow(unused)]

/// Macro for implementing [`Send`] and [`Sync`] for a struct.
macro_rules! thread_safe {
    ($struct: ident < $($generics: ident),* >) => {
        #[allow(unsafe_code)]
        unsafe impl<$($generics),*> Send for $struct<$($generics),*> {}
        #[allow(unsafe_code)]
        unsafe impl<$($generics),*> Sync for $struct<$($generics),*> {}
    };
    ($struct: ident) => {
        #[allow(unsafe_code)]
        unsafe impl Send for $struct {}
        #[allow(unsafe_code)]
        unsafe impl Sync for $struct {}
    };
}

mod holders {
    use std::sync::{Arc, Mutex, MutexGuard};

    const POISON_MESSAGE: &str = "Poisoned mutex";

    #[derive(Debug, Default)]
    pub struct ThreadSafeHolder<T>(Arc<Mutex<T>>);

    /// The Standard Result type retuned by the thread-safe holders.
    pub type StandardResult<T> = ::core::result::Result<T, &'static str>;

    /// The result type returned when borrowing from a thread-safe holder.
    pub type GetResult<'a, T> = StandardResult<SafeHeld<'a, T>>;
    /// The result type returned when mutably borrowing from a thread-safe holder.
    pub type GetResultMut<'a, T> = StandardResult<SafeHeldMut<'a, T>>;

    /// A Holder for immutable thread-safe values.
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct SafeHeld<'a, T>(MutexGuard<'a, T>);

    /// A Holder for mutable thread-safe values.
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct SafeHeldMut<'a, T>(MutexGuard<'a, T>);

    impl<'a, T> std::ops::Deref for SafeHeld<'a, T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl<'a, T> core::ops::Deref for SafeHeldMut<'a, T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl<'a, T> core::ops::DerefMut for SafeHeldMut<'a, T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.0
        }
    }

    impl<T> Clone for ThreadSafeHolder<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<'a, T: PartialEq> PartialEq for SafeHeld<'a, T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            *self.0 == *other.0
        }
    }

    impl<'a, T: PartialEq> PartialEq for SafeHeldMut<'a, T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            *self.0 == *other.0
        }
    }

    impl<T: PartialEq> PartialEq for ThreadSafeHolder<T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            match (self.get(), other.get()) {
                (Ok(a), Ok(b)) => a == b,
                _ => false,
            }
        }
    }

    impl<T> ThreadSafeHolder<T> {
        #[inline]
        pub fn new(value: T) -> Self {
            ThreadSafeHolder(Arc::new(Mutex::new(value)))
        }
        pub fn get(&self) -> GetResult<'_, T> {
            Ok(SafeHeld(self.0.lock().map_err(|_| POISON_MESSAGE)?))
        }
        pub fn get_mut(&self) -> GetResultMut<'_, T> {
            Ok(SafeHeldMut(self.0.lock().map_err(|_| POISON_MESSAGE)?))
        }
    }
}

thread_safe!(ThreadSafe<T>);

pub use holders::{GetResult, GetResultMut, SafeHeld, SafeHeldMut, StandardResult};

/// A thread-safe value.
///
/// This is a wrapper around a value that by default is not thread-safe.
pub type ThreadSafe<T> = holders::ThreadSafeHolder<T>;

/// A thread-safe error.
pub trait ThreadSafeError: std::error::Error + Send + Sync {}

impl<T: std::error::Error + Send + Sync> ThreadSafeError for T {}

macro_rules! thread_safe_primitive {
    (
        $(#[$outer:meta])*
        pub type $name:ident = $primitive:ty; { $thread_safe_mod_name:ident; $default_value:expr }
        $($rest:tt)*
    ) => (
        $(#[$outer])*
        pub type $name = $thread_safe_mod_name::$name;
        mod $thread_safe_mod_name {
            use super::*;

            #[repr(transparent)]
            #[derive(Default, Debug, Clone)]
            pub struct $name(ThreadSafe<Primitive>);

            thread_safe!($name);

            impl PartialEq for $name {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    match (self.get(), other.get() ) {
                        (Ok(a), Ok(b)) => a.get() == b.get(),
                        _ => false,
                    }
                }
            }

            impl Eq for $name {}

            impl PartialOrd for $name {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    match (self.get(), other.get() ) {
                        (Ok(a), Ok(b)) => a.get().partial_cmp(&b.get()),
                        _ => None,
                    }
                }
            }

            impl Ord for $name {
                #[inline]
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    match (self.get(), other.get() ) {
                        (Ok(a), Ok(b)) => a.get().cmp(&b.get()),
                        _ => std::cmp::Ordering::Equal,
                    }
                }
            }

            impl $name {
                #[inline]
                pub fn new(value: $primitive) -> Self {
                    Self(ThreadSafe::new(Primitive::new(value)))
                }
                #[inline]
                pub fn get(&self) -> holders::GetResult<'_, Primitive> {
                    self.0.get()
                }
                #[inline]
                pub fn get_mut(&self) -> holders::GetResultMut<'_, Primitive> {
                    self.0.get_mut()
                }
            }

            #[repr(transparent)]
            #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct Primitive($primitive);

            thread_safe!(Primitive);

            impl Primitive {
                #[inline]
                pub const fn new(value: $primitive) -> Self {
                    Self(value)
                }
                #[inline]
                pub fn get(&self) -> $primitive {
                    self.0
                }
                #[inline]
                pub fn set(&mut self, value: $primitive) {
                    self.0 = value;
                }
            }

            impl core::ops::Deref for Primitive {
                type Target = $primitive;
                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl core::ops::DerefMut for Primitive {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl From<$primitive> for Primitive {
                #[inline]
                fn from(value: $primitive) -> Self {
                    Self(value)
                }
            }

            impl From<Primitive> for $primitive {
                #[inline]
                fn from(value: Primitive) -> Self {
                    value.0
                }
            }

            impl From<$primitive> for $name {
                #[inline]
                fn from(value: $primitive) -> Self {
                    Self::new(value)
                }
            }

            impl From<$name> for $primitive {
                #[inline]
                fn from(value: $name) -> Self {
                    match value.get() {
                        Ok(b) => b.get(),
                        Err(_) => $default_value,
                    }
                }
            }
        }

        thread_safe_primitive!($($rest)*);
    );
    () => ();
}

thread_safe_primitive! {
    /// A thread-safe boolean.
    ///
    /// This is a wrapper around a [`bool`] that is thread-safe.
    ///
    /// [`bool`]: https://doc.rust-lang.org/std/primitive.bool.html
    pub type ThreadSafeBool = bool; {thread_safe_bool; false}

    /// A thread-safe 8-bit signed integer.
    ///
    /// This is a wrapper around a [`i8`] that is thread-safe.
    ///
    /// [`i8`]: https://doc.rust-lang.org/std/primitive.i8.html
    pub type ThreadSafeI8 = i8; {thread_safe_i8; 0}

    /// A thread-safe 8-bit unsigned integer.
    ///
    /// This is a wrapper around a [`u8`] that is thread-safe.
    ///
    /// [`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
    pub type ThreadSafeU8 = u8; {thread_safe_u8; 0}

    /// A thread-safe 16-bit signed integer.
    ///
    /// This is a wrapper around a [`i16`] that is thread-safe.
    ///
    /// [`i16`]: https://doc.rust-lang.org/std/primitive.i16.html
    pub type ThreadSafeI16 = i16; {thread_safe_i16; 0}

    /// A thread-safe 16-bit unsigned integer.
    ///
    /// This is a wrapper around a [`u16`] that is thread-safe.
    ///
    /// [`u16`]: https://doc.rust-lang.org/std/primitive.u16.html
    pub type ThreadSafeU16 = u16; {thread_safe_u16; 0}

    /// A thread-safe 32-bit signed integer.
    ///
    /// This is a wrapper around a [`i32`] that is thread-safe.
    ///
    /// [`i32`]: https://doc.rust-lang.org/std/primitive.i32.html
    pub type ThreadSafeI32 = i32; {thread_safe_i32; 0}

    /// A thread-safe 32-bit unsigned integer.
    ///
    /// This is a wrapper around a [`u32`] that is thread-safe.
    ///
    /// [`u32`]: https://doc.rust-lang.org/std/primitive.u32.html
    pub type ThreadSafeU32 = u32; {thread_safe_u32; 0}

    /// A thread-safe 64-bit signed integer.
    ///
    /// This is a wrapper around a [`i64`] that is thread-safe.
    ///
    /// [`i64`]: https://doc.rust-lang.org/std/primitive.i64.html
    pub type ThreadSafeI64 = i64; {thread_safe_i64; 0}

    /// A thread-safe 64-bit unsigned integer.
    ///
    /// This is a wrapper around a [`u64`] that is thread-safe.
    ///
    /// [`u64`]: https://doc.rust-lang.org/std/primitive.u64.html
    pub type ThreadSafeU64 = u64; {thread_safe_u64; 0}

    /// A thread-safe 128-bit signed integer.
    ///
    /// This is a wrapper around a [`i128`] that is thread-safe.
    ///
    /// [`i128`]: https://doc.rust-lang.org/std/primitive.i128.html
    pub type ThreadSafeI128 = i128; {thread_safe_i128; 0}

    /// A thread-safe 128-bit unsigned integer.
    ///
    /// This is a wrapper around a [`u128`] that is thread-safe.
    ///
    /// [`u128`]: https://doc.rust-lang.org/std/primitive.u128.html
    pub type ThreadSafeU128 = u128; {thread_safe_u128; 0}
}
