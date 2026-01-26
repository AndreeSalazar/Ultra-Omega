#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <tuple>
#include <algorithm>
#include <iterator>

// C++14 Standard Library Features Demo
// - std::make_unique
// - std::shared_timed_mutex
// - std::integer_sequence
// - std::exchange
// - std::get_temporary_buffer
// - Standard user-defined literals for complex numbers

#include <mutex>
#include <utility>
#include <complex>
#include <chrono>

template<typename T, T... Ints>
struct integer_sequence {
    static constexpr std::size_t size() noexcept { return sizeof...(Ints); }
};

template<std::size_t... Ints>
using index_sequence = integer_sequence<std::size_t, Ints...>;

template<typename T, std::size_t N, T... Ints>
struct make_integer_sequence_impl {
    using type = typename make_integer_sequence_impl<T, N-1, N-1, Ints...>::type;
};

template<typename T, T... Ints>
struct make_integer_sequence_impl<T, 0, Ints...> {
    using type = integer_sequence<T, Ints...>;
};

template<typename T, T N>
using make_integer_sequence = typename make_integer_sequence_impl<T, N>::type;

template<std::size_t N>
using make_index_sequence = make_integer_sequence<std::size_t, N>;

template<typename... T>
using index_sequence_for = make_index_sequence<sizeof...(T)>;

class STLFeatures {
private:
    std::vector<std::unique_ptr<int>> resources;
    
public:
    void demonstrate_make_unique() {
        std::cout << "=== std::make_unique Demo ===" << std::endl;
        
        // C++14 std::make_unique
        auto ptr1 = std::make_unique<int>(42);
        auto ptr2 = std::make_unique<std::string>("C++14");
        
        std::cout << "Unique ptr value: " << *ptr1 << std::endl;
        std::cout << "Unique ptr string: " << *ptr2 << std::endl;
        
        // Array version
        auto arr_ptr = std::make_unique<int[]>(5);
        for (int i = 0; i < 5; ++i) {
            arr_ptr[i] = i * 10;
        }
        
        std::cout << "Array values: ";
        for (int i = 0; i < 5; ++i) {
            std::cout << arr_ptr[i] << " ";
        }
        std::cout << std::endl;
    }
    
    template<typename... Args>
    void print_with_index_sequence(Args&&... args) {
        auto seq = index_sequence_for<Args...>{};
        print_helper(seq, std::forward<Args>(args)...);
    }
    
private:
    template<std::size_t... Is, typename... Args>
    void print_helper(index_sequence<Is...>, Args&&... args) {
        // Fold expression simulation using initializer list
        auto dummy = {(std::cout << Is << ": " << args << std::endl, 0)...};
        (void)dummy; // Suppress unused variable warning
    }
    
public:
    void demonstrate_integer_sequence() {
        std::cout << "\n=== std::integer_sequence Demo ===" << std::endl;
        
        print_with_index_sequence("Hello", 42, 3.14, "C++14");
        
        // Generate sequence
        using my_seq = make_integer_sequence<int, 5>;
        std::cout << "Sequence size: " << my_seq::size() << std::endl;
    }
    
    void demonstrate_exchange() {
        std::cout << "\n=== std::exchange Demo ===" << std::endl;
        
        std::string old_value = "old";
        std::string new_value = std::exchange(old_value, "new");
        
        std::cout << "Old value: " << new_value << std::endl;
        std::cout << "Current value: " << old_value << std::endl;
        
        // Use in move assignment
        std::vector<int> source = {1, 2, 3, 4, 5};
        std::vector<int> dest = std::exchange(source, {});
        
        std::cout << "Source after exchange: " << source.size() << " elements" << std::endl;
        std::cout << "Destination: ";
        for (int val : dest) {
            std::cout << val << " ";
        }
        std::cout << std::endl;
    }
    
    void demonstrate_complex_literals() {
        std::cout << "\n=== Complex Literals Demo ===" << std::endl;
        
        // Standard user-defined literals for complex numbers (C++14)
        using namespace std::complex_literals;
        
        auto c1 = 1.0 + 2.0i;  // Using i literal
        auto c2 = 3.0 + 4.0i;
        
        auto sum = c1 + c2;
        auto product = c1 * c2;
        
        std::cout << "c1: " << c1 << std::endl;
        std::cout << "c2: " << c2 << std::endl;
        std::cout << "Sum: " << sum << std::endl;
        std::cout << "Product: " << product << std::endl;
    }
    
    void demonstrate_chrono_literals() {
        std::cout << "\n=== Chrono Literals Demo ===" << std::endl;
        
        using namespace std::chrono_literals;
        
        auto duration1 = 2h + 30min + 45s;  // C++14 chrono literals
        auto duration2 = 1500ms;
        
        std::cout << "Duration 1: " << duration1.count() << " seconds" << std::endl;
        std::cout << "Duration 2: " << duration2.count() << " milliseconds" << std::endl;
        
        auto start = std::chrono::steady_clock::now();
        // Simulate work
        std::this_thread::sleep_for(100ms);
        auto end = std::chrono::steady_clock::now();
        
        auto elapsed = end - start;
        std::cout << "Elapsed time: " << 
            std::chrono::duration_cast<std::chrono::milliseconds>(elapsed).count() 
            << " ms" << std::endl;
    }
};

int main() {
    STLFeatures demo;
    
    demo.demonstrate_make_unique();
    demo.demonstrate_integer_sequence();
    demo.demonstrate_exchange();
    demo.demonstrate_complex_literals();
    demo.demonstrate_chrono_literals();
    
    std::cout << "\n=== C++14 STL Features Complete ===" << std::endl;
    
    return 0;
}
