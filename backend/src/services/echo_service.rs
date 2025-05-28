use crate::models::{content::*, echo_index::*};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

pub struct EchoService;

impl EchoService {
    /// Calculate comprehensive Echo Index for content
    pub async fn calculate_echo_index(
        content: &Content,
        propagations: &[Propagation],
        interactions: &[AudienceMetrics],
    ) -> Result<EchoIndex, Box<dyn std::error::Error>> {
        // Analyze content to extract metrics
        let content_metrics = Self::analyze_content(&content.text).await?;
        
        // Calculate propagation metrics
        let propagation_metrics = Self::calculate_propagation_metrics(propagations).await?;
        
        // Calculate audience metrics (using first one or default)
        let audience_metrics = interactions.first().cloned().unwrap_or_default();
        
        // Calculate quote metrics
        let quote_metrics = Self::calculate_quote_metrics(content, propagations).await?;
        
        // Calculate individual components
        let odf = EchoIndexCalculator::calculate_odf(&content.text, &content_metrics);
        let awr = EchoIndexCalculator::calculate_awr(&audience_metrics);
        let tpm = EchoIndexCalculator::calculate_tpm(&propagation_metrics);
        let qf = EchoIndexCalculator::calculate_qf(&quote_metrics);
        
        // Calculate overall score
        let overall_score = EchoIndexCalculator::calculate_overall_score(odf, awr, tpm, qf);
        
        Ok(EchoIndex {
            originality_depth_factor: odf,
            audience_weight_rating: awr,
            transmission_path_mapping: tpm,
            quote_frequency: qf,
            overall_score,
        })
    }
    
    /// Analyze content to extract meaningful metrics
    async fn analyze_content(text: &str) -> Result<EchoMetrics, Box<dyn std::error::Error>> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();
        let unique_words = words.iter().collect::<std::collections::HashSet<_>>().len();
        
        // Simple sentiment analysis (placeholder for more sophisticated analysis)
        let sentiment_score = Self::calculate_sentiment(text).await?;
        
        // Basic readability score using Flesch formula approximation
        let readability_score = Self::calculate_readability(text).await?;
        
        // Detect originality markers
        let originality_markers = Self::detect_originality_markers(text).await?;
        
