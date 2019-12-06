cd src
rustc -l static --cfg 'ENABLE_UTF8' chars.rs --print native-static-libs
gcc -c testrs.c
gcc -o test testrs.o libchars.a  -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lpthread -lutil -lutil
./test