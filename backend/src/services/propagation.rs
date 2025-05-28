use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PropagationNode {
    pub id: String,
    pub node_type: NodeType,
    pub influence_weight: f64,
    pub reach: u32,
    pub engagement_rate: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    User,
    Content,
    Platform,
}

#[derive(Debug, Clone)]
pub struct PropagationPath {
    pub nodes: Vec<PropagationNode>,
    pub total_weight: f64,
    pub resonance_factor: f64,
    pub decay_rate: f64,
}

#[derive(Debug, Clone)]
pub struct EchoLoop {
    pub id: String,
    pub source_content_id: String,
    pub propagation_paths: Vec<PropagationPath>,
    pub total_resonance: f64,
    pub loop_strength: f64,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

pub struct PropagationService {
    active_loops: HashMap<String, EchoLoop>,
    max_loop_depth: usize,
    resonance_threshold: f64,
    decay_factor: f64,
}

impl PropagationService {
    pub fn new() -> Self {
        Self {
            active_loops: HashMap::new(),
            max_loop_depth: 10,
            resonance_threshold: 0.3,
            decay_factor: 0.9,
        }
    }

    /// Initialize a new Echo Loop for content
    pub fn create_echo_loop(&mut self, content_id: String) -> String {
        let loop_id = format!("loop_{}", uuid::Uuid::new_v4());
        let echo_loop = EchoLoop {
            id: loop_id.clone(),
            source_content_id: content_id,
            propagation_paths: Vec::new(),
            total_resonance: 0.0,
            loop_strength: 0.0,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        self.active_loops.insert(loop_id.clone(), echo_loop);
        loop_id
    }

    /// Add a propagation event to an existing Echo Loop
    pub fn add_propagation_event(
        &mut self,
        loop_id: &str,
        from_node: PropagationNode,
        to_node: PropagationNode,
        interaction_strength: f64,
    ) -> Result<(), String> {
        let echo_loop = self.active_loops.get_mut(loop_id)
            .ok_or_else(|| "Echo Loop not found".to_string())?;

        // Calculate propagation weight
        let propagation_weight = self.calculate_propagation_weight(&from_node, &to_node, interaction_strength);
        
        // Create or update propagation path
        let mut path_updated = false;
        for path in &mut echo_loop.propagation_paths {
            if let Some(last_node) = path.nodes.last() {
                if last_node.id == from_node.id {
                    path.nodes.push(to_node.clone());
                    path.total_weight += propagation_weight;
                    path_updated = true;
                    break;
                }
            }
        }

        if !path_updated {
            let new_path = PropagationPath {
                nodes: vec![from_node, to_node],
                total_weight: propagation_weight,
                resonance_factor: 0.0,
                decay_rate: self.decay_factor,
            };
            echo_loop.propagation_paths.push(new_path);
        }

        echo_loop.last_updated = Utc::now();
        self.update_echo_loop_metrics(loop_id)?;

        Ok(())
    }

    /// Calculate propagation weight between two nodes
    fn calculate_propagation_weight(
        &self,
        from_node: &PropagationNode,
        to_node: &PropagationNode,
        interaction_strength: f64,
    ) -> f64 {
        let influence_factor = from_node.influence_weight * 0.4;
        let reach_factor = (from_node.reach as f64).ln() / 20.0;
        let engagement_factor = from_node.engagement_rate * 0.3;
        let target_receptivity = to_node.engagement_rate * 0.3;

        (influence_factor + reach_factor + engagement_factor + target_receptivity) * interaction_strength
    }

    /// Update Echo Loop metrics and detect resonance
    fn update_echo_loop_metrics(&mut self, loop_id: &str) -> Result<(), String> {
        let echo_loop = self.active_loops.get_mut(loop_id)
            .ok_or_else(|| "Echo Loop not found".to_string())?;

        // Calculate total resonance
        let mut total_resonance = 0.0;
        for path in &mut echo_loop.propagation_paths {
            path.resonance_factor = self.calculate_path_resonance(path);
            total_resonance += path.resonance_factor;
        }

        echo_loop.total_resonance = total_resonance;

        // Calculate loop strength based on path convergence and resonance
        echo_loop.loop_strength = self.calculate_loop_strength(echo_loop);

        // Check for resonance amplification
        if echo_loop.total_resonance > self.resonance_threshold {
            self.apply_resonance_amplification(echo_loop);
        }

        Ok(())
    }

    /// Calculate resonance factor for a propagation path
    fn calculate_path_resonance(&self, path: &PropagationPath) -> f64 {
        if path.nodes.len() < 2 {
            return 0.0;
        }

        let mut resonance = 0.0;
        let mut weight_accumulator = 0.0;

        for i in 0..path.nodes.len() - 1 {
            let current_node = &path.nodes[i];
            let next_node = &path.nodes[i + 1];

            // Calculate node compatibility
            let compatibility = self.calculate_node_compatibility(current_node, next_node);
            weight_accumulator += compatibility;

            // Apply temporal decay
            let time_diff = (Utc::now() - current_node.timestamp).num_hours() as f64;
            let decay = self.decay_factor.powf(time_diff / 24.0);
            
            resonance += compatibility * decay;
        }

        if weight_accumulator > 0.0 {
            resonance / weight_accumulator
        } else {
            0.0
        }
    }

    /// Calculate compatibility between two nodes
    fn calculate_node_compatibility(&self, node1: &PropagationNode, node2: &PropagationNode) -> f64 {
        // Different node types have different compatibility factors
        let type_compatibility = match (&node1.node_type, &node2.node_type) {
            (NodeType::User, NodeType::User) => 0.8,
            (NodeType::User, NodeType::Content) => 0.9,
            (NodeType::Content, NodeType::User) => 0.9,
            (NodeType::Content, NodeType::Platform) => 0.7,
            (NodeType::Platform, NodeType::User) => 0.6,
            _ => 0.5,
        };

        let influence_sync = 1.0 - (node1.influence_weight - node2.influence_weight).abs();
        let engagement_sync = 1.0 - (node1.engagement_rate - node2.engagement_rate).abs();

        (type_compatibility + influence_sync + engagement_sync) / 3.0
    }

    /// Calculate overall loop strength
    fn calculate_loop_strength(&self, echo_loop: &EchoLoop) -> f64 {
        if echo_loop.propagation_paths.is_empty() {
            return 0.0;
        }

        let path_count = echo_loop.propagation_paths.len() as f64;
        let average_resonance = echo_loop.total_resonance / path_count;
        
        // Detect convergent paths (loops that circle back)
        let convergence_factor = self.detect_path_convergence(echo_loop);
        
        // Time factor (newer loops are stronger)
        let age_hours = (Utc::now() - echo_loop.created_at).num_hours() as f64;
        let time_factor = (1.0 / (1.0 + age_hours * 0.01)).max(0.1);

        (average_resonance * 0.5 + convergence_factor * 0.3 + time_factor * 0.2).min(1.0)
    }

    /// Detect if propagation paths form convergent loops
    fn detect_path_convergence(&self, echo_loop: &EchoLoop) -> f64 {
        let mut node_visits: HashMap<String, usize> = HashMap::new();
        let mut total_nodes = 0;

        for path in &echo_loop.propagation_paths {
            for node in &path.nodes {
                *node_visits.entry(node.id.clone()).or_insert(0) += 1;
                total_nodes += 1;
            }
        }

        let repeated_nodes = node_visits.values().filter(|&&count| count > 1).count();
        if total_nodes > 0 {
            repeated_nodes as f64 / total_nodes as f64
        } else {
            0.0
        }
    }

    /// Apply resonance amplification when threshold is exceeded
    fn apply_resonance_amplification(&mut self, echo_loop: &mut EchoLoop) {
        let amplification_factor = 1.0 + (echo_loop.total_resonance - self.resonance_threshold) * 0.5;
        
        for path in &mut echo_loop.propagation_paths {
            path.total_weight *= amplification_factor;
            path.resonance_factor *= amplification_factor.min(1.5);
        }

        echo_loop.total_resonance *= amplification_factor.min(1.3);
    }

    /// Get active Echo Loops for a content piece
    pub fn get_content_echo_loops(&self, content_id: &str) -> Vec<&EchoLoop> {
        self.active_loops
            .values()
            .filter(|loop_| loop_.source_content_id == content_id)
            .collect()
    }

    /// Clean up expired Echo Loops
    pub fn cleanup_expired_loops(&mut self, max_age_hours: i64) {
        let cutoff_time = Utc::now() - chrono::Duration::hours(max_age_hours);
        
        self.active_loops.retain(|_, echo_loop| {
            echo_loop.last_updated > cutoff_time && echo_loop.loop_strength > 0.1
        });
    }

    /// Get propagation analytics for a time period
    pub fn get_propagation_analytics(&self, since: DateTime<Utc>) -> PropagationAnalytics {
        let relevant_loops: Vec<&EchoLoop> = self.active_loops
            .values()
            .filter(|loop_| loop_.created_at >= since)
            .collect();

        let total_loops = relevant_loops.len();
        let avg_loop_strength = if total_loops > 0 {
            relevant_loops.iter().map(|l| l.loop_strength).sum::<f64>() / total_loops as f64
        } else {
            0.0
        };

        let total_paths: usize = relevant_loops.iter().map(|l| l.propagation_paths.len()).sum();
        let high_resonance_loops = relevant_loops
            .iter()
            .filter(|l| l.total_resonance > self.resonance_threshold)
            .count();

        PropagationAnalytics {
            total_loops,
            avg_loop_strength,
            total_propagation_paths: total_paths,
            high_resonance_loops,
            resonance_threshold: self.resonance_threshold,
        }
    }
}

#[derive(Debug)]
pub struct PropagationAnalytics {
    pub total_loops: usize,
    pub avg_loop_strength: f64,
    pub total_propagation_paths: usize,
    pub high_resonance_loops: usize,
    pub resonance_threshold: f64,
} 