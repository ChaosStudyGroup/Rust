#include <stdio.h>
#include <stdlib.h>
void show(char c){
	printf("input : %d\n", c);
}

void show_str(char *c){
	printf("input: %s\n",c);
}

void show_str_with_free(char *c){
	printf("input: %s\n",c);
	free(c);
}

char *get_str(){
	return "hello world";
}
