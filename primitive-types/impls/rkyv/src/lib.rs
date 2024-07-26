pub use paste;

/// Add glue between `T` and rkyv `Archived<T>`
#[macro_export]
macro_rules! impl_transmute_rkyv {
	($name: ident) => {
		$crate::paste::paste! {
			impl [<Archived $name>] {
				/// Transmutes the archived type to the original type
				#[inline(always)]
				pub fn [<as_ $name:snake>](&self) -> &$name {
					unsafe { std::mem::transmute(self) }
				}

				/// Transmutes the archived type to the original type
				#[inline(always)]
				pub fn [<as_ $name:snake _mut>](&mut self) -> &mut $name {
					unsafe { std::mem::transmute(self) }
				}
			}

			impl AsRef<$name> for [<Archived $name>] {
				#[inline(always)]
				fn as_ref(&self) -> &$name {
					unsafe { std::mem::transmute(self) }
				}
            }

			impl AsMut<$name> for [<Archived $name>] {
				#[inline(always)]
				fn as_mut(&mut self) -> &mut $name {
					unsafe { std::mem::transmute(self) }
				}
            }
		}
	};
}

/// Safety test that ensures that the transmute_rkyv macro is working correctly
/// the size and alignment of the type must be the same as the archived type
#[macro_export]
macro_rules! test_transmute_rkyv {
	($name: ident) => {
		$crate::paste::paste! {
			#[test]
			fn [<test_transmute_rkyv_ $name:snake>]() {
				use ::rkyv::Archived;
				use ::std::mem::size_of;
				use ::std::mem::align_of;

				assert_eq!(size_of::<$name>(), size_of::<Archived<$name>>());
				assert_eq!(align_of::<$name>(), align_of::<Archived<$name>>());
			}
		}
	};
}