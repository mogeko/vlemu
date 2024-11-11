# Rust libkms binding

> [!CAUTION]
>
> This is a very low-level direct binding to a C library, what you may need is the [`kms`](https://github.com/mogeko/vlemu/blob/master/crates/kms) package.

This library includes the Rust bindings for `libkms`, which is part of the [`vlmcsd`](https://www.upload.ee/files/11363713/vlmcsd-1113-2020-03-28-Hotbird64-source-only.7z.html) package.

## What is KMS？

KMS is a way to activate Microsoft products that was designed for medium and large businesses. In a standard SOHO environment you enter a product key during installation and then activate your product over the Internet. This is done by sending a request to a server at `microsoft.com` which then either grants or refuses activation.

By entering a special key called General Volume License Key (GVLK), a.k.a "KMS client key", the product no longer asks the Microsoft server for activation but a user-defined server (called the KMS server) which usually resides in a company's intranet.

## What is vlmcsd?

> [!IMPORTANT]
>
> `vlmcsd` and this project are two independent projects, and we do not provide **ANY ENDORSEMENT** for `vlmcsd` or `libkms`.

`vlmcsd` is an independent open source implementation of a KMS server that is available for everyone while Microsoft gives their KMS server only to corporations that signed a so called "Select contract". In addition vlmcsd never refuses activation while the Microsoft KMS server only activates the products the customer has paid for.
