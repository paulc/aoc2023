#!/bin/sh

for d in day*
do
    cd $d
    cargo build --release
    cd ..
done

time ( 
    for d in day*
    do
        echo '===' $d
        cd $d
        ./target/release/$d
        cd ..
    done
)
