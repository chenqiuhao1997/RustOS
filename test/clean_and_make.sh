realpath=$(dirname "$0")
cd $realpath
cd ../kernel
make clean
if [ $# -lt 1 ]; then
	make build smp=1
else
	make build smp=1 target_func=$1
fi

