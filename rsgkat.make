# benchmarking
kernel = k1
solver = bdd

_DATASET0 := $(shell find dataset0 -name '*.txt')
_DATASET1 := $(shell find dataset1 -name '*.txt')
_E250B5P10NE := $(shell find benchmark/e250b5p10ne -name '*.txt')
_E250B5P10EQ := $(shell find benchmark/e250b5p10eq -name '*.txt')
_E500B5P50NE := $(shell find benchmark/e500b5p50ne -name '*.txt')
_E500B5P50EQ := $(shell find benchmark/e500b5p50eq -name '*.txt')
_E1000B10P100NE := $(shell find benchmark/e1000b10p100ne -name '*.txt')
_E1000B10P100EQ := $(shell find benchmark/e1000b10p100eq -name '*.txt')
_E2000B20P200NE := $(shell find benchmark/e2000b20p200ne -name '*.txt')
_E2000B20P200EQ := $(shell find benchmark/e2000b20p200eq -name '*.txt')
_E3000B30P200NE := $(shell find benchmark/e3000b30p200ne -name '*.txt')
_E3000B30P200EQ := $(shell find benchmark/e3000b30p200eq -name '*.txt')
_DEGENERATE := $(shell find benchmark/degenerate -name '*.txt')

DATASET0 := $(subst dataset0/,test0/,$(_DATASET0))
DATASET1 := $(subst dataset1/,test1/,$(_DATASET1))
E250B5P10NE := $(subst benchmark/e250b5p10ne/,e250b5p10ne/,$(_E250B5P10NE))
E250B5P10EQ := $(subst benchmark/e250b5p10eq/,e250b5p10eq/,$(_E250B5P10EQ))
E500B5P50NE := $(subst benchmark/e500b5p50ne/,e500b5p50ne/,$(_E500B5P50NE))
E500B5P50EQ := $(subst benchmark/e500b5p50eq/,e500b5p50eq/,$(_E500B5P50EQ))
E1000B10P100NE := $(subst benchmark/e1000b10p100ne/,e1000b10p100ne/,$(_E1000B10P100NE))
E1000B10P100EQ := $(subst benchmark/e1000b10p100eq/,e1000b10p100eq/,$(_E1000B10P100EQ))
E2000B20P200NE := $(subst benchmark/e2000b20p200ne/,e2000b20p200ne/,$(_E2000B20P200NE))
E2000B20P200EQ := $(subst benchmark/e2000b20p200eq/,e2000b20p200eq/,$(_E2000B20P200EQ))
E3000B30P200NE := $(subst benchmark/e3000b30p200ne/,e3000b30p200ne/,$(_E3000B30P200NE))
E3000B30P200EQ := $(subst benchmark/e3000b30p200eq/,e3000b30p200eq/,$(_E3000B30P200EQ))
DEGENERATE := $(subst benchmark/degenerate/,degenerate/,$(_DEGENERATE))

test0/%.txt: dataset0/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
test1/%.txt: dataset1/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e250b5p10ne/%.txt: benchmark/e250b5p10ne/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e250b5p10eq/%.txt: benchmark/e250b5p10eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e500b5p50ne/%.txt: benchmark/e500b5p50ne/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e500b5p50eq/%.txt: benchmark/e500b5p50eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e1000b10p100ne/%.txt: benchmark/e1000b10p100ne/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e1000b10p100eq/%.txt: benchmark/e1000b10p100eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e2000b20p200ne/%.txt: benchmark/e2000b20p200ne/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e2000b20p200eq/%.txt: benchmark/e2000b20p200eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e3000b30p200ne/%.txt: benchmark/e3000b30p200ne/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
e3000b30p200eq/%.txt: benchmark/e3000b30p200eq/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<
degenerate/%.txt: benchmark/degenerate/%.txt 
	./target/release/rust-gkat -k ${kernel} -s ${solver} $<

test0: $(DATASET0)
test1: $(DATASET1)
e250b5p10ne: $(E250B5P10NE)
e250b5p10eq: $(E250B5P10EQ)
e500b5p50ne: $(E500B5P50NE)
e500b5p50eq: $(E500B5P50EQ)
e1000b10p100ne: $(E1000B10P100NE)
e1000b10p100eq: $(E1000B10P100EQ)
e2000b20p200ne: $(E2000B20P200NE)
e2000b20p200eq: $(E2000B20P200EQ)
e3000b30p200ne: $(E3000B30P200NE)
e3000b30p200eq: $(E3000B30P200EQ)
degenerate: $(DEGENERATE)