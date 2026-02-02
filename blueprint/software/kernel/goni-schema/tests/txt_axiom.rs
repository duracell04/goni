use goni_schema::*;

#[test]
fn txt_axiom_holds_for_planes() {
    // Just constructing schemas ensures compile-time DSL guard ran.
    let _ = Docs::schema();
    let _ = Chunks::schema();
    let _ = Embeddings::schema();
    let _ = Requests::schema();
    let _ = Tasks::schema();
    let _ = RedactionProfiles::schema();
    let _ = RedactionEvents::schema();
    let _ = Prompts::schema();
    let _ = ContextItems::schema();
    let _ = MemoryEntries::schema();
    let _ = LlmCalls::schema();
    let _ = PlatformSignals::schema();
    let _ = PlatformCapabilities::schema();
    let _ = Metrics::schema();
}
