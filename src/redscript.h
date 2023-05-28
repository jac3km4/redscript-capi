#include <stdint.h>

struct Decompilation;
struct ScriptBundle;

struct ScriptBundle* bundle_load(const char* path);
void bundle_free(struct ScriptBundle* bundle);

struct Decompilation* decompile_global(struct ScriptBundle* bundle, const char* name);
struct Decompilation* decompile_method(struct ScriptBundle* bundle, const char* parent, const char* name);

const char* decompilation_code(const struct Decompilation* decompilation);
const uint32_t* decompilation_line_mapping(const struct Decompilation* decompilation);
uint32_t decompilation_line_count(const struct Decompilation* decompilation);
void decompilation_free(struct Decompilation* bundle);
