#!/bin/sh

if test ! -d "./exes/"; then
	echo "Creating exe directory"
	mkdir "./exes/"
fi

for comp in 0 8 16 32
do
	for size in 1 32 128 512 1024
	do
		for iter in 10 20 30
		do
			SIZE_MB=$((size * 1024 * 1024))
			ln -s "${PWD}/target/debug/ioskel" "./exes/ioskel.${comp}.${SIZE_MB}.${iter}"
		done
	done

done

