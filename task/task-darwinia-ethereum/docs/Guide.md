### Prepare
1. `cp .maintain/config/task-darwinia-ethereum.toml ~/.bridger`
   The dir can be any path you like, and fill the content.
2. `cp ~/.bridger/task-darwinia-ethereum.toml ~/.bridger/linked-darwinia.toml`
   Keep the two file's content the same.
3. `cargo b --release`

### Run
4. `./target/release/darwinia-bridger server --base-path ~/.bridger`
   it may take a while to run all services if the network is not good. this will run all services except the darwinia and ethereum subscribe services. These two services should be started manually.

5. Open another shell

6. Start the darwinia subscribe service
    ```
    ./target/release/darwinia-bridger task exec --name task-darwinia-ethereum --api start-darwinia
    ``` 
   or start it with a block_number
    ```
    ./target/release/darwinia-bridger task exec --name task-darwinia-ethereum --api start-darwinia --param block_number=4230622
    ```

7. Start the ethereum subscribe service
    ```
    ./target/release/darwinia-bridger task exec --name task-darwinia-ethereum --api start-ethereum
    ```
   or start it with a block_number
    ```
    ./target/release/darwinia-bridger task exec --name task-darwinia-ethereum --api start-ethereum --param block_number=12856303
    ```
   
Note: Darwinia web socket node connected requires enabling [offchain-indexing](https://github.com/darwinia-network/bridger/issues/196#issuecomment-884056708).