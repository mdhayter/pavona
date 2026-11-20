# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

"""The LLVM toolchain used for host and device software.

toolchains_llvm's own `llvm` extension may only be used by the root module, so
a repository that depends on Pavona (for example one holding an out-of-tree
top) could not build. This extension creates the same `llvm_toolchain` and
`llvm_toolchain_llvm` repositories by calling toolchains_llvm's repository
macro directly, which works whether or not Pavona is the root module.

Moved from MODULE.bazel.
"""

load("@toolchains_llvm//toolchain:rules.bzl", "llvm_toolchain")

_LLVM_VERSION = "23.1.5"

def _pavona_llvm_impl(module_ctx):
    # Pin the macOS SDK instead of using the installed one.
    macos_sdk = Label("@macos_sdk//:sysroot")
    llvm_toolchain(
        name = "llvm_toolchain",
        # TODO: Drop once clang's GCC detection prefers complete installations.
        extra_compile_flags = {"": ["-Wno-gcc-install-dir-libstdcxx"]},
        # macOS links the system libc++ dylib, so use its headers too.
        extra_cxx_flags = {"darwin-aarch64": [
            "-nostdinc++",
            "-isystem=/usr/include/c++/v1",
        ]},
        # Add the cryptolib warning features to the host toolchain; the device
        # toolchain lists them in //toolchain:pavona_toolchain.
        extra_known_features = [
            Label("//toolchain:feat_cryptolib_base_warnings"),
            Label("//toolchain:feat_cryptolib_warnings_host"),
        ],
        # Clang only reads the SDK version from `-isysroot`, not from `--sysroot`.
        extra_link_flags = {"darwin-aarch64": [
            "-isysroot",
            macos_sdk.workspace_root,
        ]},
        llvm_versions = {"": _LLVM_VERSION},
        sha256 = {
            "darwin-aarch64": "aba592b4a15e8f90c634c5367b51ec8418b8bcd2829865db331a17dc9418dcff",
            "linux-aarch64": "44f1e4b7a125b2763d56386a199928dd57ec32e151cc6042dce2f7b0da7bb651",
            "linux-x86_64": "f72edb0b36580ece7f52e23858f00c18fad4e87395ea7ad05d7e55c39f8fc78a",
        },
        strip_prefix = {
            "darwin-aarch64": "LLVM-{}-macOS-ARM64".format(_LLVM_VERSION),
            "linux-aarch64": "LLVM-{}-Linux-ARM64".format(_LLVM_VERSION),
            "linux-x86_64": "LLVM-{}-Linux-X64".format(_LLVM_VERSION),
        },
        sysroot = {"darwin-aarch64": str(macos_sdk)},
        urls = {
            "darwin-aarch64": ["https://github.com/llvm/llvm-project/releases/download/llvmorg-{0}/LLVM-{0}-macOS-ARM64.tar.xz".format(_LLVM_VERSION)],
            "linux-aarch64": ["https://github.com/llvm/llvm-project/releases/download/llvmorg-{0}/LLVM-{0}-Linux-ARM64.tar.xz".format(_LLVM_VERSION)],
            "linux-x86_64": ["https://github.com/llvm/llvm-project/releases/download/llvmorg-{0}/LLVM-{0}-Linux-X64.tar.xz".format(_LLVM_VERSION)],
        },
    )
    return module_ctx.extension_metadata(reproducible = True)

pavona_llvm = module_extension(
    implementation = _pavona_llvm_impl,
    doc = "Creates @llvm_toolchain and @llvm_toolchain_llvm (see the file docstring).",
)
