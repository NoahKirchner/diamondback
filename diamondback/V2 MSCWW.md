## Must Have
* Communication from local client to team client to target host and back.
* The ability to run arbitrary commands on the target host.
* A UI on the client that allows commands to be sent.
## Should Have
* Beaconing functionality and a buffer stored on the team server.
* The ability to configure the beacon to be compiled by the local client at runtime.
* A properly implemented default communication protocol.

## Could Have
* The ability for the beacon to only have certain features compiled into it, for example configuring it to have the ability to communicate over HTTP, DNS or QUIC, or any combination of the three.
* The ability to chain multiple beacons together to communicate, TCP pivoting basically.

## Won't Have This Time
* The ability to dynamically load shared objects into memory FROM memory during runtime (IE transferring the bytes of a shared object over the network to be interpreted by libloading or dlopen at runtime)
* Process injection or migration functionality
* Implementations of popular tools built into the source code (Like Rubeus in Sliver)
## Won't Have Ever