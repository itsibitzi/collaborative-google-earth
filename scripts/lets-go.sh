#!/bin/bash
SCRIPT_PATH=$( cd $(dirname $0) ; pwd -P )

INTERCEPTOR_DLL=$(realpath "$SCRIPT_PATH/../target/debug/libinterceptor.dylib")

if [ ! -f "$INTERCEPTOR_DLL" ]; then
    echo 'Could not find interceptor DLL, sad bois'
    exit 1 
fi

DYLD_FORCE_FLAT_NAMESPACE=1 DYLD_PRINT_LIBRARIES=1 X=1 DYLD_INSERT_LIBRARIES="$INTERCEPTOR_DLL" /Applications/Google\ Earth\ Pro.app/Contents/MacOS/Google\ Earth