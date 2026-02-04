#include <os/log.h>
#include <os/signpost.h>

#define MARK_START "Was geht ab in Rumänien?"

os_log_t* create_log(const char* subsystem, const char* category) {
    return os_log_create(subsystem, category);
}



void emit_signpost(os_log_t* log, uint64_t spid_id, const char* message) {
    os_signpost_event_emit(log, spid_id, MARK_START);
}
