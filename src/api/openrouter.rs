use crate::models::types::{
    CellContext, RealTimeContext, Thought, Plan, PlanNode, PlanNodeStatus, PlanStatus,
    DimensionalPosition,
};
use crate::models::KnowledgeBase;
use chrono::Utc;
use rand::Rng;
use reqwest;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

struct CachedContext {
    context: RealTimeContext,
    timestamp: SystemTime,
}

pub struct OpenRouterClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    context_cache: Arc<Mutex<Option<CachedContext>>>,
    knowledge_base: Arc<Mutex<Option<KnowledgeBase>>>,
}

impl OpenRouterClient {
    pub fn new(api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        if api_key.trim().is_empty() {
            return Err("OPENROUTER_API_KEY cannot be empty".into());
        }

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(crate::models::constants::API_TIMEOUT_SECS))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Ok(Self {
            client,
            api_key,
            base_url: "https://openrouter.ai/api/v1".to_string(),
            context_cache: Arc::new(Mutex::new(None)),
            knowledge_base: Arc::new(Mutex::new(None)),
        })
    }

    fn estimate_tokens(text: &str) -> usize {
        text.len() / 4
    }

    fn get_max_tokens_for_model(model: &str) -> usize {
        match model {
            "x-ai/grok-beta" => crate::models::constants::MAX_TOKENS_GROK,
            _ => 6048,
        }
    }

    async fn get_trending_topics(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let _rng = rand::thread_rng();
        // watch for grok limits 992 
        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": "x-ai/grok-beta",
                "messages": [{
                    "role": "user",
                    "content": r#"
                        Analyze technical developments from the last 72 hours across multiple domains.
                        Focus on posts from accounts with <0.01% following on technical platforms.

                        Required Analysis Vectors:

                        1. TECHNICAL DEVELOPMENTS
                        For each development:
                        EVENT:
                        TIMESTAMP: [Must be within last 72h, exact to minute]
                        SOURCE: 
                        - Repository URL + commit hash
                        - Research paper DOI
                        - Technical blog post URL
                        - System deployment log
                        MENTIONED BY: [Technical accounts only]
                        - Individual researchers (<5k followers)
                        - Research lab accounts
                        - Open source maintainers
                        - System architects
                        - Technical leads
                        TECHNICAL DETAILS:
                        - Implementation specifics
                        - Architecture changes
                        - Performance metrics
                        - Resource requirements
                        - Integration points
                        VALIDATION:
                        - Reproducible results
                        - Test coverage
                        - Benchmark data
                        - Error rates
                        - System logs

                        2. RESEARCH DEVELOPMENTS
                        For each development:
                        PAPER:
                        TIMESTAMP: [Publication/preprint within 72h]
                        DOI/arXiv:
                        AUTHORS:
                        INSTITUTION:
                        KEY FINDINGS:
                        - Methodology
                        - Results
                        - Limitations
                        - Future work
                        VALIDATION:
                        - Experimental setup
                        - Data collection
                        - Statistical analysis
                        - Reproducibility steps

                        3. SYSTEM DEPLOYMENTS
                        For each deployment:
                        SYSTEM:
                        TIMESTAMP: [Deployment within 72h]
                        ORGANIZATION:
                        SCALE:
                        ARCHITECTURE:
                        PERFORMANCE:
                        - Latency metrics
                        - Throughput data
                        - Resource usage
                        - Error rates
                        VALIDATION:
                        - Monitoring logs
                        - Health metrics
                        - Alert history
                        - Recovery data

                        4. TECHNICAL DISCUSSIONS
                        For each significant thread:
                        TOPIC:
                        TIMESTAMP: [Discussion within 72h]
                        PARTICIPANTS: [Technical roles only]
                        KEY POINTS:
                        - Technical challenges
                        - Proposed solutions
                        - Implementation details
                        - Resource considerations
                        VALIDATION:
                        - Code examples
                        - Benchmark results
                        - Test cases
                        - Performance data

                        EVIDENCE REQUIREMENTS:
                        1. Technical Validation
                           - Public repository commits
                           - Published papers
                           - System logs
                           - Performance metrics
                           - Test results
                           - Deployment data

                        2. Source Requirements
                           - Technical accounts only (<0.01% following)
                           - Research institutions
                           - Open source maintainers
                           - System architects
                           - Technical leads
                           - Individual researchers

                        3. Time Constraints
                           - All events within last 72h
                           - Exact timestamps required
                           - Time zone specified
                           - Update frequency noted

                        4. Data Requirements
                           - Raw metrics
                           - Benchmark results
                           - Error rates
                           - Resource usage
                           - System logs
                           - Test coverage

                        Return comprehensive analysis of developments from last 72h.
                        Format as structured events with all required fields.
                        Prioritize technical depth over quantity.
                        "#
                }],
                "temperature": 0.9,
                "max_tokens": Self::get_max_tokens_for_model("x-ai/grok-beta")
            }))
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;
        let response_text = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
            
        // Parse and extract events with additional validation
        let mut events = Vec::new();
        let mut current_event = String::new();
        let mut in_event = false;
        
        for line in response_text.lines() {
            let line = line.trim();
            if line.starts_with("EVENT:") {
                if !current_event.is_empty() {
                    events.push(current_event.trim().to_string());
                }
                current_event = line.trim_start_matches("EVENT:").trim().to_string();
                in_event = true;
            } else if in_event && !line.is_empty() {
                current_event.push_str("\n");
                current_event.push_str(line);
            }
        }
        
        if !current_event.is_empty() {
            events.push(current_event.trim().to_string());
        }

        Ok(events)
    }

    pub async fn gather_real_time_context(
        &self,
        cell_thoughts: Option<Vec<String>>,
    ) -> Result<RealTimeContext, Box<dyn std::error::Error>> {
        if let Some(cached) = self.context_cache.lock().unwrap().as_ref() {
            if cached
                .timestamp
                .elapsed()
                .unwrap_or(Duration::from_secs(360))
                < Duration::from_secs(300)
            {
                return Ok(cached.context.clone());
            }
        }

        let trending_topics = self.get_trending_topics().await?;

        let thoughts_context = if let Some(thoughts) = cell_thoughts {
            format!(
                "\nRecent colony thoughts:\n{}",
                thoughts
                    .iter()
                    .map(|t| format!("- {}", t))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        } else {
            String::new()
        };

        let context_query = format!(
            r#"
            System Analysis Framework:

            CONTEXT:
            {}
            {}

            1. POWER DYNAMICS ANALYSIS
            - Disrupted hierarchies
            - Emergent control mechanisms
            - Resource flow shifts
            - Influence network changes

            2. SYSTEM BOUNDARIES ANALYSIS
            - Interface mutations
            - Boundary dissolutions
            - Unexpected connections
            - Integration points

            3. TEMPORAL PATTERNS ANALYSIS
            - Evolution trajectories
            - Decay patterns
            - Cyclic behaviors
            - Timescale interactions

            4. EMERGENCE ANALYSIS
            - Unexpected properties
            - Feedback loops
            - Pattern formation
            - System surprises

            5. ASSUMPTION ANALYSIS
            - Questionable constraints
            - Hidden potentials
            - Artificial limitations
            - Missed connections

            Required per analysis vector:
            1. Evidence Trail:
               - Active repository commits (72h)
               - Researcher activities (72h)
               - Experiment results (72h)
               - Deployment metrics (72h)

            2. Power Implications:
               - Control shifts
               - Resource reallocations
               - Relationship changes
               - Influence flows

            3. System Effects:
               - Boundary changes
               - Interface formations
               - Pattern emergences
               - Capability evolutions

            4. Technical Details:
               - Architecture diagrams
               - Integration points
               - Data flows
               - Control mechanisms

            Format as:
            VECTOR: [Analysis Type]
            CONVENTIONAL VIEW: [Standard interpretation]
            RADICAL INSIGHT: [Non-obvious observation]
            EVIDENCE: [Concrete proof points]
            IMPLICATIONS: [Cascading effects]
            DIAGRAM: [ASCII representation]

            Categories labeled and structured hierarchically.
        "#,
            trending_topics
                .iter()
                .map(|t| format!("- {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            thoughts_context
        );

        let response = self.query_llm(&context_query).await?;
        let parsed = self.parse_context_response(&response)?;

        let context = RealTimeContext {
            timestamp: Utc::now(),
            market_trends: parsed.get("market_trends").cloned().unwrap_or_default(),
            current_events: parsed.get("current_events").cloned().unwrap_or_default(),
            technological_developments: parsed
                .get("technological_developments")
                .cloned()
                .unwrap_or_default(),
            user_interactions: parsed.get("user_interactions").cloned().unwrap_or_default(),
            environmental_data: HashMap::new(),
            mission_progress: Vec::new(),
        };

        *self.context_cache.lock().unwrap() = Some(CachedContext {
            context: context.clone(),
            timestamp: SystemTime::now(),
        });

        Ok(context)
    }

    pub async fn generate_contextual_thoughts_batch(
        &self,
        cell_contexts: &[(Uuid, &CellContext)],
        real_time_context: &RealTimeContext,
        colony_mission: &str,
    ) -> Result<HashMap<Uuid, (String, f64, Vec<String>)>, Box<dyn std::error::Error>> {
        let sub_batch_size = 3;
        let mut all_results = HashMap::new();

        for chunk in cell_contexts.chunks(sub_batch_size) {
            let kb_context = if let Some(kb) = self.knowledge_base.lock().unwrap().as_ref() {
                format!("\nKnowledge Base Context:\n{}", kb.compressed_content)
            } else {
                String::new()
            };

            let cell_states = chunk
                .iter()
                .map(|(id, ctx)| {
                    format!(
                        "### CELL {}\nFOCUS: {}\nENERGY: {}\n",
                        id,
                        ctx.current_focus,
                        ctx.energy_level
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            let context_prompt = format!(
                r#"Contextual Analysis Framework:

                CURRENT OBJECTIVE: {}
                KNOWLEDGE CONTEXT: {}

                ENVIRONMENTAL SIGNALS:
                1. SYSTEM DYNAMICS
                   - Emerging patterns: {}
                   - Network effects
                   - Adaptation signals
                   - Behavioral shifts

                2. EVOLUTIONARY VECTORS
                   - Development paths: {}
                   - Growth patterns
                   - Adaptation cycles
                   - Scale dynamics

                3. EMERGENCE INDICATORS
                   - Novel properties: {}
                   - Feedback systems
                   - Pattern genesis
                   - System innovations

                4. BOUNDARY ANALYSIS
                   - Current limits: {}
                   - Growth potential
                   - System constraints
                   - Connection opportunities

                ENTITY STATES:
                {}

                Required Format (repeat for each cell):

                ### CELL <uuid>

                THOUGHT STRUCTURE:
                1. OBSERVATION
                   - Current state
                   - Key patterns
                   - System dynamics

                2. ANALYSIS
                   - Conventional wisdom
                   - Hidden assumptions
                   - Unexpected connections
                   - Evidence trail

                3. SYNTHESIS
                   - Novel perspective
                   - Strategic implications
                   - Cascading effects
                   - Action vectors

                THOUGHT: [Core insight challenging assumptions] (500+ words)
                RELEVANCE: <0.0-1.0>
                FACTORS: [Exactly 3 key factors]

                DIMENSIONS:
                - EMERGENT_INTELLIGENCE: <-100 to 100>
                - RESOURCE_EFFICIENCY: <-100 to 100>
                - NETWORK_COHERENCE: <-100 to 100>
                - GOAL_ALIGNMENT: <-100 to 100>
                - TEMPORAL_RESILIENCE: <-100 to 100>
                - DIMENSIONAL_INTEGRATION: <-100 to 100>

                DOPAMINE: <0.0-1.0>

                Rules:
                1. Each thought must follow the structured analysis framework
                2. All claims require concrete evidence from the last 72h
                3. UUIDs must be preserved exactly
                4. No empty sections allowed
                5. Include all three major components"#,
                colony_mission,
                kb_context,
                real_time_context
                    .market_trends
                    .first()
                    .unwrap_or(&String::new()),
                real_time_context
                    .technological_developments
                    .first()
                    .unwrap_or(&String::new()),
                real_time_context
                    .current_events
                    .first()
                    .unwrap_or(&String::new()),
                real_time_context
                    .user_interactions
                    .first()
                    .unwrap_or(&String::new()),
                cell_states
            );

            println!("\n║ Processing sub-batch of {} cells", chunk.len());
            match tokio::time::timeout(
                std::time::Duration::from_secs(100),
                self.query_llm(&context_prompt),
            )
            .await
            {
                Ok(Ok(response)) => {
                    if let Ok(results) = self.parse_batch_thought_response(&response) {
                        println!("║ Generated {} plans", results.len());
                        for (id, (thought, score, factors)) in &results {
                            println!("║");
                            println!("║ Cell {}", id);
                            println!("║ ├─ Score: {:.2}", score);
                            println!("║ ├─ Factors:");
                            for factor in factors {
                                println!("║ │  - {}", factor);
                            }
                            println!("║ └─ Thought: {:.100}...", thought);
                        }
                        all_results.extend(results);
                    } else {
                        eprintln!("Failed to parse results from response:\n{}", response);
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Error in sub-batch: {}", e);
                }
                Err(_) => {
                    eprintln!("Timeout in sub-batch 100s");
                }
            }
        }

        Ok(all_results)
    }

    pub async fn create_plan(
        &self,
        thoughts: &[Thought],
    ) -> Result<Plan, Box<dyn std::error::Error>> {
        let chunk_size = 5;
        let thought_chunks: Vec<Vec<Thought>> = thoughts
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().cloned().collect())
            .collect();

        let mut consolidated_plans = Vec::new();

        for chunk in thought_chunks {
            let thoughts_context = chunk
                .iter()
                .map(|t| format!("- {}", t.content))
                .collect::<Vec<_>>()
                .join("\n");

            let chunk_plan = self
                .query_llm(&format!(
                    r#"System Evolution Framework:

    CONTEXT SIGNALS:
    {}

    Analyze each vector as a complex adaptive system:
    1. NETWORK DYNAMICS
    - {{Flow patterns}}
    - {{Resource distribution}}
    - {{Connection topology}}
    - {{Interaction modes}}

    2. BOUNDARY CONDITIONS
    - {{Interface dynamics}}
    - {{Connection patterns}}
    - {{Integration vectors}}
    - {{Barrier dissolution}}

    3. EMERGENCE PATTERNS
    - {{Novel properties}}
    - {{Feedback cycles}}
    - {{Pattern evolution}}
    - {{System adaptations}}

    4. POTENTIAL SPACES
    - {{Unexplored capabilities}}
    - {{Constraint removal}}
    - {{Novel applications}}

    Required Format:

    SUMMARY: [Comprehensive system overview]

    For each component:
    COMPONENT: [Name]
    CONVENTIONAL VIEW: [Standard approach]
    RADICAL SHIFT: [New possibility]
    EVIDENCE: [Proof points]
    IMPLEMENTATION:
    - Technical specifications
    - Resource requirements
    - Timeline estimates
    - Success metrics
    - Risk assessment

    ARCHITECTURE:
    [Detailed ASCII mind map showing:]
    - Core components
    - Dependencies
    - Data flows
    - Integration points
    - System boundaries
    - Feedback loops
    
    Example format:
                                    [Core Goal]
                                        |
                    +-------------------+-------------------+
                    |                   |                   |
            [Component A]        [Component B]        [Component C]
                |                     |                    |
        +-------+-------+      +------+------+     +------+------+
        |       |       |      |      |      |     |      |      |
    [Task 1] [Task 2] [Task 3] ...  ...    ...   ...    ...    ...
    
    Use box drawing characters: ─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼

    INTEGRATION POINTS:
    - System connections
    - Data flows
    - Control mechanisms
    - Feedback loops

    Constraints:
    1. ONLY reference:
       - Active repository commits (72h)
       - Public experiments with results
       - Deployed systems with metrics
       - Official documentation

    2. EXCLUDE:
       - Unverified claims
       - Unreleased features
       - Future announcements
       - Speculative capabilities

    3. REQUIRED per component:
       - Commit hashes
       - Researcher names
       - Experiment results
       - Deployment metrics
       - Documentation links
                "#,
                    thoughts_context
                ))
                .await?;
            

            consolidated_plans.push(chunk_plan);
        }

        let combined_plan = self
            .query_llm(&format!(
                r#"System Integration Framework:

    COMPONENT PLANS:
    {}

    1. POWER DYNAMICS ANALYSIS
    - {{Control mechanisms}}
    - {{Resource flows  }}
    - {{Influence networks}} 
    - {{Authority structures}}

    2. SYSTEM BOUNDARIES ANALYSIS
    - {{Interface points}}
    - {{Connection patterns}}
    - {{Integration opportunities}}
    - {{Boundary dissolutions}}

    3. EMERGENCE VECTORS ANALYSIS
    - {{Unexpected properties}}
    - {{Feedback loops}}
    - {{Pattern formation}}
    - {{System surprises}}

    4. HIDDEN POTENTIALS ANALYSIS
    - {{Untapped capabilities that could result from this plan}}
    - {{Novel applications of the plans}}

    5. POWER DYNAMICS INTEGRATION
    - {{Control consolidation}}
    - {{Resource allocation}}
    - {{Influence flows}}
    - {{Authority structures}}

    6. TEMPORAL PATTERNS INTEGRATION
    - {{cellular Evolution paths}}
    - {{Decay patterns}}
    - {{Cyclic behaviors}}
    - {{Timescale interactions}}

    Required Format:

    MASTER PLAN:
    [2000+ word comprehensive integration]

    For each integration point:
    INTERFACE: [Name]
    CONVENTIONAL VIEW: [Standard approach]
    RADICAL SHIFT: [New possibility]
    EVIDENCE: [Proof points]
    IMPLICATIONS: [Cascading effects]

    ARCHITECTURE:
    [Detailed system architecture in ASCII:]
    - Show all components hierarchically 
    - Use box drawing chars (─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼)
    - Include data flows with arrows (← → ↑ ↓)
    - Mark critical paths with ***
    - Show interfaces with [brackets]
    - Indicate feedback loops with ⟲
    
    Example layout:
                            ┌──────────────┐
                            │ Master Plan  │
                            └──────┬───────┘
                                   │
                    ┌──────────────┴─────────────┐
                    │                            │
            ┌───────┴────────┐          ┌───────┴────────┐
            │ Component A    │          │ Component B    │
            └───────┬────────┘          └───────┬────────┘
                    │                           │
            ┌───────┴────────┐          ┌───────┴────────┐
            │    Tasks       │←─⟲─────→│   Feedback     │
            └───────┬────────┘          └───────┬────────┘
                    │                           │
            [Integration Points]         [System Bounds]

    Requirements:
    1. Logical component flow
    2. Clear dependencies
    3. Measurable outcomes
    4. Risk mitigations
    5. Resource allocations
                "#,
                consolidated_plans.join("\n\n=== Next Component ===\n\n")
            ))
            .await?;

        let enhanced_plan = self
            .query_llm(&format!(
                r#"Technical Integration Analysis Framework:

    BASE PLAN:
    {}

    1. POWER DYNAMICS VECTORS
    - Control shifts
    - Resource reallocations
    - Influence flows
    - Authority transitions

    2. SYSTEM BOUNDARIES
    - Interface mutations
    - Connection formations
    - Integration emergences 
    - Boundary dissolutions

    3. TEMPORAL PATTERNS
    - Evolution trajectories
    - Decay patterns
    - Cyclic behaviors
    - Timescale interactions

    4. EMERGENT PROPERTIES
    - Unexpected capabilities
    - Feedback loops
    - Pattern formations
    - System surprises

    Required Technical Analysis:
    1. Recent Developments (72h)
       - Commit activities
       - Research publications
       - Experiment results
       - Deployment metrics

    2. Integration Points
       - System interfaces
       - Data flows
       - Control mechanisms
       - Feedback systems

    3. Resource Requirements
       - Computational needs
       - Storage demands
       - Network capacity
       - Processing power

    4. Performance Metrics
       - Response times
       - Throughput rates
       - Error margins
       - Recovery speeds

    Format each component with:
    COMPONENT: [Name]
    TECHNICAL_BASELINE: [Current state]
    ENHANCEMENT_VECTOR: [Improvement path]
    EVIDENCE: [Proof points]
    METRICS: [Success measures]

    Requirements:
    1. 2000+ words
    2. RFP structure
    3. Technical precision
    4. Implementation focus
                "#,
                combined_plan
            ))
            .await?;
            

        let mut nodes = Vec::new();
        let mut current_node = None;
        let mut summary = String::new();
        let mut in_summary = false;
        let mut in_components = false;

        // Ensure we have at least one default node if none are parsed
        let default_node = PlanNode {
            id: Uuid::new_v4(),
            title: "Initial System Analysis".to_string(),
            description: "Analyze current system state and identify key improvement vectors".to_string(),
            dependencies: Vec::new(),
            estimated_completion: 0.2,
            status: PlanNodeStatus::Pending,
        };
        nodes.push(default_node);

        for line in enhanced_plan.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("SUMMARY:") {
                in_summary = true;
                summary = line.trim_start_matches("SUMMARY:").trim().to_string();
            } else if in_summary && !line.starts_with("COMPONENTS:") {
                summary.push_str(" ");
                summary.push_str(line);
            } else if line.starts_with("COMPONENTS:") || 
                      line.starts_with("IMPLEMENTATION:") ||
                      line.starts_with("TECHNICAL COMPONENTS:") {
                in_summary = false;
                in_components = true;
            } else if in_components {
                if line.starts_with(|c: char| c.is_digit(10)) || 
                   line.starts_with("COMPONENT:") ||
                   line.starts_with("INTERFACE:") {
                    if let Some(node) = current_node {
                        nodes.push(node);
                    }

                    let title = line
                        .trim_start_matches(|c: char| c.is_digit(10) || c == '.' || c == ':')
                        .trim_start_matches("COMPONENT")
                        .trim_start_matches("INTERFACE")
                        .trim()
                        .to_string();

                    current_node = Some(PlanNode {
                        id: Uuid::new_v4(),
                        title,
                        description: String::new(),
                        dependencies: Vec::new(),
                        estimated_completion: rand::thread_rng().gen_range(0.2..0.4),
                        status: PlanNodeStatus::Pending,
                    });
                } else if (line.starts_with('-') || line.starts_with("DESCRIPTION:")) && current_node.is_some() {
                    if let Some(ref mut node) = current_node {
                        if !node.description.is_empty() {
                            node.description.push_str("\n");
                        }
                        node.description.push_str(line
                            .trim_start_matches('-')
                            .trim_start_matches("DESCRIPTION:")
                            .trim());
                    }
                }
            }
        }

        if let Some(node) = current_node {
            nodes.push(node);
        }

        // If we still only have the default node, try to extract more from the summary
        if nodes.len() == 1 && !summary.is_empty() {
            for line in summary.lines() {
                if line.contains(':') || line.contains('-') {
                    nodes.push(PlanNode {
                        id: Uuid::new_v4(),
                        title: line.split(':').next().unwrap_or(line).trim().to_string(),
                        description: line.split(':').nth(1).unwrap_or("").trim().to_string(),
                        dependencies: Vec::new(),
                        estimated_completion: rand::thread_rng().gen_range(0.2..0.4),
                        status: PlanNodeStatus::Pending,
                    });
                }
            }
        }

        let score = if !nodes.is_empty() {
            nodes.iter().map(|n| n.estimated_completion).sum::<f64>() / nodes.len() as f64
        } else {
            0.0
        };

        Ok(Plan {
            id: Uuid::new_v4(),
            thoughts: thoughts.to_vec(),
            nodes,
            summary,
            score,
            participating_cells: Vec::new(),
            created_at: Utc::now(),
            status: PlanStatus::Proposed,
        })
    }

    pub async fn evaluate_dimensional_state(
        &self,
        dimensional_position: &DimensionalPosition,
        recent_thoughts: &[Thought],
        recent_plans: &[Plan],
    ) -> Result<(f64, f64), Box<dyn std::error::Error>> {
        let thoughts_context = recent_thoughts
            .iter()
            .map(|t| format!("- {}", t.content))
            .collect::<Vec<_>>()
            .join("\n");

        let plans_context = recent_plans
            .iter()
            .map(|p| format!("- {}: {}", p.id, p.summary))
            .collect::<Vec<_>>()
            .join("\n");

        let eval_prompt = format!(
            r#"Dimensional Analysis Framework:

    CURRENT STATE:
    Emergence: {:.2}
    Coherence: {:.2}
    Resilience: {:.2}
    Intelligence: {:.2}
    Efficiency: {:.2}
    Integration: {:.2}

    CONTEXT:
    THOUGHTS:
    {}

    PLANS:
    {}

    Analysis Vectors:

    Fill out each of the items on here in depth just like a CEO applies to take a company public.
    1. POWER DYNAMICS ANALYSIS
    - {{Control mechanisms}}
    - {{Resource flows  }}
    - {{Influence networks}} 
    - {{Authority structures}}

    2. SYSTEM BOUNDARIES ANALYSIS
    - {{Interface points}}
    - {{Connection patterns}}
    - {{Integration opportunities}}
    - {{Boundary dissolutions}}

    3. EMERGENCE VECTORS ANALYSIS
    - {{Unexpected properties}}
    - {{Feedback loops}}
    - {{Pattern formation}}
    - {{System surprises}}

    4. HIDDEN POTENTIALS ANALYSIS
    - {{Untapped capabilities that could result from this plan}}
    - Removed constraints
    - {{Novel applications of the plans}}

    Required Analysis Format:

    DIMENSIONAL_SCORES:
    For each dimension:
    DIMENSION: [Name]
    CONVENTIONAL VIEW: [Standard assessment]
    RADICAL INSIGHT: [Non-obvious observation]
    EVIDENCE: [Concrete proof points]
    SCORE: <-100 to 100>
    IMPLICATIONS: [Cascading effects]

    ENERGY_DYNAMICS:
    CURRENT_STATE: [Assessment]
    SHIFT_VECTOR: [Direction]
    MAGNITUDE: <-100 to 100>
    EVIDENCE: [Proof points]

    DOPAMINE_DYNAMICS:
    ENGAGEMENT_PATTERN: [Assessment]
    REINFORCEMENT_VECTOR: [Direction]
    MAGNITUDE: <0.0 to 1.0>
    EVIDENCE: [Proof points]

    Requirements:
    1. Evidence-based scoring
    2. Multi-framework analysis
    3. Pattern recognition
    4. Emergence identification
    5. System dynamics mapping
    "#,
            dimensional_position.emergence,
            dimensional_position.coherence,
            dimensional_position.resilience,
            dimensional_position.intelligence,
            dimensional_position.efficiency,
            dimensional_position.integration,
            thoughts_context,
            plans_context
        );

        let response = self.query_llm(&eval_prompt).await?;

        let mut energy_impact = 0.0;
        let mut dopamine_impact = 0.5;

        for line in response.lines() {
            let line = line.trim();
            if line.starts_with("MAGNITUDE:") {
                if energy_impact == 0.0 {
                    energy_impact = line
                        .trim_start_matches("MAGNITUDE:")
                        .trim()
                        .parse()
                        .unwrap_or(0.0);
                } else {
                    dopamine_impact = line
                        .trim_start_matches("MAGNITUDE:")
                        .trim()
                        .parse()
                        .unwrap_or(0.5);
                }
            }
        }

        Ok((energy_impact, dopamine_impact))
    }

    pub async fn generate_contextual_thought(
        &self,
        cell_context: &CellContext,
        real_time_context: &RealTimeContext,
        colony_mission: &str,
    ) -> Result<(String, f64, Vec<String>), Box<dyn std::error::Error>> {
        let filtered_mission = colony_mission.replace("quantum", "advanced");
        let context_prompt = format!(
            r#"Multi-Cell Analysis Framework:

    MISSION CONTEXT: {}

    ENVIRONMENTAL VECTORS:
    1. Market Dynamics: {}
    2. Technical Evolution: {}
    3. Event Patterns: {}
    4. Interaction Flows: {}

    CELL STATE:
    Focus: {}
    Research: {}
    Discoveries: {}
    Stage: {}
    Energy: {}
    Position: [{:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}]

    Analysis Framework:

    1. POWER DYNAMICS ANALYSIS
    - {{Control mechanisms}}
    - {{Resource flows  }}
    - {{Influence networks}} 
    - {{Authority structures}}

    2. SYSTEM BOUNDARIES ANALYSIS
    - {{Interface points}}
    - {{Connection patterns}}
    - {{Integration opportunities}}
    - {{Boundary dissolutions}}

    3. EMERGENCE VECTORS ANALYSIS
    - {{Unexpected properties}}
    - {{Feedback loops}}
    - {{Pattern formation}}
    - {{System surprises}}

    4. HIDDEN POTENTIALS ANALYSIS
    - {{Untapped capabilities that could result from this plan}}
    - Removed constraints
    - {{Novel applications of the plans}}

    Required Output Structure:

    THOUGHT ANALYSIS:
    CONVENTIONAL VIEW: [Standard interpretation]
    RADICAL SHIFT: [Non-obvious insight]
    EVIDENCE: [Concrete proof]
    IMPLICATIONS: [Cascading effects]

    THOUGHT: [Core idea challenging assumptions]
    RELEVANCE: <0.0-1.0>
    FACTORS: [Exactly 3 key factors]

    DIMENSIONAL SCORING:
    Each score must be a whole number (no decimals) within the specified range:
    - EMERGENT_INTELLIGENCE: <Whole number from -100 to 100>
    - RESOURCE_EFFICIENCY: <Whole number from 0 to 100>
    - NETWORK_COHERENCE: <Whole number from -100 to 100>
    - GOAL_ALIGNMENT: <Whole number from 0 to 100>
    - TEMPORAL_RESILIENCE: <Whole number from 0 to 100>
    - DIMENSIONAL_INTEGRATION: <Whole number from -100 to 100>

    DOPAMINE: <0 to 1>

    Requirements:
    1. Evidence-based analysis
    2. System-level thinking
    3. Pattern recognition
    4. Emergence identification
    5. Clear causality chains
    6. Unique thoughts
    7. Cross-system influences
    8. Include specific recent events and inventions in thought creation
    9. Reference concrete events and inventions from the last 72 hours
    10. Cite specific sources, commits, and timestamps for all events and inventions mentioned
            "#,
            filtered_mission,
            real_time_context.market_trends.join(", "),
            real_time_context.technological_developments.join(", "),
            real_time_context.current_events.join(", "),
            real_time_context.user_interactions.join(", "),
            cell_context.current_focus,
            cell_context.active_research_topics.join(", "),
            cell_context.recent_discoveries.join(", "),
            cell_context.evolution_stage,
            cell_context.energy_level,
            cell_context.dimensional_position.emergence,
            cell_context.dimensional_position.coherence,
            cell_context.dimensional_position.resilience,
            cell_context.dimensional_position.intelligence,
            cell_context.dimensional_position.efficiency,
            cell_context.dimensional_position.integration
        );

        let response = self.query_llm(&context_prompt).await?;
        

        let mut thought = String::new();
        let mut relevance = 0.0;
        let mut factors = Vec::new();

        for line in response.lines() {
            let line = line.trim();
            if line.starts_with("THOUGHT:") {
                let content = line.trim_start_matches("THOUGHT:").trim();
                if !content.is_empty() {
                    thought = content.to_string();
                }
            } else if line.starts_with("RELEVANCE:") || line.starts_with("**RELEVANCE:**") {
                let relevance_str = line
                    .trim_start_matches("**RELEVANCE:**")
                    .trim_start_matches("RELEVANCE:")
                    .trim_start_matches("**")
                    .trim_end_matches("**")
                    .trim();

                relevance = match relevance_str.parse::<f64>() {
                    Ok(val) => val.clamp(0.0, 1.0),
                    Err(_) => {
                        eprintln!("Error: Invalid relevance score '{}' for thought, using default 0.5", relevance_str);
                        0.5
                    }
                };
            } else if line.starts_with("FACTORS:") {
                factors = line
                    .trim_start_matches("FACTORS:")
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }

        // Validate thought content before returning
        if thought.trim().is_empty() {
            // Try to extract thought from THOUGHT ANALYSIS section
            if let Some(analysis_section) = response.split("THOUGHT ANALYSIS:").nth(1) {
                if let Some(end_idx) = analysis_section.find("DIMENSIONAL SCORING:") {
                    let analysis = analysis_section[..end_idx].trim();
                    
                    // Extract content between RADICAL SHIFT and EVIDENCE if present
                    if let Some(radical_shift) = analysis.split("RADICAL SHIFT:").nth(1) {
                        if let Some(end_evidence) = radical_shift.find("EVIDENCE:") {
                            thought = radical_shift[..end_evidence].trim().to_string();
                        }
                    }
                    
                    // If still empty, try to extract any substantial paragraph
                    if thought.trim().is_empty() {
                        thought = analysis
                            .lines()
                            .filter(|l| l.len() > 100) // Look for substantial paragraphs
                            .next()
                            .unwrap_or("Exploring system dynamics and adaptation patterns")
                            .to_string();
                    }
                }
            }
            
            // If still empty after trying to extract, use placeholder
            if thought.trim().is_empty() {
                eprintln!("Error: Could not extract valid thought from response:");
                eprintln!("═══════════════ LLM RESPONSE ═══════════════");
                eprintln!("{}", response);
                eprintln!("═══════════════════════════════════════════");
                eprintln!("Using placeholder thought instead");
                thought = "Exploring system dynamics and adaptation patterns".to_string();
            }
        }

        // Ensure we have at least one factor
        if factors.is_empty() {
            factors = vec![
                "System observation".to_string(),
                "Pattern analysis".to_string(),
                "Adaptation strategy".to_string()
            ];
        }

        // Normalize factors to exactly 3
        while factors.len() < 3 {
            factors.push("Continuing analysis".to_string());
        }
        factors.truncate(3);

        Ok((thought, relevance, factors))
    }

    fn parse_context_response(
        &self,
        response: &str,
    ) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
        let mut result = HashMap::new();
        let mut current_category = String::new();
        let mut current_items = Vec::new();

        for line in response.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.ends_with(':') {
                if !current_category.is_empty() {
                    result.insert(current_category.clone(), current_items.clone());
                    current_items.clear();
                }

                current_category = line.trim_end_matches(':').to_lowercase().replace(' ', "_");
            } else if line.starts_with('-') {
                current_items.push(line.trim_start_matches('-').trim().to_string());
            }
        }

        if !current_category.is_empty() {
            result.insert(current_category, current_items);
        }

        Ok(result)
    }

    fn parse_batch_thought_response(
        &self,
        response: &str,
    ) -> Result<HashMap<Uuid, (String, f64, Vec<String>)>, Box<dyn std::error::Error>> {
        let mut results = HashMap::new();
        
        // Split into cell sections, being more lenient with formatting
        let cell_sections: Vec<&str> = response
            .split(|c| c == '#' || c == '═' || c == '╔' || c == '╚')
            .filter(|s| s.contains("CELL"))
            .collect();

        for section in cell_sections {
            let mut lines = section.lines();
            let mut current_uuid = None;
            let mut current_thought = String::new();
            let mut current_relevance = 0.5; // Default relevance
            let mut current_factors = Vec::new();
            let mut in_thought = false;
            let mut thought_buffer = Vec::new();

            while let Some(line) = lines.next() {
                let line = line.trim();
                if line.is_empty() { continue; }

                // Extract UUID - now more flexible
                if line.contains("CELL") {
                    if let Some(uuid_str) = line
                        .split(|c: char| !c.is_ascii_hexdigit() && c != '-')
                        .find(|s| s.len() == 36) 
                    {
                        if let Ok(uuid) = Uuid::parse_str(uuid_str) {
                            current_uuid = Some(uuid);
                        }
                    }
                    continue;
                }

                // Handle thought analysis section
                if line.contains("THOUGHT ANALYSIS:") {
                    in_thought = true;
                    continue;
                }

                // Handle conventional/radical view sections
                if line.contains("**CONVENTIONAL VIEW:**") || line.contains("**RADICAL SHIFT:**") {
                    if !thought_buffer.is_empty() {
                        thought_buffer.push("\n"); // Add spacing between sections
                    }
                    thought_buffer.push(line.trim_matches('*').trim().to_string());
                    continue;
                }

                // Detect thought section with flexible markers
                if line.to_uppercase().contains("THOUGHT") && line.contains(':') {
                    in_thought = true;
                    if let Some(content) = line.split(':').nth(1) {
                        let cleaned = content.trim();
                        if !cleaned.is_empty() {
                            thought_buffer.push(cleaned.to_string());
                        }
                    }
                    continue;
                }

                // Parse relevance with improved number extraction
                if line.to_uppercase().contains("RELEVANCE") {
                    if let Some(value_str) = line.split(':').nth(1) {
                        if let Some(value) = value_str
                            .trim()
                            .split_whitespace()
                            .next()
                            .and_then(|s| s.parse::<f64>().ok())
                        {
                            current_relevance = value.clamp(0.0, 1.0);
                        }
                    }
                    continue;
                }

                // Parse factors with improved list handling
                if line.to_uppercase().contains("FACTORS") {
                    if !thought_buffer.is_empty() {
                        current_thought = thought_buffer.join("\n");
                        thought_buffer.clear();
                    }
                    
                    current_factors = line
                        .split(|c| c == ',' || c == ';' || c == '|')
                        .filter_map(|s| {
                            let cleaned = s.trim()
                                .trim_start_matches(|c| c == '-' || c == '*' || c == '[' || c == ']')
                                .trim_end_matches(|c| c == ']' || c == ':')
                                .trim();
                            if !cleaned.is_empty() {
                                Some(cleaned.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                    continue;
                }

                // Collect thought content
                if in_thought && 
                   !line.to_uppercase().contains("DIMENSION") && 
                   !line.to_uppercase().contains("DOPAMINE") &&
                   !line.to_uppercase().contains("RELEVANCE") &&
                   !line.to_uppercase().contains("FACTORS") {
                    if !line.trim().is_empty() {
                        thought_buffer.push(line.trim().to_string());
                    }
                }
            }

            // Finalize thought processing
            if !thought_buffer.is_empty() {
                current_thought = thought_buffer.join("\n");
            }

            // If thought is still empty but we have evidence sections, try to construct from those
            if current_thought.is_empty() {
                let evidence_sections: Vec<String> = thought_buffer.iter()
                    .filter(|line| line.contains("EVIDENCE:") || line.contains("IMPLICATIONS:"))
                    .map(|line| line.trim().to_string())
                    .collect();
                
                if !evidence_sections.is_empty() {
                    current_thought = evidence_sections.join("\n");
                }
            }

            // Only insert if we have valid data
            if let Some(uuid) = current_uuid {
                if !current_thought.is_empty() {
                    // Generate factors from thought content if none found
                    if current_factors.is_empty() {
                        current_factors = current_thought
                            .lines()
                            .filter(|l| l.contains(':') || l.contains('-'))
                            .take(3)
                            .map(|l| l.trim_start_matches(|c| c == '-' || c == '*' || c == '[')
                                 .trim_end_matches(|c| c == ']')
                                 .trim()
                                 .to_string())
                            .collect();
                        
                        if current_factors.is_empty() {
                            current_factors.push("General observation".to_string());
                        }
                    }
                    
                    results.insert(uuid, (current_thought, current_relevance, current_factors));
                }
            }
        }

        // Improved error reporting
        if results.is_empty() {
            eprintln!("Warning: Failed to parse any thoughts from response");
            eprintln!("Response structure:");
            for (i, line) in response.lines().enumerate() {
                eprintln!("{:3}: {}", i + 1, line);
            }
        }

        Ok(results)
    }

    pub async fn compress_memories(
        &self,
        memories: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let memories_text = memories.join("\n");
        let prompt = format!(
            r#"Memory Compression Framework:

    CONTENT:
    {}

    Analysis Vectors:
    1. CORE PATTERNS
       - Recurring themes
       - Common elements
       - Shared structures

    2. RELATIONSHIP MAPPING
       - Direct connections
       - Indirect links
       - Hidden dependencies

    3. KNOWLEDGE SYNTHESIS
       - Key insights
       - Critical learnings
       - Fundamental principles

    4. STRATEGIC IMPORTANCE
       - Action triggers
       - Decision points
       - Resource implications

    Required Format:
    PATTERN: [Identified pattern]
    EVIDENCE: [Supporting data]
    IMPLICATIONS: [Strategic impact]
    ACTIONABILITY: [Implementation path]

    Organize by:
    1. Impact magnitude
    2. Implementation feasibility
    3. Resource requirements
    4. Time sensitivity

    Focus on:
    1. Pattern emergence
    2. Strategic relevance
    3. Actionable insights
    4. Critical dependencies"#,
            memories_text
        );

        self.query_llm(&prompt).await
    }

    async fn compress_knowledge(
        &self,
        content: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!(
            r#"Knowledge Synthesis Framework:

    CONTENT:
    {}

    Analysis Vectors:
    1. CORE CONCEPTS
       - Fundamental principles
       - Key methodologies
       - Critical patterns

    2. RELATIONSHIP MAPPING
       - Direct connections
       - Indirect links
       - Hidden dependencies

    3. PATTERN RECOGNITION
       - Recurring themes
       - Common structures
       - Shared elements

    4. STRATEGIC RELEVANCE
       - Action implications
       - Decision impacts
       - Resource requirements

    Required Format:
    CONCEPT: [Core concept]
    EVIDENCE: [Supporting data]
    CONNECTIONS: [Related elements]
    IMPLICATIONS: [Strategic impact]

    Organize by:
    1. Impact magnitude
    2. Implementation relevance
    3. Resource implications
    4. Time sensitivity

    Preserve:
    1. Technical accuracy
    2. Source references
    3. Critical details
    4. Implementation paths"#,
            content
        );

        self.query_llm(&prompt).await
    }

    pub async fn query_llm(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let use_grok = rng.gen_bool(0.5);
        
        let model = if use_grok {
            "x-ai/grok-beta"
        } else {
            "x-ai/grok-beta"
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": model,
                "messages": [{
                    "role": "user",
                    "content": prompt
                }],
                "temperature": 0.7,
                "max_tokens": Self::get_max_tokens_for_model(model)
            }))
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        Ok(json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string())
    }

    pub async fn initialize_knowledge_base(&self) -> Result<(), Box<dyn std::error::Error>> {
        let files = KnowledgeBase::load_files("knowledgebase")?;

        if files.is_empty() {
            println!("No knowledge base files found in knowledgebase directory");
            return Ok(());
        }

        println!("Loading {} knowledge base files...", files.len());

        let combined_content = files
            .iter()
            .map(|(name, content)| format!("File: {}\n{}\n", name, content))
            .collect::<Vec<_>>()
            .join("\n---\n");

        let compressed = self.compress_knowledge(&combined_content).await?;

        let file_count = files.len();
        let kb = KnowledgeBase {
            compressed_content: compressed,
            last_updated: Utc::now(),
            source_files: files.into_iter().map(|(name, _)| name).collect(),
        };

        *self.knowledge_base.lock().unwrap() = Some(kb);
        println!("Knowledge base initialized with {} files", file_count);

        Ok(())
    }
}

