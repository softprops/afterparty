# 0.1.3 (unreleased)

* make signature verification happen in constant time, avoiding potential timing attack
* only parse inbound events if interested hooks exists
* replaced inferred payload structs with explicit payload structs. which speeds up compilation and fixes
some issues with the generated output.

# 0.1.2

* disabling hyper in build dependencies to work around musl build issue on musl

# 0.1.1

* update serde/hyper dependencies
* rework code generation

# 0.1.0

* initial release
