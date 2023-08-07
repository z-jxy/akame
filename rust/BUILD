genrule(
    name = "cargo_codegen",
    srcs = ["Cargo.toml", "Cargo.lock"] + glob(["src/**/*.rs"]),
    outs = ["main.ll"],
    cmd = """
        cd rust
        cargo run -- llvm
        echo "Intended output path: $(location main.ll)"
        echo "[+]: compiled | $$(pwd)" && ls -al
        cp main.ll ../$(location main.ll)
    """
)

genrule(
    name = "llvm_to_obj",
    srcs = [":cargo_codegen"],  # Depending on the output of the cargo_codegen rule
    outs = ["main.o"],
    cmd = "llc -filetype=obj $(location :cargo_codegen) -o $(OUTS)"
)

cc_binary(
    name = "main",
    srcs = [":llvm_to_obj"],
    copts = ["-Wall"],
    #linkopts = ["-lm"],  # Add other linker options if required.
)