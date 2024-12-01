#include <stdio.h>
#include <stdlib.h>
#include <math.h>

int compare(const void* a, const void* b) {
	return (*(int*)a - *(int*)b);
}

int main() {
	FILE* file;
	int* xs = NULL;
	int* ys = NULL;
	int size = 0;
	int capacity = 128;
	int x, y;
	int i;
	long long sum = 0;
	
	xs = (int*)malloc(capacity *  sizeof(int));
	ys = (int*)malloc(capacity *  sizeof(int));

	if (xs == NULL || ys == NULL) {
		fprintf(stderr, "Memeroy malloc of xs and ys failed");
		return 1;
	}
	
	
	file = fopen("input.txt", "r");
	if (file == NULL) {
		fprintf(stderr, "Failed to open input.txt");
		free(xs);
		free(ys);
		return 1;
	}

	while(fscanf(file, "%d   %d", &x, &y) == 2) {
		if (size == capacity) {
			capacity *= 2;
			xs = (int*)realloc(xs, capacity * sizeof(int)); 
			ys = (int*)realloc(ys, capacity * sizeof(int));
			
			if (xs == NULL ||ys == NULL) {
				fprintf(stderr, "Could not realloc xs or ys");
				fclose(file);
				free(xs);
				free(ys);
				return 1;
			}
		}
		printf("x = %d, y = %d\n", x, y);

		xs[size] = x;
		ys[size] = y;
		size++;
	}

	fclose(file);

	qsort(xs, size, sizeof(int), compare);
	qsort(ys, size, sizeof(int), compare);


	for (i = 0; i < size; i++) {
		sum += abs(xs[i] - ys[i]);
	}


	printf("Sum = %lld\n", sum);

	free(xs);
	free(ys);
	return 0;
}