        Ok(EchoMetrics {
            content_length: text.len(),
            word_count,
            unique_words,
            sentiment_score,
            readability_score,
            originality_markers,
        })
    }
    
    /// Calculate propagation-related metrics
    async fn calculate_propagation_metrics(
        propagations: &[Propagation]
    ) -> Result<PropagationMetrics, Box<dyn std::error::Error>> {
        let total_propagations = propagations.len() as i32;
        let unique_propagators = propagations
            .iter()
            .map(|p| &p.from_user_id)
            .collect::<std::collections::HashSet<_>>()
            .len() as i32;
        
        // Calculate platform distribution
        let mut platform_distribution = HashMap::new();
        for propagation in propagations {
            *platform_distribution.entry(propagation.platform.clone()).or_insert(0) += 1;
        }
        
        // Calculate propagation velocity (propagations per hour)
        let propagation_velocity = if !propagations.is_empty() {
            let time_span = Self::calculate_time_span(propagations).await?;
            if time_span > 0.0 {
                total_propagations as f64 / time_span
            } else {
                0.0
            }
        } else {
            0.0
        };
        
        // Estimate network reach (unique users touched)
        let network_reach = unique_propagators * 2; // Simplified calculation
        
        Ok(PropagationMetrics {
            total_propagations,
            unique_propagators,
            platform_distribution,
            propagation_velocity,
            network_reach,
        })
    }
    
    /// Calculate quote-related metrics
    async fn calculate_quote_metrics(
        content: &Content,
        propagations: &[Propagation]
    ) -> Result<QuoteMetrics, Box<dyn std::error::Error>> {
        let direct_quotes = propagations
            .iter()
            .filter(|p| p.propagation_type == "quote")
            .count() as i32;
        
        let indirect_references = propagations
            .iter()
            .filter(|p| p.propagation_type == "mention" || p.propagation_type == "reply")
            .count() as i32;
        
        // Count discussion threads (simplified)
        let discussion_threads = propagations
            .iter()
            .filter(|p| p.propagation_type == "reply")
            .count() as i32;
        
        // Citation quality based on context and platform
        let citation_quality = if direct_quotes + indirect_references > 0 {
            Self::calculate_citation_quality(propagations).await?
        } else {
            0.0
        };
        
        Ok(QuoteMetrics {
            direct_quotes,
            indirect_references,
            discussion_threads,
            citation_quality,
        })
    }
    
    /// Calculate sentiment score using simple heuristics
    async fn calculate_sentiment(text: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let positive_words = [
            "good", "great", "excellent", "amazing", "brilliant", "innovative",
            "revolutionary", "breakthrough", "success", "positive", "love", "like"
        ];
        
        let negative_words = [
            "bad", "terrible", "awful", "horrible", "failure", "problem",
            "issue", "wrong", "negative", "hate", "dislike", "poor"
        ];
        
        let words: Vec<&str> = text.to_lowercase().split_whitespace().collect();
        let mut sentiment_score = 0.0;
        
        for word in &words {
            if positive_words.contains(word) {
                sentiment_score += 1.0;
            } else if negative_words.contains(word) {
                sentiment_score -= 1.0;
            }
        }
        
        // Normalize to [-1, 1] range
        if !words.is_empty() {
            sentiment_score /= words.len() as f64;
        }
        
        Ok(sentiment_score.max(-1.0).min(1.0))
    }
    
    /// Calculate readability score (simplified Flesch formula)
    async fn calculate_readability(text: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let sentences = text.split(&['.', '!', '?'][..]).count() as f64;
        let words = text.split_whitespace().count() as f64;
        let syllables = Self::count_syllables(text).await? as f64;
        
        if sentences == 0.0 || words == 0.0 {
            return Ok(0.0);
        }
        
        let avg_sentence_length = words / sentences;
        let avg_syllables_per_word = syllables / words;
        
        // Simplified Flesch Reading Ease formula
        let score = 206.835 - (1.015 * avg_sentence_length) - (84.6 * avg_syllables_per_word);
        
        // Normalize to [0, 1] range
        Ok((score / 100.0).max(0.0).min(1.0))
    }
    
    /// Count syllables in text (approximation)
    async fn count_syllables(text: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        let mut syllable_count = 0;
        
        for word in text.to_lowercase().split_whitespace() {
            let mut word_syllables = 0;
            let mut prev_was_vowel = false;
            
            for ch in word.chars() {
                let is_vowel = vowels.contains(&ch);
                if is_vowel && !prev_was_vowel {
                    word_syllables += 1;
                }
                prev_was_vowel = is_vowel;
            }
            
            // Every word has at least one syllable
            syllable_count += word_syllables.max(1);
        }
        
        Ok(syllable_count)
    }
    
    /// Detect originality markers in content
    async fn detect_originality_markers(text: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let originality_keywords = [
            "innovative", "revolutionary", "breakthrough", "novel", "unique",
            "pioneering", "cutting-edge", "disruptive", "transformative",
            "first-time", "never-before", "unprecedented", "groundbreaking"
        ];
        
        let mut markers = Vec::new();
        let text_lower = text.to_lowercase();
        
        for keyword in &originality_keywords {
            if text_lower.contains(keyword) {
                markers.push(keyword.to_string());
            }
        }
        
        Ok(markers)
    }
    
    /// Calculate time span of propagations in hours
    async fn calculate_time_span(propagations: &[Propagation]) -> Result<f64, Box<dyn std::error::Error>> {
        if propagations.is_empty() {
            return Ok(0.0);
        }
        
        let timestamps: Vec<DateTime<Utc>> = propagations
            .iter()
            .map(|p| p.timestamp.parse().unwrap_or_else(|_| Utc::now()))
            .collect();
        
        if let (Some(earliest), Some(latest)) = (timestamps.iter().min(), timestamps.iter().max()) {
            let duration = latest.signed_duration_since(*earliest);
            Ok(duration.num_seconds() as f64 / 3600.0) // Convert to hours
        } else {
            Ok(0.0)
        }
    }
    
    /// Calculate citation quality based on propagation context
    async fn calculate_citation_quality(propagations: &[Propagation]) -> Result<f64, Box<dyn std::error::Error>> {
        let mut quality_score = 0.0;
        let total_citations = propagations.len() as f64;
        
        if total_citations == 0.0 {
            return Ok(0.0);
        }
        
        for propagation in propagations {
            // Higher quality for direct quotes vs mentions
            let type_weight = match propagation.propagation_type.as_str() {
                "quote" => 1.0,
                "reply" => 0.8,
                "mention" => 0.6,
                _ => 0.4,
            };
            
            // Consider the weight of the propagation
            quality_score += propagation.weight * type_weight;
        }
        
        Ok((quality_score / total_citations).max(0.0).min(1.0))
    }
    
    /// Update Echo Index for existing content
    pub async fn update_echo_index(
        content_id: &str,
        new_propagations: &[Propagation]
    ) -> Result<EchoIndex, Box<dyn std::error::Error>> {
        // This would typically fetch the content from database
        // For now, we'll use placeholder logic
        
        // Recalculate with new propagations
        // This is a simplified version - in practice, you'd fetch all data
        let propagation_metrics = Self::calculate_propagation_metrics(new_propagations).await?;
        let tpm = EchoIndexCalculator::calculate_tpm(&propagation_metrics);
        
        // In a real implementation, you'd fetch existing ODF, AWR, QF values
        // and only recalculate TPM, then compute new overall score
        let odf = 0.8; // Placeholder - would come from existing calculation
        let awr = 0.7; // Placeholder - would come from existing calculation
        let qf = 0.6;  // Placeholder - would come from existing calculation
        
        let overall_score = EchoIndexCalculator::calculate_overall_score(odf, awr, tpm, qf);
        
        Ok(EchoIndex {
            originality_depth_factor: odf,
            audience_weight_rating: awr,
            transmission_path_mapping: tpm,
            quote_frequency: qf,
            overall_score,
        })
    }
}

impl Default for AudienceMetrics {
    fn default() -> Self {
        Self {
            total_interactions: 0,
            quality_interactions: 0,
            audience_diversity: 0.0,
            influencer_ratio: 0.0,
            engagement_depth: 0.0,
        }
    }
} 