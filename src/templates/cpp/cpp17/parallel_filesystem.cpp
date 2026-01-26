#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>
#include <execution>
#include <filesystem>
#include <memory>
#include <string_view>

// C++17 Parallel Algorithms and Filesystem Demo
// - parallel execution policies
// - std::filesystem
// - std::reduce
// - std::transform_reduce
// - parallel sorting
// - parallel for_each

namespace fs = std::filesystem;

class ParallelProcessing {
private:
    std::vector<int> large_dataset;
    
public:
    ParallelProcessing() {
        // Generate large dataset
        large_dataset.resize(1000000);
        std::iota(large_dataset.begin(), large_dataset.end(), 1);
    }
    
    void demonstrate_parallel_algorithms() {
        std::cout << "=== Parallel Algorithms Demo ===" << std::endl;
        
        // Sequential vs Parallel performance comparison
        auto start = std::chrono::high_resolution_clock::now();
        
        // Sequential sort
        auto data_seq = large_dataset;
        std::sort(std::execution::seq, data_seq.begin(), data_seq.end());
        
        auto end_seq = std::chrono::high_resolution_clock::now();
        auto duration_seq = std::chrono::duration_cast<std::chrono::milliseconds>(end_seq - start);
        
        // Parallel sort
        start = std::chrono::high_resolution_clock::now();
        
        auto data_par = large_dataset;
        std::sort(std::execution::par, data_par.begin(), data_par.end());
        
        auto end_par = std::chrono::high_resolution_clock::now();
        auto duration_par = std::chrono::duration_cast<std::chrono::milliseconds>(end_par - start);
        
        std::cout << "Sequential sort time: " << duration_seq.count() << " ms" << std::endl;
        std::cout << "Parallel sort time: " << duration_par.count() << " ms" << std::endl;
        
        // Verify results are the same
        bool same = std::equal(data_seq.begin(), data_seq.end(), data_par.begin());
        std::cout << "Results identical: " << (same ? "Yes" : "No") << std::endl;
    }
    
    void demonstrate_reduce_operations() {
        std::cout << "\n=== Reduce Operations Demo ===" << std::endl;
        
        // std::reduce (parallel reduction)
        auto sum_seq = std::reduce(std::execution::seq, large_dataset.begin(), large_dataset.end());
        auto sum_par = std::reduce(std::execution::par, large_dataset.begin(), large_dataset.end());
        
        std::cout << "Sequential sum: " << sum_seq << std::endl;
        std::cout << "Parallel sum: " << sum_par << std::endl;
        
        // Custom reduction
        auto product = std::reduce(
            std::execution::par,
            large_dataset.begin(),
            large_dataset.end(),
            1L,
            std::multiplies<long>()
        );
        
        std::cout << "Product (first 100 elements): ";
        auto product_100 = std::reduce(
            std::execution::par,
            large_dataset.begin(),
            large_dataset.begin() + 100,
            1L,
            std::multiplies<long>()
        );
        std::cout << product_100 << std::endl;
        
        // transform_reduce
        auto sum_of_squares = std::transform_reduce(
            std::execution::par,
            large_dataset.begin(),
            large_dataset.end(),
            0L,
            std::plus<>(),
            [](long x) { return x * x; }
        );
        
        std::cout << "Sum of squares: " << sum_of_squares << std::endl;
    }
    
    void demonstrate_parallel_for_each() {
        std::cout << "\n=== Parallel For Each Demo ===" << std::endl;
        
        std::vector<std::string> words(100000, "hello");
        
        // Transform words in parallel
        std::for_each(
            std::execution::par,
            words.begin(),
            words.end(),
            [](std::string& word) {
                word += "_processed";
            }
        );
        
        std::cout << "Processed " << words.size() << " words in parallel" << std::endl;
        std::cout << "First word: " << words[0] << std::endl;
        std::cout << "Last word: " << words.back() << std::endl;
    }
    
    void demonstrate_filesystem() {
        std::cout << "\n=== std::filesystem Demo ===" << std::endl;
        
        // Current path
        auto current_path = fs::current_path();
        std::cout << "Current path: " << current_path << std::endl;
        
        // Create directory structure
        fs::path demo_dir = "demo_files";
        fs::path sub_dir = demo_dir / "subdir";
        
        try {
            // Create directories
            fs::create_directories(sub_dir);
            std::cout << "Created directories: " << demo_dir << std::endl;
            
            // Create files
            std::vector<fs::path> files = {
                demo_dir / "file1.txt",
                demo_dir / "file2.txt",
                sub_dir / "file3.txt"
            };
            
            for (const auto& file : files) {
                std::ofstream out(file);
                out << "Content for " << file.filename().string() << std::endl;
            }
            
            // List directory contents
            std::cout << "\nDirectory contents:" << std::endl;
            for (const auto& entry : fs::directory_iterator(demo_dir)) {
                std::cout << "  " << entry.path().filename() 
                          << " (" << (entry.is_directory() ? "dir" : "file") << ")" << std::endl;
            }
            
            // Recursive directory iteration
            std::cout << "\nRecursive directory listing:" << std::endl;
            for (const auto& entry : fs::recursive_directory_iterator(demo_dir)) {
                std::cout << "  " << entry.path() << std::endl;
            }
            
            // File operations
            auto file1 = files[0];
            if (fs::exists(file1)) {
                auto file_size = fs::file_size(file1);
                auto last_write = fs::last_write_time(file1);
                
                std::cout << "\nFile info for " << file1.filename() << ":" << std::endl;
                std::cout << "  Size: " << file_size << " bytes" << std::endl;
                std::cout << "  Last modified: " << last_write.time_since_epoch().count() << std::endl;
            }
            
            // Path operations
            std::cout << "\nPath operations:" << std::endl;
            std::cout << "  Parent: " << file1.parent_path() << std::endl;
            std::cout << "  Filename: " << file1.filename() << std::endl;
            std::cout << "  Stem: " << file1.stem() << std::endl;
            std::cout << "  Extension: " << file1.extension() << std::endl;
            
            // Clean up
            fs::remove_all(demo_dir);
            std::cout << "\nCleaned up demo directory" << std::endl;
            
        } catch (const fs::filesystem_error& e) {
            std::cerr << "Filesystem error: " << e.what() << std::endl;
        }
    }
    
    void demonstrate_parallel_copy() {
        std::cout << "\n=== Parallel Copy Algorithm Demo ===" << std::endl;
        
        std::vector<int> source(1000000);
        std::iota(source.begin(), source.end(), 1);
        
        std::vector<int> destination(source.size());
        
        // Parallel copy
        auto start = std::chrono::high_resolution_clock::now();
        
        std::copy(
            std::execution::par,
            source.begin(),
            source.end(),
            destination.begin()
        );
        
        auto end = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
        
        std::cout << "Parallel copy time: " << duration.count() << " ms" << std::endl;
        std::cout << "First element: " << destination[0] << std::endl;
        std::cout << "Last element: " << destination.back() << std::endl;
    }
};

int main() {
    std::cout << "C++17 Parallel Algorithms & Filesystem Demo" << std::endl;
    std::cout << "============================================" << std::endl;
    
    ParallelProcessing demo;
    
    demo.demonstrate_parallel_algorithms();
    demo.demonstrate_reduce_operations();
    demo.demonstrate_parallel_for_each();
    demo.demonstrate_filesystem();
    demo.demonstrate_parallel_copy();
    
    std::cout << "\n=== C++17 Parallel & Filesystem Features Complete ===" << std::endl;
    
    return 0;
}
