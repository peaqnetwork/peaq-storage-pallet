use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

//Test to add an item
#[test]
fn add_item_test_ok(){
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        let acct="Iredia";
        let origin = account_key(acct);
        let item_type = b"itemType";
        let item = b"item";        
        
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       
        
    });
}

//Test to add a duplicate item
#[test]
fn add_item_duplicate_test(){
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        let acct="Iredia";
        let origin = account_key(acct);
        let item_type = b"itemType";
        let item = b"item";
        
        //Add an item
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       

        //Add the same item again
        assert_noop!(
            PeaqStorage::add_item(
            Origin::signed(origin),
            item_type.to_vec(),
            item.to_vec()            
        ),
        Error::<Test>::ItemTypeAlreadyExists
    );
        
    });
}

//Test to add item with item type length exceed maximum limit
#[test]
fn add_item_type_length_exceeds_limit_test(){
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        let acct="Iredia";
        let origin = account_key(acct);
        let item = b"item";
        let tmp = String::from("a").repeat(65);               
        let item_type = tmp.as_bytes();    
        
        assert_noop!(
            PeaqStorage::add_item(
                Origin::signed(origin), 
                item_type.to_vec(), 
                item.to_vec()
        ),
        Error::<Test>::ItemTypeExceedMax64
    );      
        
    });
}

//Test to add an item with item length exceed maximum limit
#[test]
fn add_item_length_exceeds_limit_test(){
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        let acct="Iredia";
        let origin = account_key(acct);
        let item_type = b"itemType";
        let tmp = String::from("a").repeat(129);
        let item = tmp.as_bytes();
        
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

//Test to update an item
#[test]
fn update_item_test_ok() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);     

        let acct="Iredia";
        let origin = account_key(acct);
        let item_type = b"itemType";
        let  item = b"item";
        
        //Add an item
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       

        //update item
        assert_ok!(
            PeaqStorage::update_item(
                Origin::signed(origin), 
                item_type.to_vec(), 
                b"new_item".to_vec()
            )
        );
        
    });
}

//Test to update a non existing item
#[test]
fn update_non_existing_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);     

        let acct="Iredia";
        let origin = account_key(acct);
                
        assert_noop!(
            PeaqStorage::update_item(
                Origin::signed(origin), 
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

        let acct="Iredia";
        let origin = account_key(acct);
        let item_type = b"itemType";
        let item = b"item";
        let tmp = String::from("a").repeat(129);

        //Add an item
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       

        let item=tmp.as_bytes();

        //Update the item with item length exceed the limit
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
//Test to update an other owner's item
fn update_other_owner_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);     

        let acct="Iredia";
        let acct2 = "Fake";
        let origin = account_key(acct);
        let fake_origin = account_key(acct2);
        let item_type = b"itemType";
        let item = b"item";
        
        //Add an item with user Iredia
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );

        //Update the item with fake owner
        assert_noop!(
            PeaqStorage::update_item(
                Origin::signed(fake_origin), 
                item_type.to_vec(), 
                item.to_vec()
            ),
            Error::<Test>::ItemNotFound
        );               

    });
}

//Test to get an item
#[test]
fn get_item_test_ok() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);     

        let acct="Iredia";
        let origin = account_key(acct);
        let item_type = b"itemType";
        let item = b"item";
        
        //Add an item
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       

        //Get the same item
        assert_ok!(
            PeaqStorage::get_item(
                Origin::signed(origin),
                item_type.to_vec()                
            )

        );     

                
    });
}

//Test to get a non existing item
#[test]
fn get_non_existing_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);     

        let acct="Iredia";
        let origin = account_key(acct);
        let item_type = b"itemType";
        let item = b"item";
        
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );     
              
        //Get a non existing item
        assert_noop!(
            PeaqStorage::get_item(
                Origin::signed(origin),
                b"new_item_type".to_vec()
        ),
        Error::<Test>::ItemNotFound
        );
                
    });
}

//Test to get another owner item
#[test]
fn get_other_owner_item_test() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);     

        let acct="Iredia";
        let acct2 = "fake";
        let origin = account_key(acct);
        let fake_origin = account_key(acct2);
        let item_type = b"itemType";
        let item = b"item";
        
        //Add an item 
        assert_ok!(
            PeaqStorage::add_item(
                Origin::signed(origin),
                item_type.to_vec(),
                item.to_vec()
            )
        );       

        //Get anotehr owner's item
        assert_noop!(
            PeaqStorage::get_item(
                Origin::signed(fake_origin),
                item_type.to_vec()                                
            ),
            Error::<Test>::ItemNotFound

        );
        
    });
}
