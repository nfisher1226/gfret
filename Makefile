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
SRCS          += main.rs
SRCS          += template.rs
INSTALLDIRS   += $(XDGDIR)
INSTALLDIRS   += $(ICONDIR)
INSTALL_OBJS  += $(BINDIR)/$(PROGNAME)
INSTALL_OBJS  += $(XDGDIR)/$(PROGNAME).desktop
INSTALL_OBJS  += $(ICONDIR)/$(PROGNAME).svg

all: $(PROGNAME)

$(PROGNAME): $(SRCS)
	cargo build --release

install: $(INSTALL_OBJS)

install-strip: $(INSTALL_OBJS)
	strip -s $<

$(BINDIR)/$(PROGNAME): $(PROGNAME) | $(BINDIR)
	install -m0755 $< $@

$(XDGDIR)/$(PROGNAME).desktop: $(PROGNAME).desktop | $(XDGDIR)
	install -m644 $< $@

$(ICONDIR)/$(PROGNAME).svg: $(PROGNAME).svg | $(ICONDIR)
	install -m644 $< $@

$(INSTALLDIRS):
	install -d $@

clean:
	rm -rf target/

uninstall:
	rm -rf $(BINDIR)/$(PROGNAME)

.PHONY: all clean install install-strip
