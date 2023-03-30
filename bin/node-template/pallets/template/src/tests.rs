use crate::{mock::*, Error, Event};
use codec::{Decode, Encode};
use frame_support::bounded_vec;
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use frame_support::{assert_noop, assert_ok};

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Go past genesis block so events get deposited
// 		System::set_block_number(1);
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(TemplateModule::something(), Some(42));
// 		// Assert that the correct event was deposited
// 		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
// 	});
// }

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }

#[test]
fn encode_decode_works() {
	let bounded: BoundedVec<u32, ConstU32<7>> = bounded_vec![1, 2, 3, 4, 5, 6];
	let normal: Vec<u32> = vec![1, 2, 3, 4, 5, 6];

	let bounded_encoded = bounded.encode();
	let normal_encoded = normal.encode();
	// let decoded: BoundedVec<u32, ConstU32<7>> = Decode::decode(&mut &encoded[..]).unwrap();

	assert_eq!(bounded_encoded, normal_encoded);
}

#[test]
fn do_something_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(TemplateModule::do_something(
			RuntimeOrigin::signed(1),
			bounded_vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
		));
	})
}

#[test]
fn do_something() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(TemplateModule::do_something(
			RuntimeOrigin::signed(1),
			bounded_vec![1, 2, 2, 2, 2, 2, 2, 2]
		));
	})
}
