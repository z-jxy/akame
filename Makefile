# Variables
LLC=llc
GCC=gcc
BUILD_DIR=build
BINARY_OUTPUT=$(BUILD_DIR)/main
OBJ_OUTPUT=$(BUILD_DIR)/main.o
LL_INPUT=$(BUILD_DIR)/main.ll
lib=lib
libs=$(wildcard $(lib)/*.o)
DARWIN_LIBS=$(wildcard $(lib)/darwin-*.o)
# Default target
all: $(BINARY_OUTPUT)

# Build the Rust project (assuming it outputs the LLVM IR)
$(LL_INPUT):
	@echo "Building Rust project..."
	cargo run -- llvm > $(LL_INPUT)

# Generate object code from LLVM IR
$(OBJ_OUTPUT): $(LL_INPUT)
	$(LLC) -filetype=obj $(LL_INPUT) -o $(OBJ_OUTPUT)

# Link the object code to create the binary
$(BINARY_OUTPUT): $(OBJ_OUTPUT)
	$(GCC) $(OBJ_OUTPUT) $(DARWIN_LIBS) -o $(BINARY_OUTPUT)

ll-redo: $(LL_INPUT)
	$(LLC) -filetype=obj $(LL_INPUT) -o $(OBJ_OUTPUT)
	$(GCC) $(OBJ_OUTPUT) printd.o -o $(BINARY_OUTPUT)

run: $(BINARY_OUTPUT)
	./build/main

# Clean up the build artifacts
clean:
	rm -f $(LL_INPUT) $(OBJ_OUTPUT) $(BINARY_OUTPUT)

# Additional phony targets for clarity
.PHONY: all clean
