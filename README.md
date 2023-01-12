
# Peaq Pallet Storage

#### Introduction
The Peaq Storage pallet is  for storing an item value with the type of that item [CID](https://docs.ipfs.tech/concepts/content-addressing/#content-addressing-and-cids) (content Identifier)

It stores a key-value pair with the key being user public key + type of item and the value of the item.

#### Installation
* Import the pallet dependecies by adding below snippets to your `runtime/Cargo.toml` file.
```
# --snip--

[dependencies.peaq-pallet-storage]
default-features = false
git = 'https://github.com/peaqnetwork/peaq-storage-pallet.git'
version = '0.0.1'

# --snip--

[features]
default = ['std']
runtime-benchmarks = [
  # --snip--
  'peaq-pallet-storage/runtime-benchmarks',
]
std = [
  'peaq-pallet-storage/std',
  # --snip--
]
```

* Implement peaq storage pallet on your runtime by adding below snippets to `runtime/src/lib.rs` file.
```
# --snip--

pub use peaq_pallet_storage;

# --snip--

// Config the storage in pallets/storage
impl peaq_pallet_storage::Config for Runtime {
	type Event = Event;
	type WeightInfo = peaq_pallet_storage::weights::SubstrateWeight<Runtime>;
}
```

* Add PeaqStorage parameter type to the runtime construct on your `runtime/src/lib.rs` file using below snippet.
```
# --snip--
PeaqStorage: peaq_pallet_storage::{Pallet, Call, Storage, Event<T>},
# --snip--
```

### Usage
* After installation, build your node
* Run and connect your node to Polkadorjs App
* Check for `PeaqStorage` under `developer - Extrinsics` tab.


### Implementation:
Currently peaq storage pallet supports following three Extrinsics:

#### Add Item:
* For adding an item (max lenght 128 bytes) with the item type (max lenght 64 bytes)
* If item type length exceed maximum lenght, return item type exceed maximum length error 
* If item length exceed maximum lenght, return item exceed maximum length error 

**Example**

![](https://user-images.githubusercontent.com/101552881/201901023-51fbb930-ca33-44e1-85e9-b6625fafddb4.png)

#### Update Item:
* Update item that already exists against an item type
* Return Not Found Error if no item exists against the given item type
* Owner can only update their  owned item. 
* In case an owner try to update item of another owner, item not found error is returned,   
  since item type is stored as user public key + item type 
  
**Example**

![](https://user-images.githubusercontent.com/101552881/201902039-34a01db5-b478-4dd7-b9e8-9d01dcb7ab56.png)


#### Read Item:
* To read an item against the given item type
* Owner can only read their owned item
* if a user try to read item owned by another  owner, item not found error will be 
  returned

**Example**

![](https://user-images.githubusercontent.com/101552881/201901841-b592432d-90c6-451a-ad93-89483b85ce41.png)


## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)

