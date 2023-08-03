PROGRAMNAME := mediacuddle
PREFIX ?= /usr/local
REL_SITE_FUNCTIONS := share/zsh/site-functions
SITE_FUNCTIONS := $(PREFIX)/$(REL_SITE_FUNCTIONS)

TARGET := ./target/release/$(PROGRAMNAME)
BIN_PATH := $(PREFIX)/bin
BIN := $(BIN_PATH)/$(PROGRAMNAME)


install:
	chmod 755 $(TARGET)
	mkdir -p $(BIN_PATH)
	mkdir -p $(SITE_FUNCTIONS)
	cp $(TARGET) $(BIN)
	cp $(REL_SITE_FUNCTIONS)/_$(PROGRAMNAME) $(SITE_FUNCTIONS)

uninstall:
	rm -f $(BIN)
	rm -f $(SITE_FUNCTIONS)/_$(PROGRAMNAME)


.PHONY: install uninstall 
