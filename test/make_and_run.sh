realpath=$(dirname "$0")
cd $realpath
cd ../kernel
if [ $# -lt 1 ]; then
    make run smp=1
else
	make test smp=1 target_func=$1
fi
