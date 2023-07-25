PROGRAMNAME=mediacuddle
PREFIX=/usr/local
TARGET=./target/release/$(PROGRAMNAME)
BIN_PATH=$(PREFIX)/bin
BIN=$(BIN_PATH)/$(PROGRAMNAME)


install:
	chmod 755 $(TARGET)
	mkdir -p $(BIN_PATH)
	cp $(TARGET) $(BIN)

uninstall:
	rm -f $(BIN)


.PHONY: install uninstall 
