#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <libgen.h>
#include <limits.h>
#include <unistd.h>

#define CC "../main"

bool is_c_file(char* file){
	int i = 0;
	while(file[i] != '.'){
		i+=1;
	}
	if (file[i+1] == 'c'){
		return true;
	}else{
		return false;
	}
}

int main(int argc, char* argv[]){
	
	char path[PATH_MAX];
    realpath(argv[0], path);

    char *dir = dirname(path);
    chdir(dir);

	char buffer[128];

    FILE *fp = popen("ls tests2/ | sort -n", "r");  // run command, read its stdout
    if (fp == NULL) {
        perror("popen failed");
        return 1;
    }

    // no 34, 53, 56, 57, 58, 59, 62, 63, 65, 66, 68, 69, 74, 98, 99, 138, 139, 140, 141, 145
    int test_to_do = 0;
    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
		if (test_to_do == 10){
			break;
		}
    	if (is_c_file(buffer)){
			// printf("Test N°%d:\n", test_to_do);
			// printf("OUTPUT: %s", buffer);
			char compile_cmd[128];
        	sprintf(compile_cmd, "%s tests2/%.*s tests2/assembled/%.*ss > /dev/null 2>&1", CC, (int)strlen(buffer)-1, buffer, (int)strlen(buffer)-2, buffer);
			// printf("BUILD COMMAND: %s\n", compile_cmd);
        	if (system(compile_cmd) == 0){
				char assemble_cmd[128];
	        	sprintf(assemble_cmd, "gcc tests2/assembled/%.*ss -o tests2/build/%.*s", (int)strlen(buffer)-2, buffer, (int)strlen(buffer)-3, buffer);
				// printf("ASSEMBLE COMMAND: %s\n", assemble_cmd);
	        	if (system(assemble_cmd) == 0){
					char execution_cmd[128];
	        		sprintf(execution_cmd, "./tests2/build/%.*s", (int)strlen(buffer)-3, buffer);
					// printf("EXECUTION COMMAND: %s\n", execution_cmd);
	        		// system(execution_cmd);
					
					FILE *test_fp = popen(execution_cmd, "r");
				    if (test_fp == NULL) {
				        perror("popen failed");
				        return 1;
				    }

					char expected_cmd[128];
	        		sprintf(expected_cmd, "cat tests2/%.*sexpect", (int)strlen(buffer)-2, buffer);
					// printf("EXPECTED COMMAND: %s\n", expected_cmd);
					FILE *expected_fp = popen(expected_cmd, "r");
				    if (expected_fp == NULL) {
				        perror("popen failed");
				        return 1;
				    }

					char program_out[128];
					char expected_out[128];
				    while (fgets(program_out, sizeof(program_out), test_fp) != NULL){
				    	fgets(expected_out, sizeof(expected_out), expected_fp);
				    	if (strcmp(expected_out, program_out) != 0){
				    		printf("program output does not match expected output (expected \"%s\", got \"%s\")!\n", expected_out, program_out);
				    		return 1;
				    	}
				    }
				    printf("Test \"%.*s\" : passed\n", (int)strlen(buffer)-3, buffer);
				    // printf("buff : %s\n", program_out);
    				// while (fgets(buffer, sizeof(buffer), fp) != NULL) {

				}
			}
			else{
				printf("Test \"%.*s\" : failed at compilation\n", (int)strlen(buffer)-3, buffer);
				return 1;
			}
			test_to_do++;
		}
	}

    pclose(fp);
    return 0;
}