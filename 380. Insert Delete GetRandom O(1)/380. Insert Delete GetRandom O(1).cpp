#include <vector>
#include <unordered_map>
#include <random>

class RandomizedSet {
private:
    std::vector<int> vec;
    std::unordered_map<int, int> map;
    std::mt19937 gen;
    std::uniform_int_distribution<> dis;

public:
    RandomizedSet() : gen(std::random_device{}()), dis(0, 0) {}
    
    bool insert(int val) {
        if (map.count(val)) {
            return false;
        }
        map[val] = vec.size();
        vec.push_back(val);
        dis = std::uniform_int_distribution<>(0, vec.size() - 1);
        return true;
    }
    
    bool remove(int val) {
        if (!map.count(val)) {
            return false;
        }
        int last = vec.back();
        int idx = map[val];
        vec[idx] = last;
        map[last] = idx;
        vec.pop_back();
        map.erase(val);
        dis = std::uniform_int_distribution<>(0, vec.size() - 1);
        return true;
    }
    
    int getRandom() {
        return vec[dis(gen)];
    }
};