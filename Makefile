VERSION := $(shell cat VERSION)
RUSTV?=stable
GIT_COMMIT=$(shell git rev-parse HEAD)
ARCH=$(shell uname -m)
TARGET?=
IMAGE_VERSION?=					# If set, this indicates that the image is pre-built and should not be built
BUILD_PROFILE=$(if $(RELEASE),release,debug)
CARGO_BUILDER=$(if $(findstring arm,$(TARGET)),cross,cargo) # If TARGET contains the substring "arm"
FLUVIO_BIN?=$(if $(TARGET),./target/$(TARGET)/$(BUILD_PROFILE)/Nozuru,./target/$(BUILD_PROFILE)/Nozuru)
RELEASE_FLAG=$(if $(RELEASE),--release,)
TARGET_FLAG=$(if $(TARGET),--target $(TARGET),)
VERBOSE_FLAG=$(if $(VERBOSE),--verbose,)

BUILD_FLAGS = $(RELEASE_FLAG) $(TARGET_FLAG) $(VERBOSE_FLAG)

include makefiles/build.mk

clean:
	cargo clean

.EXPORT_ALL_VARIABLES:
CC_aarch64_unknown_linux_musl=$(PWD)/build-scripts/aarch64-linux-musl-zig-cc
CC_x86_64_unknown_linux_musl=$(PWD)/build-scripts/x86_64-linux-musl-zig-cc
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=$(PWD)/build-scripts/ld.lld
CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=$(PWD)/build-scripts/ld.lld
