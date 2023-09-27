# Copyright lowRISC contributors.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

"""Rules for writing per-crate build files for tock and libtock."""

BUILD_FILE = """###############################################################################
# @generated
# DO NOT MODIFY: This file is auto-generated by the crate_build function in
# //rules:rust.bzl.
###############################################################################

load("@rules_rust//rust:defs.bzl", "rust_library")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "{name}",
    srcs = {srcs},
    compile_data = {compile_data},
    crate_features = {crate_features},
    crate_name = "{crate_name}",
    crate_root = "{crate_root}",
    edition = "{edition}",
    rustc_flags = {rustc_flags},
    tags = {tags},
    version = "{version}",
    deps = {deps},
)

{additional_build_file_content}
"""

COMPILE_DATA = """glob(
        include = ["**"],
        exclude = [
            "**/* *",
            "BUILD",
            "BUILD.bazel",
            "WORKSPACE",
            "WORKSPACE.bazel",
        ],
    )"""

RUSTC_FLAGS = ["--cap-lints=allow"]

TAGS = """[
        "cargo-bazel",
        "crate-name={crate_name}",
        "manual",
        "noclippy",
        "norustfmt",
    ]"""

def _format_list(items):
    items = ["\"{}\",".format(i) for i in items]
    return "[\n        " + "\n        ".join(items) + "\n    ]"

def crate_build(
        name,
        srcs = [],
        compile_data = [],
        crate_features = [],
        crate_name = None,
        crate_root = "src/lib.rs",
        edition = "2021",
        rustc_flags = [],
        tags = [],
        version = "0.1.0",
        deps = [],
        additional_build_file_content = ""):
    """Create a BUILD file for a simple rust crate.

    Args:
        name: str; The name of the `rust_library` rule to generate.
        srcs: list[str]; A list of sources for library (optional).
        compile_data: list[str]; As list of files need at compile time (optional).
        crate_features: list[str]; Features to enable in the crate.
        crate_name: str; The name of the create (defaults to `name`).
        crate_root: str; The root source of the crate (defaults to `src/lib.rs`).
        edition: str; The rust edition to use (defaults to 2021).
        rustc_flags: list[str]; Flags to pass to the compiler (optional).
        tags: list[str]; Tags to for the library rule (optional).
        version: str; The crate version (defaults to "0.1.0").
        deps: list[str]; The dependencies needed by this library (optional).
        additional_build_file_content: str; Additional content to place in the build
            file after the automatic rules.
    Returns:
        str: The BUILD file content.
    """
    subst = {
        "name": name,
        "crate_features": _format_list(crate_features),
        "crate_root": crate_root,
        "edition": edition,
        "version": version,
        "deps": _format_list(deps),
        "additional_build_file_content": additional_build_file_content,
    }

    if srcs:
        subst["srcs"] = _format_list(srcs)
    else:
        subst["srcs"] = "glob([\"**/*.rs\"])"

    if compile_data:
        subst["compile_data"] = _format_list(compile_data)
    else:
        subst["compile_data"] = COMPILE_DATA

    if rustc_flags:
        subst["rustc_flags"] = _format_list(rustc_flags)
    else:
        subst["rustc_flags"] = _format_list(RUSTC_FLAGS)

    if crate_name:
        subst["crate_name"] = crate_name.format(**subst).replace("-", "_")
    else:
        subst["crate_name"] = name.replace("-", "_")

    if tags:
        subst["tags"] = _format_list(tags)
    else:
        subst["tags"] = TAGS.format(**subst)

    return BUILD_FILE.format(**subst)