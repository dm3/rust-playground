// from https://users.rust-lang.org/t/hashmap-performance/6476
#include <unordered_map>
#include <ext/pb_ds/assoc_container.hpp>
using namespace std;
static unordered_map<unsigned int, unsigned int> um;
static __gnu_pbds::cc_hash_table<unsigned int, unsigned int> ccht;
extern "C" void um_create() { }
extern "C" void ccht_create() { }
extern "C" void um_destroy() { um.clear(); }
extern "C" void ccht_destroy() { ccht.clear(); }
extern "C" void um_increment_count(unsigned int i) {
    try {
        auto pos = um.find(i);
        if (pos == um.end()) um[i] = 1;
        else ++pos->second;
    }
    catch (...) { }
}
extern "C" void ccht_increment_count(unsigned int i) {
    try {
        auto pos = ccht.find(i);
        if (pos == ccht.end()) ccht[i] = 1;
        else ++pos->second;
    }
    catch (...) { }
}
extern "C" unsigned int um_get_count(unsigned int i) {
    try {
        return um[i];
    }
    catch (...) { }
    return 0;
}
extern "C" unsigned int ccht_get_count(unsigned int i) {
    try {
        return ccht[i];
    }
    catch (...) { }
    return 0;
}
