#include<stdio.h>
#include<sys/types.h>
#include<unistd.h>
#include<sys/wait.h>
#include<stdlib.h>

int main(int argc, char *argv[])
{
	int status;
    int* ptr;
	int res_size = atoi(argv[1]);
    ptr = (int*)malloc(res_size * 1024 * sizeof(int));
	for (int i = 0; i < 40000; i++) 
	{
		int pid = fork();
		if (pid < 0) return -1;
		if (pid == 0) return 0;
		waitpid(pid, &status, 0);
	}
    free(ptr);
	return 0;
}