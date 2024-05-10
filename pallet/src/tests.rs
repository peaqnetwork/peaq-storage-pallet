use crate::{mock::*, Error};
use crate::{Config, MAX_STORAGE_ITEM_SIZE};
use frame_support::{assert_noop, assert_ok};

const ITEM_TYPE: &[u8; 8] = b"itemType";
const ITEM: &[u8; 4] = b"item";

pub(crate) const EXPECTED_DEPOSIT: Balance =
    (DEPOSIT_PER_BYTE * MAX_STORAGE_ITEM_SIZE as Balance) + DEPOSIT_BASE;

//Test to add an item
#[test]
fn add_item_test_ok() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));

        // correct storage deposit was deducted or not
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&ALICE),
            EXPECTED_DEPOSIT
        );
    });
}

//Test to add a duplicate item
#[test]
fn add_item_duplicate_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        //Add an item
        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));

        //Add the same item again
        assert_noop!(
            PeaqStorage::add_item(
                RuntimeOrigin::signed(ALICE),
                ITEM_TYPE.to_vec(),
                ITEM.to_vec()
            ),
            Error::<Test>::ItemTypeAlreadyExists
        );

        // correct storage deposit was deducted or not
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&ALICE),
            EXPECTED_DEPOSIT
        );
    });
}

//Test to add item with item type length exceed maximum limit
#[test]
fn add_item_type_length_exceeds_limit_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let invalid_item_typ = ITEM_TYPE.repeat(9);

        assert_noop!(
            PeaqStorage::add_item(
                RuntimeOrigin::signed(ALICE),
                invalid_item_typ.to_vec(),
                ITEM.to_vec()
            ),
            Error::<Test>::ItemTypeExceedMax64
        );

        // no deposit deducted
        assert_eq!(<Test as Config>::Currency::reserved_balance(&ALICE), 0);
    });
}

//Test to add an item with item length exceed maximum limit
#[test]
fn add_item_length_exceeds_limit_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let invalid_item = ITEM.repeat(66);

        assert_noop!(
            PeaqStorage::add_item(
                RuntimeOrigin::signed(ALICE),
                ITEM_TYPE.to_vec(),
                invalid_item.to_vec()
            ),
            Error::<Test>::ItemExceedMax256
        );

        // no deposit deducted
        assert_eq!(<Test as Config>::Currency::reserved_balance(&ALICE), 0);
    });
}

//Test to update an item
#[test]
fn update_item_test_ok() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        //Add an item
        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));

        //update item
        assert_ok!(PeaqStorage::update_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            b"new_item".to_vec()
        ));

        // correct storage deposit was deducted or not
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&ALICE),
            EXPECTED_DEPOSIT
        );
    });
}

//Test to update a non existing item
#[test]
fn update_non_existing_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_noop!(
            PeaqStorage::update_item(
                RuntimeOrigin::signed(ALICE),
                b"new_item_type".to_vec(),
                b"new_item".to_vec()
            ),
            Error::<Test>::ItemNotFound
        );
    });
}

#[test]
fn update_item_with_item_length_exceed_limit_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let invalid_item = ITEM.repeat(66);

        //Add an item
        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));

        //Update the item with item length exceed the limit
        assert_noop!(
            PeaqStorage::update_item(
                RuntimeOrigin::signed(ALICE),
                ITEM_TYPE.to_vec(),
                invalid_item.to_vec()
            ),
            Error::<Test>::ItemExceedMax256
        );

        // correct storage deposit was deducted or not
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&ALICE),
            EXPECTED_DEPOSIT
        );
    });
}

#[test]
//Test to update an other owner's item
fn update_other_owner_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        //Add an item with user Iredia
        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));
        // correct storage deposit was deducted or not
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&ALICE),
            EXPECTED_DEPOSIT
        );
        //Update the item with fake owner
        assert_noop!(
            PeaqStorage::update_item(
                RuntimeOrigin::signed(BOB),
                ITEM_TYPE.to_vec(),
                ITEM.to_vec()
            ),
            Error::<Test>::ItemNotFound
        );
        // correct storage deposit was deducted or not
        assert_eq!(<Test as Config>::Currency::reserved_balance(&BOB), 0);
    });
}

//Test to get an item
#[test]
fn get_item_test_ok() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        //Add an item
        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));

        //Get the same item
        assert_ok!(PeaqStorage::get_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec()
        ));
    });
}

//Test to get a non existing item
#[test]
fn get_non_existing_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));

        //Get a non existing item
        assert_noop!(
            PeaqStorage::get_item(RuntimeOrigin::signed(ALICE), b"new_item_type".to_vec()),
            Error::<Test>::ItemNotFound
        );
    });
}

//Test to get another owner item
#[test]
fn get_other_owner_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        //Add an item
        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));

        //Get anotehr owner's item
        assert_noop!(
            PeaqStorage::get_item(RuntimeOrigin::signed(BOB), ITEM_TYPE.to_vec()),
            Error::<Test>::ItemNotFound
        );
    });
}

#[test]
fn remove_item_is_ok() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        //Add an item
        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&ALICE),
            EXPECTED_DEPOSIT
        );

        //Remove the item
        assert_ok!(PeaqStorage::remove_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec()
        ));

        assert_eq!(<Test as Config>::Currency::reserved_balance(&ALICE), 0);
    });
}

#[test]
fn remove_non_existing_item() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        //Remove the item
        assert_noop!(
            PeaqStorage::remove_item(RuntimeOrigin::signed(ALICE), ITEM_TYPE.to_vec()),
            Error::<Test>::ItemNotFound
        );
        assert_eq!(<Test as Config>::Currency::reserved_balance(&ALICE), 0);
    });
}

#[test]
fn remove_someone_elses_item() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        //Add an item
        assert_ok!(PeaqStorage::add_item(
            RuntimeOrigin::signed(ALICE),
            ITEM_TYPE.to_vec(),
            ITEM.to_vec()
        ));
        assert_eq!(
            <Test as Config>::Currency::reserved_balance(&ALICE),
            EXPECTED_DEPOSIT
        );

        //Remove the item
        assert_noop!(
            PeaqStorage::remove_item(RuntimeOrigin::signed(BOB), ITEM_TYPE.to_vec()),
            Error::<Test>::ItemNotFound
        );
        assert_eq!(<Test as Config>::Currency::reserved_balance(&BOB), 0);
    });
}
