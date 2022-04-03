include config.mk
PROGNAME       = gfret
INSTALLDIRS    = $(BINDIR)
VPATH         += src
VPATH         += target/release
VPATH         += data
SRCS          += Cargo.toml
SRCS          += backend.rs
SRCS          += config.rs
SRCS          += gui/adjustments.rs
SRCS          += gui/file.rs
SRCS          += gui/gui.ui
SRCS          += gui/mod.rs
SRCS          += gui/dialogs/mod.rs
SRCS          += gui/dialogs/prefs.ui
SRCS          += build.rs
SRCS          += cli.rs
SRCS          += main.rs
SRCS          += template.rs
INSTALLDIRS   += $(XDGDIR)
INSTALLDIRS   += $(ICONDIR)/scalable/apps
INSTALL_OBJS  += $(BINDIR)/$(PROGNAME)
INSTALL_OBJS  += $(XDGDIR)/$(PROGNAME).desktop
INSTALL_OBJS  += $(ICONDIR)/scalable/apps/$(PROGNAME).svg
ifeq ($(PNGICONS), true)
ICON128DIR     = $(ICONDIR)/128x128
ICON64DIR      = $(ICONDIR)/64x64
ICON48DIR      = $(ICONDIR)48x48
ICON32DIR      = $(ICONDIR)/32x32
INSTALLDIRS   += $(ICON128DIR) $(ICON64DIR) $(ICON48DIR) $(ICON32DIR)
ICON128        = $(ICON128DIR)/$(PROGNAME).png
ICON64         = $(ICON64DIR)/$(PROGNAME).png
ICON48         = $(ICON48DIR)/$(PROGNAME).png
ICON32         = $(ICON32DIR)/$(PROGNAME).png
INSTALL_OBJS  += $(ICON128) $(ICON64) $(ICON48) $(ICON32)
endif

all: $(PROGNAME)

$(PROGNAME): $(SRCS)
	cargo build --release

install: $(INSTALL_OBJS)

$(BINDIR)/$(PROGNAME): $(PROGNAME) | $(BINDIR)
	install -m0755 $< $@

$(XDGDIR)/$(PROGNAME).desktop: $(PROGNAME).desktop | $(XDGDIR)
	install -m644 $< $@

$(ICONDIR)/scalable/apps/$(PROGNAME).svg: $(PROGNAME).svg | $(ICONDIR)/scalable/apps
	install -m644 $< $@

$(ICON128): $(PROGNAME).svg | $(ICON128DIR)
	rsvg-convert $< -w 128 -h 128 -o $@

$(ICON64): $(PROGNAME).svg | $(ICON64DIR)
	rsvg-convert $< -w 64 -h 64 -o $@

$(ICON48): $(PROGNAME).svg | $(ICON48DIR)
	rsvg-convert $< -w 48 -h 48 -o $@

$(ICON32): $(PROGNAME).svg | $(ICON32DIR)
	rsvg-convert $< -w 32 -h 32 -o $@

$(INSTALLDIRS):
	install -d $@

clean:
	rm -rf target/

uninstall:
	rm -rf $(BINDIR)/$(PROGNAME)

.PHONY: all clean install install-strip
