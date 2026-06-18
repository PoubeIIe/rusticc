clear
rustc main.rs
if [[ $? == 0 ]]; then
	./main
	if [[ $? == 0 ]]; then
		gcc out.s -o out_cmp
		echo "Output of program execution:"
		./out_cmp
		res=$?
		echo "Result of program execution:"
		echo $res
	fi
fi