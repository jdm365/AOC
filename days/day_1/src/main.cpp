#include <iostream>
#include <sstream>
#include <fstream>
#include <vector>
#include <string>
#include <regex>
#include <chrono>
#include <omp.h>


void init_regexes(std::vector<std::regex>& regexes) {
	regexes.push_back(std::regex("one"));
	regexes.push_back(std::regex("two"));
	regexes.push_back(std::regex("three"));
	regexes.push_back(std::regex("four"));
	regexes.push_back(std::regex("five"));
	regexes.push_back(std::regex("six"));
	regexes.push_back(std::regex("seven"));
	regexes.push_back(std::regex("eight"));
	regexes.push_back(std::regex("nine"));
}

void replace_string_with_digit(std::string& line, std::vector<std::regex>& regexes) {
	line = std::regex_replace(line, regexes[0], "one1one");
	line = std::regex_replace(line, regexes[1], "two2two");
	line = std::regex_replace(line, regexes[2], "three3three");
	line = std::regex_replace(line, regexes[3], "four4four");
	line = std::regex_replace(line, regexes[4], "five5five");
	line = std::regex_replace(line, regexes[5], "six6six");
	line = std::regex_replace(line, regexes[6], "seven7seven");
	line = std::regex_replace(line, regexes[7], "eight8eight");
	line = std::regex_replace(line, regexes[8], "nine9nine");
}

std::vector<std::vector<int>> read_file(std::string filename, std::vector<std::regex>& regexes) {
	std::vector<std::vector<int>> data;
	data.reserve(1000);

	std::ifstream file(filename);
	std::string line;
	std::string new_line;
	std::regex re("\\d");

	while (std::getline(file, line)) {
		std::vector<int> row;
		replace_string_with_digit(line, regexes);
		std::sregex_iterator next(line.begin(), line.end(), re);
		std::sregex_iterator end;
		while (next != end) {
			std::smatch match = *next;
			row.push_back(std::stoi(match.str()));
			next++;
		}
		data.push_back(row);
	}
	return data;
}

int sum_lines(std::vector<std::vector<int>> data) {
	int sum = 0;
	
	#pragma omp simd reduction(+:sum)
	for (int line_idx = 0; line_idx < (int)data.size(); ++line_idx) {
		int value = data[line_idx].front() * 10 + data[line_idx].back();
		sum += value;
	}
	return sum;
}


int main() {
	auto init = std::chrono::steady_clock::now();
	std::vector<std::regex> regexes;
	init_regexes(regexes);

	std::vector<std::vector<int>> data = read_file("res/puzzle_input.txt", regexes);

	int sum = sum_lines(data);

	auto end = std::chrono::steady_clock::now();
	auto diff = end - init;
	std::cout << "Time: " << std::chrono::duration <double, std::milli> (diff).count() << " ms" << std::endl;

	std::cout << "Answer: " << sum << std::endl;
	return 0;
}
