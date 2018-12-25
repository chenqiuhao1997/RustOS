realpath=$(dirname "$0")
cd $realpath
cd ../kernel
make run test=lab_test smp=1
