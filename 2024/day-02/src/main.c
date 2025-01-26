#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<ctype.h>
#include<stdbool.h>

typedef struct Row {
	int size;
	int items[10];
} Row;

char file_peek(FILE *file_ptr) {
	int c = fgetc(file_ptr);
	ungetc(c, file_ptr);
	return c;
}

Row* read_line(FILE *file_ptr) {
	if (file_peek(file_ptr) == EOF) {
		return NULL;
	}

	Row *row = (Row*) malloc(sizeof(Row));
	if (row == NULL) {
		return NULL;
	}

	row->size = 0;
	char buf[3] = "\0";
	char c[2] = {'\0', '\0'};
	while ((*c = fgetc(file_ptr)) != EOF) {
		if (isdigit(*c)) {
			strcat(buf, c);
			continue;
		}

		int n = strtol(buf, NULL, 10);
		row->items[row->size] = n;
		row->size++;
		if (*c == '\n') {
			break;
		}
		*buf = '\0';
	}

	return row;
}

bool check_level_save(Row* report) {
	typedef enum {
		None,
		Inc,
		Dec
	} Variation;

	Variation trend = None;

	for (int i = 0; i < report->size - 1; i++) {
		int current = report->items[i];
		int next = report->items[i + 1];
		Variation current_variation = next > current ? Inc : Dec;
		if (trend == None) {
			trend = current_variation;
		} else if (trend != current_variation) {
			return false;
		}
		int difference = abs(next - current);
		if (difference > 3 || difference == 0) {
			return false;
		}
	}
	return true;
}

int main() {
	FILE *input_ptr = fopen("input.txt", "r");

	if (input_ptr == NULL) {
		printf("File can't be open\n");
		return EXIT_FAILURE;
	}

	int safe_reports = 0;
	Row *report;
	while ((report = read_line(input_ptr)) != NULL) {
		bool safe = check_level_save(report);
		for (int i = 0; i < report->size; i++) {
			printf("%d ", report->items[i]);
		}
		printf("%s\n", safe ? "safe" : "unsafe");
		free(report);
		if (safe) {
			safe_reports++;
		}
	}
	printf("Safe reports: %d\n", safe_reports);

	fclose(input_ptr);
	return EXIT_SUCCESS;
}
