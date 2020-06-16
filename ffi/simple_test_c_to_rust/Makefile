LDFLAGS := -Wl,--gc-sections -lpthread -ldl

all: target/show

target:
	mkdir -p $@

target/show: target/main.o target/debug/libshow.a
	@echo $(LDFLAGS)
	$(CC) -o $@ $^ $(LDFLAGS)



target/debug/libshow.a: src/lib.rs Cargo.toml
	cargo build


target/main.o: src/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target
