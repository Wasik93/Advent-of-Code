#include <iostream>
#include <vector>

using namespace std;

int solve_part1(vector<char>&spring, vector<int>&numbers, int current_char, int current_number, int value, string a);

int dot_case(vector<char>&spring, vector<int>&numbers, int current_char, int current_number, int value, string a){
    if(value != -1 && numbers[current_number] != value){
        return 0;
    }
    else if (value == -1){
        return solve_part1(spring, numbers, current_char + 1, current_number, -1, a);
    }
    else{
        return solve_part1(spring, numbers, current_char + 1, current_number + 1, -1, a);
    }
}

int tag_case(vector<char>&spring, vector<int>&numbers, int current_char, int current_number, int value, string a){
    if (value == -1){
        value = 0;
    }
    if(numbers[current_number] <= value){
        return 0;
    }
    return solve_part1(spring, numbers, current_char + 1, current_number, value + 1, a);
}


int solve_part1(vector<char>&spring, vector<int>&numbers, int current_char, int current_number, int value, string a){
    if(current_char >= spring.size()){
        if(a[current_char - 1] == '#' && current_number == numbers.size() - 2 && value == numbers[current_number]){
            //cout << a << '\n';
            return 1;
        }
        else if (a[current_char - 1] != '#' && current_number == numbers.size() - 1){
            //cout << a << '\n';
            return 1;
        }
        return 0;
    }
    if(current_number > numbers.size()){
        return 0;
    }
    if(spring[current_char] == '?'){
        int d = dot_case(spring, numbers, current_char, current_number, value, a+'.');
        int t = tag_case(spring, numbers, current_char, current_number, value, a+'#');
        //cout << d << " + " << t << " - dt "<< current_char << '\n';
        return d + t; 
    }
    if(spring[current_char] == '.'){
        int d = dot_case(spring, numbers, current_char, current_number, value, a+'.');
        //cout << d << " - d "<< current_char << '\n';
        return d;
    }
    if(spring[current_char] == '#'){
        int t = tag_case(spring, numbers, current_char, current_number, value, a+'#');
        //cout << t << " - t "<< current_char << '\n';
        return t;
    }
    return 0;
}

int solve_part2(vector<char>&spring, vector<int>&numbers, int current_char, int current_number, int value, string a){
    return 0;
}

void part1(){
    int sum_val = 0;
    for(int i = 0; i < 1000; i++){
        string first;
        string second;
        vector<int> numbers;
        vector<char> spring;
        cin >> first;
        cin >> second;
        int number = 0;
        for(int j = 0; j < first.length(); j++){
            spring.push_back(first[j]);
        }
        for(int j = 0; j < second.length(); j++){
            if(second[j] >= '0' && second[j] <= '9'){
                number = number * 10 + (second[j] - '0');
            }
            else{
                numbers.push_back(number);
                number = 0;
            }
        }
        numbers.push_back(number);
        numbers.push_back(-1);
        int tmp_val = solve_part1(spring, numbers, 0, 0, -1, "");
        sum_val += tmp_val;
        //cout << tmp_val << '\n';
    }
    cout << sum_val << '\n';
}

void part2(){
    int sum_val = 0;
    cout << sum_val <<'\n';
}

int main(){
    cout << "Day 12\n";
    cout << "Part 1\n";
    part1();
    cout << "Part 2\n";
    part2();
    return 0;
}