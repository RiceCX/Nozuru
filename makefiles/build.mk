# Build targets
build-cli: install_rustup_target
	$(CARGO_BUILDER) build --bin nozuru $(RELEASE_FLAG) $(TARGET_FLAG) $(VERBOSE_FLAG)

install_rustup_target:
	./build-scripts/install_target.sh
