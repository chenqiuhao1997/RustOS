realpath=$(dirname "$0")
cd $realpath
cd ../kernel
make clean
if [ $# -lt 1 ]; then
    make run smp=1
else
	make test smp=1 test_target=$1
fi
