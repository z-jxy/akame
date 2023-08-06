cc_binary(
    name = "main",
    srcs = ["main.o"] + glob(["lib/darwin-*.o"]),
    copts = ["-Wall"],
    #linkopts = ["-lm"],  # Add other linker options if required.
)