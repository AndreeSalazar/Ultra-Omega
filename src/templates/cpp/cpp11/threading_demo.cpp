#include <iostream>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <vector>
#include <atomic>
#include <chrono>

// C++11 Threading Demo
// - std::thread
// - std::mutex
// - std::condition_variable
// - std::atomic
// - std::future and std::promise (if needed)

class ThreadSafeCounter {
private:
    mutable std::mutex mtx;
    std::atomic<int> atomic_count{0};
    int count = 0;
    
public:
    void increment() {
        std::lock_guard<std::mutex> lock(mtx);
        ++count;
        ++atomic_count;
    }
    
    int get() const {
        std::lock_guard<std::mutex> lock(mtx);
        return count;
    }
    
    int get_atomic() const {
        return atomic_count.load();
    }
};

class ProducerConsumer {
private:
    std::vector<int> buffer;
    std::mutex buffer_mutex;
    std::condition_variable cv;
    const size_t max_size = 10;
    bool done = false;
    
public:
    void produce(int value) {
        std::unique_lock<std::mutex> lock(buffer_mutex);
        cv.wait(lock, [this] { return buffer.size() < max_size || done; });
        
        if (!done) {
            buffer.push_back(value);
            std::cout << "Produced: " << value << std::endl;
        }
        
        lock.unlock();
        cv.notify_one();
    }
    
    int consume() {
        std::unique_lock<std::mutex> lock(buffer_mutex);
        cv.wait(lock, [this] { return !buffer.empty() || done; });
        
        int value = 0;
        if (!buffer.empty()) {
            value = buffer.back();
            buffer.pop_back();
            std::cout << "Consumed: " << value << std::endl;
        }
        
        lock.unlock();
        cv.notify_one();
        return value;
    }
    
    void finish() {
        std::lock_guard<std::mutex> lock(buffer_mutex);
        done = true;
        cv.notify_all();
    }
};

void worker_function(ThreadSafeCounter& counter, int id) {
    for (int i = 0; i < 100; ++i) {
        counter.increment();
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    std::cout << "Worker " << id << " finished" << std::endl;
}

int main() {
    std::cout << "C++11 Threading Demo" << std::endl;
    
    // Demo 1: Multiple threads with shared counter
    ThreadSafeCounter counter;
    std::vector<std::thread> workers;
    
    // Create multiple threads
    for (int i = 0; i < 4; ++i) {
        workers.emplace_back(worker_function, std::ref(counter), i);
    }
    
    // Wait for all threads to complete
    for (auto& worker : workers) {
        worker.join();
    }
    
    std::cout << "Final count (mutex): " << counter.get() << std::endl;
    std::cout << "Final count (atomic): " << counter.get_atomic() << std::endl;
    
    // Demo 2: Producer-Consumer pattern
    std::cout << "\nProducer-Consumer Demo:" << std::endl;
    ProducerConsumer pc;
    
    std::thread producer([&pc]() {
        for (int i = 1; i <= 20; ++i) {
            pc.produce(i);
            std::this_thread::sleep_for(std::chrono::milliseconds(50));
        }
        pc.finish();
    });
    
    std::thread consumer([&pc]() {
        int total = 0;
        for (int i = 0; i < 20; ++i) {
            total += pc.consume();
            std::this_thread::sleep_for(std::chrono::milliseconds(30));
        }
        std::cout << "Total consumed: " << total << std::endl;
    });
    
    producer.join();
    consumer.join();
    
    return 0;
}
