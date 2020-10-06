This is a work in progress rust interface to zfs.

It started as a wrapper around libzfs, but since libzfs is not considered
a stable interface, this now directly sends ioctls to /dev/zfs. The only
C library required for usage is libnvpair.

The goals of this project are to have types representing everything needed in ZFS and only
permitting actions that should succeed. For example setting a property on a
dataset that the property does not apply to should be prevented by the type
system.

Properties are dymaically generated from libzfs, and stored in src/_generated.
Use the regen feature to regenerate them.
