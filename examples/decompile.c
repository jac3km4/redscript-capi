#include "redscript.h"

void main() {
    struct ScriptBundle* bundle = bundle_load("C:\\Games\\Cyberpunk 2077\\r6\\cache\\final.redscripts.bk");
    if (bundle == NULL) {
        printf("Failed to load bundle\n");
        return;
    }
    struct Decompilation* dec = decompile_global(bundle, "LogStats;script_ref<String>");
    if (dec == NULL) {
        printf("Failed to decompile\n");
        return;
    }

    printf("Decompiled function:\n%s\n", decompilation_code(dec));

    decompilation_free(dec);
    bundle_free(bundle);
}
