# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

"""Builds a target for a given top, whatever `--//hw/top` says.

The host tools are built once for every top, and take everything that differs between tops
from the top description at run time. Their tests check a description against the C
definitions generated for its top, so they need those definitions built for a top of
their own choosing.

For tests that check one top's definitions; build everything else for the `--//hw/top` top.
"""

load("//hw/top:defs.bzl", "pavona_require_top")

def pavona_require_tops(tops):
    """Returns a `target_compatible_with` value for a target that builds only for `tops`."""
    return pavona_require_top(tuple(tops))

def _top_transition_impl(_settings, attr):
    return {"//hw/top": attr.top}

_top_transition = transition(
    implementation = _top_transition_impl,
    inputs = [],
    outputs = ["//hw/top"],
)

def _pavona_for_top_impl(ctx):
    dep = ctx.attr.dep[0]  # An attribute with a transition is a list.
    files = dep[DefaultInfo].files.to_list()
    if len(files) == 1:
        # In this target's configuration (the consumer's) and under its own name: rules_rust
        # relocates sources from other configurations, and `dep`'s file may also be built in
        # this configuration under its own name.
        out = ctx.actions.declare_file(ctx.label.name + "." + files[0].extension)
        ctx.actions.symlink(output = out, target_file = files[0])
        files = [out]
    return [DefaultInfo(files = depset(files))]

pavona_for_top = rule(
    implementation = _pavona_for_top_impl,
    attrs = {
        "dep": attr.label(
            mandatory = True,
            cfg = _top_transition,
            doc = "The target to build for `top`. Its files (a single file as `<name>.<ext>`) " +
                  "are passed on.",
        ),
        "top": attr.string(
            mandatory = True,
            doc = "The top to build `dep` for (a `//hw/top` value).",
        ),
    },
    doc = "Builds `dep` with `//hw/top` set to `top` and passes on its files.",
)
