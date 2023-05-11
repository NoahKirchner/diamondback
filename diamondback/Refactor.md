* Refactor the protocol that everything is communicated over.
* Refactor the NetworkManager object and break it down into smaller, more manageable pieces that can be iterated on or modified for things like tcp pivoting.
* The command system:
  For every command that the beacon binary is meant to have, store a Box(command) reference in a hashmap. To ensure that certain features can be skipped over with compilation without being added to the hashmap, use `#[cfg(feature = "feature")] `above the insert for the hashmap
  * At some point you need to implement some type of thread channel system or this code is going to make you want to kill yourself.