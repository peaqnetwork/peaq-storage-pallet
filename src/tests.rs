use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_item_test(){
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        let acct="Iredia";
        let origin = account_key(acct);
        let item_type = b"itemType";
        let item = b"item";
        let mut tmp = String::from("a").repeat(65);            

        //test to add an item
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       

        //test to add duplicate  item
        assert_noop!(
            PeaqStorage::add_item(
            Origin::signed(origin),
            item_type.to_vec(),
            item.to_vec()            
        ),
        Error::<Test>::ItemTypeAlreadyExists
    );
    
    let item_type = tmp.as_bytes();    
    //Test for item type with item type's length exceed maximum limit
    assert_noop!(
        PeaqStorage::add_item(
            Origin::signed(origin), 
            item_type.to_vec(), 
            item.to_vec()
        ),
        Error::<Test>::ItemTypeExceedMax64
    );

    let item_type = b"itemType1";  
    tmp = tmp.repeat(2);  
    let item= tmp.as_bytes();

    //Test for item with item's length exceed maximum limit
    assert_noop!(
        PeaqStorage::add_item(
            Origin::signed(origin), 
            item_type.to_vec(), 
            item.to_vec()
        ),
        Error::<Test>::ItemExceedMax128
    );     
        
    });
}

#[test]
fn update_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);     

        let acct="Iredia";
        let acct2 = "Fake";
        let origin = account_key(acct);
        let fake_origin = account_key(acct2);
        let item_type = b"itemType";
        let item = b"item";
        let tmp = String::from("a").repeat(129);

        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       

        //Test to update an item
        assert_ok!(
            PeaqStorage::update_item(
                Origin::signed(origin), 
                item_type.to_vec(), 
                item.to_vec()
            )
        );

        //Test to update a not exisitngs item
        assert_noop!(
            PeaqStorage::update_item(
                Origin::signed(origin), 
                b"new_item_type".to_vec(),
                item.to_vec()
            ),
            Error::<Test>::ItemNotFound

        );

        //Test to update another owner's item
        assert_noop!(
            PeaqStorage::update_item(
                Origin::signed(fake_origin), 
                item_type.to_vec(), 
                item.to_vec()
            ),
            Error::<Test>::ItemNotFound
        );

        let item=tmp.as_bytes();

        //Test to update an item with an invalid item
        assert_noop!(
            PeaqStorage::update_item(
                Origin::signed(origin),
                item_type.to_vec(), 
                item.to_vec()
            ),
            Error::<Test>::ItemExceedMax128

        );        

    });
}

#[test]
fn get_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);     

        let acct="Iredia";
        let acct2 = "fake";
        let origin = account_key(acct);
        let fake_origin = account_key(acct2);
        let item_type = b"itemType";
        let item = b"item";
        
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       

        //Test to get an item
        assert_ok!(
            PeaqStorage::get_item(
                Origin::signed(origin),
                item_type.to_vec()                
            )

        );
        
        //Test to get a non existing item
        assert_noop!(
            PeaqStorage::get_item(
                Origin::signed(origin),
                b"new_item_type".to_vec()
        ),
        Error::<Test>::ItemNotFound
        );

        //Test to get anotehr owner's item
        assert_noop!(
            PeaqStorage::get_item(
                Origin::signed(fake_origin),
                item_type.to_vec()                                
            ),
            Error::<Test>::ItemNotFound

        );
        
    });
}
