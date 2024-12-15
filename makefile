SHELL := bash

DEVICES := $(foreach svd,$(wildcard svds/*.svd),$(svd:svds/%.svd=%))
PATCHED_SVDS := $(foreach device,$(DEVICES),svds/$(device).svd.patched)

.PHONY: patch

svds/%.svd.patched: svds/%.yaml svds/%.svd
	svdtools patch $<
patch: $(PATCHED_SVDS)

crate: $(PATCHED_SVDS)
	cargo run --release -- crate
