#include "redscript.h"

void main() {
    struct ScriptBundle* bundle = bundle_load("C:\\Games\\Cyberpunk 2077\\r6\\cache\\final.redscripts.bk");
    struct Decompilation* dec = decompile_global(bundle, "LogStats;script_ref<String>");
    printf("Decompiled function:\n%s\n", decompilation_code(dec));

    decompilation_free(dec);
    bundle_free(bundle);
}
