#!/bin/bash
brainfuck=$1
testdir=$(dirname "$0")

tmp=`mktemp -d /tmp/brainfuck-test-XXXXXX`
trap 'rm -rf $tmp' INT TERM HUP EXIT

check() {
    if [ $? -eq 0 ]; then
        echo "testing $1 ... passed"
    else
        echo "testing $1 ... failed"
        exit 1
    fi
}

# mandelbrot
rm -f $tmp/out
$brainfuck $testdir/mandelbrot.b > $tmp/mandelbrot.actual
diff $tmp/mandelbrot.actual $testdir/mandelbrot.expect
check mandelbrot

echo OK
