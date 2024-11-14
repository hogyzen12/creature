use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref ASCII_TEMPLATES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        
        // Neural network pattern
        m.insert("neural", r#"
    ┌─[Input]─┐    ┌─(Process)─┐    ┌─[Output]─┐
    └────┬────┘    └─────┬─────┘    └────┬────┘
         │               │               │
         └───────── Feedback Loop ───────┘
    "#);

        // Tree structure
        m.insert("tree", r#"
           ┌──[Root]──┐
           │          │
     ┌─────┴────┐    └────┐
     │          │         │
   [Node A]  [Node B]  [Node C]
     │          │         │
  ┌──┴──┐   ┌──┴──┐   ┌──┴──┐
  │     │   │     │   │     │
[Leaf] [Leaf] [Leaf] [Leaf] [Leaf]
    "#);

        // Circuit pattern
        m.insert("circuit", r#"
    ┌────────[Module]────────┐
    │     ┌──────────┐      │
    │  ───┤ Process  ├───   │
    │     └──────────┘      │
    └──────────┬───────────┘
               │
    ┌──────────┼───────────┐
    │     ┌────┴─────┐     │
    │     │ Output   │     │
    │     └──────────┘     │
    └────────────────────┘
    "#);

        // Feedback loop
        m.insert("feedback", r#"
    ┌───────────────────┐
    │    ┌─────────┐    │←──────┐
    │ ───┤ Process ├─── │       │
    │    └─────────┘    │       │
    └────────┬──────────┘       │
             ↓                   │
    ┌────────┴──────────┐       │
    │   ┌──────────┐    │       │
    │   │ Feedback │    ├───────┘
    │   └──────────┘    │
    └───────────────────┘
    "#);

        // Thought chain
        m.insert("chain", r#"
    ┌─────────┐     ┌─────────┐     ┌─────────┐
    │ Prior   │════>│ Current │════>│  Next   │
    └────┬────┘     └────┬────┘     └────┬────┘
         │               │               │
         └──────── Evolution ────────────┘
    "#);

        m
    };
}

pub fn get_ascii_template(template_name: &str) -> Option<&'static str> {
    ASCII_TEMPLATES.get(template_name).copied()
}

pub fn list_templates() -> Vec<&'static str> {
    ASCII_TEMPLATES.keys().copied().collect()
}
