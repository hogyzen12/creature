// MIT License

/*Copyright (c) 2024 Based Labs

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.*/

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
