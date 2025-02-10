#include <stdint.h>
extern void alloc_host(char* name_ptr, void* source_ptr, uint32_t source_size);
extern int get_host(char* name_ptr , void* dest_ptr); // Will return -1 if name doesn't exist in data
extern void print(char* string);

typedef struct {
    int x;
    int y;
} State;

State state = {0};
char* state_var = "state";

char* __name() {
    return "test_plug_c";
}

void __init() {
    print("[test_plug_c.init] Initializing...\n");
    if (get_host(state_var, &state) < 0) {
        state.x = 1;
        state.y = 5;
    }
    print("[test_plug_c.init] Initialization complete!\n");
}

void run() {
    print("[test_plug_c.run] state.x: ");
    for (int i = 0; i < state.x; ++i) print("x");
    print("\n[test_plug_c.run] state.y: ");
    for (int i = 0; i < state.y; ++i) print("y");
    print("\n");
    print("[test_plug_c.run] Mutating state.x...\n");
    state.x += 5;
    print("[test_plug_c.run] New value of state.x: ");
    for (int i = 0; i < state.x; ++i) print("x");
    print("\n");
}

void __reset() {
    print("[test_plug_c.reset] Storing `state`\n");
    alloc_host(state_var, &state, sizeof(State));    
    print("[test_plug_c.reset] Storage complete\n");
}
