load("//tools:defs.bzl", "my_rule", helper = "helper_fn")

DEPS = ["//lib:core"]

def my_macro(name, srcs = []):
    inner = 1
    native.filegroup(name = name + "_files", srcs = srcs)

py_library(
    name = "lib",
    srcs = ["lib.py"],
)

target = my_rule(name = "gen")
