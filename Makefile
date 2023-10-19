ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 17929844
# aave 17929844
# wbtc 17922757
STOP_BLOCK ?= +1000000

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_liquidations -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams/sink/database,sf/substreams/rpc,sf/substreams/v1"

.PHONY: pack
pack: build
	substreams pack substreams.yaml
