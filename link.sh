ld -o hello test.o -lSystem -syslibroot $(xcrun -sdk macosx --show-sdk-path) -e _start -arch arm64
