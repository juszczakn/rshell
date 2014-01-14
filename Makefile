RUSTC=rustc
RFLAGS=-o
SRC=src/
EXE=rshell
SOURCES=rshell.rs

rshell: $(SRC)$(SOURCES)
	$(RUSTC) $(SRC)$(SOURCES) $(RFLAGS) $(EXE)
