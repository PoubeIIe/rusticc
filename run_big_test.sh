clear
rustc main.rs
if [[ $? == 0 ]]; then
	./main Tests/test.c Tests/out.s
	if [[ $? == 0 ]]; then
		gcc ./Tests/out.s -o ./Tests/test
		echo "Output of program execution:"
		./Tests/test
		res=$?
		echo "Result of program execution:"
		echo $res
	fi
fi