// TODO : SUPPORT THESE
// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>

int new_fn(int x, char y, char* z){ /*//
	this function tests several things, just 
	like the// other ones//*///lol
	struct Teststruct {
		int a;
		char b;
		char* chara;
	};
	struct Teststruct structure;
	structure.a = x;
	structure.b = y;
	// TODO: in compiler check if field actually exist befure trying to access it, eg acceccing a field chara while in the struct there is no chara, but the inteded one was called just "c".
	structure.chara = z;
	printf("structure.b : %d, y : %d \n", structure.b, y);
	printf("%s : %d\n",structure.chara, structure.b / structure.a);
	// TODO : test with *stucture.chara or structure->chara
	return structure.a + structure.b; // 144
}

/*test things*/
int function_b(char* str){
	bool a = true;
	if (a != true){
		puts("What?");
	}
	int x = 9;
	if (x+5 == 14){
		puts("yay : ");
		puts(str);
		printf("x : %d\n", x);
	}
	char buff[32];
	char chara = 'E';
	if (chara >= 67){
		x = 92;
		memcpy(buff, "test\n12 34", 11);
	}
	printf("%p | %c | %d\n", &buff, *buff, x);
	char* str_new_fn = calloc(32, 1);
	x = 16;
	chara = 128;
	sprintf(str_new_fn, "Division of %d by %d", chara, x);
	puts("sucess !");
	putchar('s');
	putchar('t');
	putchar('r');
	putchar(':');
	printf(" %s\n", str_new_fn);
	int add = new_fn(16, 128, str_new_fn);
	return add/6;//24
}

int while_fn(int how_many){
	int thing = 6;
	int idx = 0;
	if (thing == how_many){
		while (idx < how_many){
			printf("Loooooped %d times\n", idx);
			idx = idx+1;
		}
	}
	return 9;
}

/*
test other things
*/
int function_a(){
	int z = 3*30;
	int h = 10-7;
	int a = z+h;
	int b = a+9;
	int x = b/6;
	int w = x+3/2*5;
	char* str = "Hello world";
	printf("fn_b : %d\n", function_b(str));

	x = while_fn(6);
	printf("just in case : %d\n", x); // 9
	return w; // return 25
}


int main(){
	int ret_val = function_a();
	return ret_val;
}
