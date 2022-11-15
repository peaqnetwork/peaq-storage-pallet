# peaq-storage-pallet

# Overview
To call these extrinsic go to the Polkadot app and switch to agung network.
Go to Developer → Extrinsics. And choose the peaqStorage pallet from the list.
Storage pallet has 3 extrinsic calls as of now.

### addItem
  
### Params - item type (Max length 64), item (Max length 128).
Description - For adding an item type with any item.

## Example

![image](https://user-images.githubusercontent.com/101552881/201901023-51fbb930-ca33-44e1-85e9-b6625fafddb4.png)

### getItem

### Params - item type (Max length 64).

Description - For reading the item with the item's type. A user can only access those items which were added through that user an account/public key.

## Example

![image](https://user-images.githubusercontent.com/101552881/201901841-b592432d-90c6-451a-ad93-89483b85ce41.png)

### updateItem

### Params - item type (Max length 64), item (Max length 128).

Description - For updating item type with a new item. Only items can be updated not item type. Each item type is attached to the user's public key.

![image](https://user-images.githubusercontent.com/101552881/201902039-34a01db5-b478-4dd7-b9e8-9d01dcb7ab56.png)




