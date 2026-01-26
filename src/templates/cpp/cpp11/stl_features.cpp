#include <iostream>
#include <vector>
#include <memory>
#include <algorithm>
#include <iterator>
#include <functional>

// C++11 STL Features Demo
// - std::array
// - std::forward_list
// - std::unordered_map/set
// - std::move algorithm
// - std::begin/end
// - std::function

#include <array>
#include <forward_list>
#include <unordered_map>
#include <unordered_set>

class STLFeatures {
private:
    std::unordered_map<std::string, int> word_counts;
    std::unordered_set<int> unique_numbers;
    
public:
    void demonstrate_array() {
        std::cout << "=== std::array Demo ===" << std::endl;
        
        std::array<int, 5> arr = {1, 2, 3, 4, 5};
        
        // std::begin/std::end
        std::for_each(std::begin(arr), std::end(arr), 
                     [](int x) { std::cout << x << " "; });
        std::cout << std::endl;
        
        // Range-based for
        for (const auto& val : arr) {
            std::cout << val * 2 << " ";
        }
        std::cout << std::endl;
    }
    
    void demonstrate_forward_list() {
        std::cout << "\n=== std::forward_list Demo ===" << std::endl;
        
        std::forward_list<std::string> words = {"hello", "world", "cpp11"};
        
        // Insert before
        words.push_front("modern");
        
        // Using std::function
        std::function<void(const std::string&)> print_word = 
            [](const std::string& word) {
                std::cout << word << " ";
            };
        
        std::for_each(words.begin(), words.end(), print_word);
        std::cout << std::endl;
        
        // Count words
        for (const auto& word : words) {
            word_counts[word]++;
        }
    }
    
    void demonstrate_unordered() {
        std::cout << "\n=== std::unordered Containers Demo ===" << std::endl;
        
        // unordered_map
        word_counts["modern"] = 1;
        word_counts["cpp"] = 2;
        word_counts["features"] = 3;
        
        std::cout << "Word counts:" << std::endl;
        for (const auto& pair : word_counts) {
            std::cout << pair.first << ": " << pair.second << std::endl;
        }
        
        // unordered_set
        std::vector<int> numbers = {1, 2, 3, 2, 4, 5, 3, 1};
        unique_numbers.insert(numbers.begin(), numbers.end());
        
        std::cout << "\nUnique numbers: ";
        for (int num : unique_numbers) {
            std::cout << num << " ";
        }
        std::cout << std::endl;
    }
    
    void demonstrate_move_algorithms() {
        std::cout << "\n=== Move Algorithms Demo ===" << std::endl;
        
        std::vector<std::string> source = {"move", "these", "strings", "around"};
        std::vector<std::string> destination;
        
        // std::move_backward
        destination.resize(source.size());
        std::move_backward(source.begin(), source.end(), destination.end());
        
        std::cout << "After move_backward:" << std::endl;
        for (const auto& str : destination) {
            std::cout << str << " ";
        }
        std::cout << std::endl;
        
        // Check if source is moved from
        std::cout << "Source strings (may be empty): ";
        for (const auto& str : source) {
            std::cout << "'" << str << "' ";
        }
        std::cout << std::endl;
    }
};

int main() {
    STLFeatures demo;
    
    demo.demonstrate_array();
    demo.demonstrate_forward_list();
    demo.demonstrate_unordered();
    demo.demonstrate_move_algorithms();
    
    std::cout << "\n=== C++11 STL Features Complete ===" << std::endl;
    
    return 0;
}
