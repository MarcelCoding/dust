# Dust

## protocol

A package has a 40 bit header and a variable length of data.

```
u8        u32            vec<u8>
^         ^              ^
pkg type  pkg data size  pkg data
```
