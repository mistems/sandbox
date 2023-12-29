#include <iostream>
#include <string>
#include <utility>
class FizzBuzz_number {
public:
    explicit FizzBuzz_number(std::string str) : max(std::move(str)) {};
    void print() {
        std::string buf;
        if (getDigitSum() % 3 == 0) {
            buf += "Fizz ";
        }
        if (*current.rbegin() == '0' || *current.rbegin() == '5') {
            buf += "Buzz";
        }
        if (buf.empty()) {
            buf = current;
        }
        std::cout << buf << "\n";
        incr();
    }
    int getDigitSum() const {
        int result = 0;
        for (const auto& i : current) {
            result += i - '0';
        }
        return result;
    }
    void print_all() {
        while (max != current) {
            print();
        }
        print();
    }
private:
    void incr() {
        auto now = current.rbegin();
        (*now)++;
        while (*now == '9'+1) {
            *now = '0';
            if (now+1 == current.rend()) {
                current = "1" + current;
                return;
            }
            ++now;
            (*now)++;
        }
    }
    const std::string max;
    std::string current = "1";
};

int main() {
    std::string input;
    std::cin >> input;
    FizzBuzz_number num(input);
    num.print_all();
}
