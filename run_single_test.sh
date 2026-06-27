clear
rustc main.rs
if [[ $? == 0 ]]; then
	./main Tests/tests2/05_array.c Tests/tests2/assembled/out.s
	if [[ $? == 0 ]]; then
		gcc Tests/tests2/assembled/out.s -o Tests/tests2/build/out_cmp
		echo "Output of program execution:"
		./Tests/tests2/build/out_cmp
		res=$?
		echo "Result of program execution:"
		echo $res
	fi
fi