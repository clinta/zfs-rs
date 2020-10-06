This is *not* a usable libzfs wrapper. This exists to expose limited functions from libzfs-core and libzfs
that are used by zfs-rs to generate types for datasets and properties.

To build this for a different version of zfs than what is installed on the system first build zfs then install
to an alternate directory with `DESTDIR=/tmp/zfs make install`. Then build with `ZFS_DIR=/tmp/zfs cargo build`.
Make sure zfs was configured with `--prefix=/usr` if you choose to do this.