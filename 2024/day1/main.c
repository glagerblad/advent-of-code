#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef struct HashNode {
	int key;
	int count;
	struct HashNode *next;
} HashNode;

typedef struct {
	int size;
	HashNode** table;
} HashMap;

HashMap* create_hashmap(int size) {
	HashMap* map = (HashMap*)malloc(sizeof(HashMap));
	map->size = size;
	map->table = (HashNode**)calloc(size, sizeof(HashNode*));
	return map;
}

int hashing_function(HashMap* map, int key) {
	return abs(key) % map->size;
}

void insert_hashmap(HashMap* map, int key) {
	int hash_index = hashing_function(map, key);
	HashNode* node = map->table[hash_index];
	
	while (node) {
		if (node->key == key) {
			node->count += 1;
			return;
		}
		node = node->next;
	}

	node = (HashNode*)malloc(sizeof(HashNode));
	node->key = key;
	node->count = 1;
	node->next = map->table[hash_index];
	map->table[hash_index] = node;
}

int get_count_hashmap(HashMap* map, int key) {
	int hash_index = hashing_function(map, key);
	HashNode* node = map->table[hash_index];
	
	while (node) {
		if (node->key == key) return node->count;
		node = node->next;
	}
	return 0;
}

void free_hashmap(HashMap* map) {
	int i;
	for (i = 0; i < map->size; i++) {
		HashNode* node = map->table[i];
		while (node) {
			HashNode* temp_node = node;
			node = node->next;
			free(temp_node);
		}
	}
	free(map->table);
	free(map);
}

int main() {
	FILE* file;
	int* xs = NULL;
	HashMap* ymap = NULL;
	int size = 0;
	int capacity = 1000;
	int x, y;
	int i;
	int value1;
	int value2;
	long long sum = 0;
	
	xs = (int*)malloc(capacity *  sizeof(int));
	ymap = create_hashmap(capacity);

	if (xs == NULL) {
		fprintf(stderr, "Memeroy malloc of xs failed");
		return 1;
	}
	
	if (ymap == NULL) {
		fprintf(stderr, "Memeroy malloc of ymap failed");
		return 1;
	}
	
	file = fopen("input.txt", "r");
	if (file == NULL) {
		fprintf(stderr, "Failed to open input.txt");
		free(xs);
		free_hashmap(ymap);
		return 1;
	}

	while(fscanf(file, "%d   %d", &x, &y) == 2) {
		if (size == capacity) {
			capacity *= 2;
			xs = (int*)realloc(xs, capacity * sizeof(int)); 
			
			if (xs == NULL) {
				fprintf(stderr, "Could not realloc xs or ys");
				fclose(file);
				free(xs);
				free_hashmap(ymap);
				return 1;
			}
		}

		xs[size] = x;
		insert_hashmap(ymap, y);
		size++;
	}

	fclose(file);


	for (i = 0; i < size; i++) {
		value1 = xs[i];
		value2 = get_count_hashmap(ymap, value1);
		sum += value1 * value2;
		printf("value 1 = %d * value 2 = %d :: Sum = %lld\n", value1, value2, sum);

	}


	printf("Sum = %lld\n", sum);

	free(xs);
	free_hashmap(ymap);
	return 0;
}
