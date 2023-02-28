
chibrust: 
	cargo build

test: 
	./test.sh

clean:
	cargo clean && rm -f *.o *~ tmp*

.PHONY: test clean
	