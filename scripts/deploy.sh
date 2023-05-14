#!/bin/bash

wasmd tx wasm store $(pwd)/artifacts/$CONTRACT_FILE --from wallet $TXFLAG -y -b block