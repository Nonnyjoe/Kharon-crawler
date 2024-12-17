## Simple Documentation for available endpoints and how to use them.

- **CREATE A NEW USER:**

  - **url:** http://127.0.0.1/user
  - **method:** POST
  - **body:** An object containing; "name" and "email".
  - **example:**

  ```javascript
    {
        "name": "alice",
        "email": "alice@gmail.com"
    }
  ```

- **ADD A WALLET :**

  - **url:** http://127.0.0.1/user/wallets
  - **method:** POST
  - **body:** An object containing; "user_id", "wallet_address" and "network".
  - **example:**

  ```javascript
  {
    "user_id":"9e164e8f-5201-4fcf-abc4-d0f6824a58ed",
    "wallet_address": "0x07b649b20453b7efd8168056287540fbae24da819348689a7592e2ea55d0680d",
    "network": "Starknet"
  }
  ```

- **CREATE A NETWORK (ADMIN ACCESS):**

  - **url:** http://127.0.0.1/admin/network
  - **method:** POST
  - **body:** An object containing; "network_type" and "chain_id".
  - **example:**

  ```javascript
  {
    "network_type" : "Ethereum",
    "chain_id": "0x534e5f4d41494e"
  }
  ```

- **UPDATE A USERS EMAIL:**

  - **url:** http://127.0.0.1/user/email
  - **method:** PATCH
  - **body:** An object containing; "old_email" and "new_email".
  - **example:**

  ```javascript
  {
    "old_email": "samuel1@gmail.com",
    "new_email": "samuel@gmail.com"
  }
  ```

- **UPDATE A USERS WALLET:**

  - **url:** http://127.0.0.1/user/wallets
  - **method:** PATCH
  - **body:** An object containing; "user_id", "wallet_address" and "new_network".
  - **example:**

  ```javascript
  {
    "user_id":"31ae366b-ac54-4f3c-a17a-b70af0645bbb",
    "wallet_address": "0x07b649b20453b7efd8168056287540fbae24da819348689a7592e2ea55d0680d",
    "new_network": "Starknet"
  }
  ```

- **UPDATE NETWORK (ADMIN ACCESS):**

  - **url:** http://127.0.0.1/admin/network
  - **method:** PATCH
  - **body:** An object containing; "old_chain_id" and "new_chain_id".
  - **example:**

  ```javascript
  {
    "old_chain_id": "0x534e5f4d41494e",
    "new_chain_id": "0x534e5f4d41494e333333333"
  }
  ```

- **UPDATE LAST SCANNED BLOCK (ADMIN ACCESS):**

  - **url:** http://127.0.0.1/admin/network/last_scanned_block
  - **method:** PATCH
  - **body:** An object containing; "network_type" and "last_scanned_block".
  - **example:**

  ```javascript
  {
    "network_type": "Starknet",
    "last_scanned_block": 100
  }
  ```

- **DELETE A WALLET :**

  - **url:** http://127.0.0.1/user/wallets
  - **method:** DELETE
  - **body:** An object containing; "user_id" and "wallet_address".
  - **example:**

  ```javascript
  {
    "user_id":"9e164e8f-5201-4fcf-abc4-d0f6824a58ed",
    "wallet_address": "0x07b649b20453b7efd8168056287540fbae24da819348689a7592e2ea55d0680d",
  }
  ```

- **DELETE A NETWORK (ADMIN ACCESS):**

  - **url:** http://127.0.0.1/admin/network
  - **method:** DELETE
  - **body:** An object containing; "network_type".
  - **example:**

  ```javascript
  {
    "network_type": "Starknet",
  }
  ```

- **GET A USERS WALLETS:**

  - **url:** http://127.0.0.1/user/wallets/by-id/{USER_ID}
  - **method:** GET

- **GET A USERS PROFILE VIA ID:**

  - **url:** http://127.0.0.1/user/by-id/{USER_ID}
  - **method:** GET

- **GET A USERS PROFILE VIA EMAIL:**

  - **url:** http://127.0.0.1/user/by-email/{EMAIL_ADDRESS}
  - **method:** GET

- **GET ALL USERS THAT HAVE ADDRESS UNDER A PARTICULAR NETWORK:**

  - **url:** http://127.0.0.1/users/{NETWORK}
  - **method:** GET

- **GET A USERS PROFILE VIA WALLET ADDRESS:**

  - **url:** http://127.0.0.1/users/by-wallet/{WALLET_ADDRESS}
  - **method:** GET

- **GET ALL USERS:**

  - **url:** http://127.0.0.1/users
  - **method:** GET

- **GET ALL WALLETS BY NETWORK:**

  - **url:** http://127.0.0.1/wallets/{NETWORK}
  - **method:** GET

- **GET ALL SUPPORTED NETWORKS:**

  - **url:** http://127.0.0.1/admin/networks
  - **method:** GET

- **GET LAST SCANNED BLOCK:**
  - **url:** http://127.0.0.1/admin/network/{NETWORK}/last_scanned_block
  - **method:** GET
