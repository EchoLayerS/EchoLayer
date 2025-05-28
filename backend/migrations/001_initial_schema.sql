-- EchoLayer Database Schema Migration 001
-- Description: Initial database schema for EchoLayer platform
-- Created: 2024-01-01
-- Version: 1.0.0

-- Enable necessary extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "btree_gin";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- ====================================
-- USERS TABLE
-- ====================================
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet_address VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(50) UNIQUE,
    email VARCHAR(255) UNIQUE,
    bio TEXT,
    avatar_url TEXT,
    echo_score DECIMAL(10,2) DEFAULT 0.00,
    total_rewards DECIMAL(18,8) DEFAULT 0.00000000,
    rank INTEGER,
    is_verified BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    preferences JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Users indexes
CREATE INDEX idx_users_wallet_address ON users(wallet_address);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_echo_score ON users(echo_score DESC);
CREATE INDEX idx_users_rank ON users(rank);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_preferences ON users USING gin(preferences);

-- ====================================
-- SOCIAL PLATFORMS TABLE
-- ====================================
CREATE TYPE platform_type AS ENUM (
    'twitter',
    'telegram',
    'linkedin',
    'youtube',
    'instagram',
    'tiktok',
    'reddit'
);

CREATE TABLE social_platforms (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    platform platform_type NOT NULL,
    platform_user_id VARCHAR(255) NOT NULL,
    platform_username VARCHAR(255),
    access_token TEXT,
    access_token_secret TEXT,
    refresh_token TEXT,
    token_expires_at TIMESTAMP WITH TIME ZONE,
    is_connected BOOLEAN DEFAULT TRUE,
    verification_status VARCHAR(50) DEFAULT 'pending',
    last_sync_at TIMESTAMP WITH TIME ZONE,
    sync_status VARCHAR(50) DEFAULT 'success',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, platform)
);

-- Social platforms indexes
CREATE INDEX idx_social_platforms_user_id ON social_platforms(user_id);
CREATE INDEX idx_social_platforms_platform ON social_platforms(platform);
CREATE INDEX idx_social_platforms_platform_user_id ON social_platforms(platform_user_id);
CREATE INDEX idx_social_platforms_is_connected ON social_platforms(is_connected);

-- ====================================
-- CONTENT TABLE
-- ====================================
CREATE TYPE content_type AS ENUM (
    'text',
    'image',
    'video',
    'audio',
    'document',
    'link',
    'poll'
);

CREATE TYPE content_status AS ENUM (
    'draft',
    'active',
    'archived',
    'deleted',
    'flagged'
);

CREATE TABLE content (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    platform platform_type NOT NULL,
    external_id VARCHAR(255) NOT NULL,
    content_type content_type NOT NULL,
    title TEXT,
    body TEXT,
    media_urls TEXT[],
    tags TEXT[],
    echo_index DECIMAL(6,2) DEFAULT 0.00,
    propagation_count INTEGER DEFAULT 0,
    total_rewards DECIMAL(18,8) DEFAULT 0.00000000,
    status content_status DEFAULT 'active',
    platform_metadata JSONB DEFAULT '{}',
    echo_components JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(platform, external_id)
);

-- Content indexes
CREATE INDEX idx_content_user_id ON content(user_id);
CREATE INDEX idx_content_platform ON content(platform);
CREATE INDEX idx_content_external_id ON content(external_id);
CREATE INDEX idx_content_echo_index ON content(echo_index DESC);
CREATE INDEX idx_content_status ON content(status);
CREATE INDEX idx_content_created_at ON content(created_at DESC);
CREATE INDEX idx_content_tags ON content USING gin(tags);
CREATE INDEX idx_content_platform_metadata ON content USING gin(platform_metadata);
CREATE INDEX idx_content_title_body ON content USING gin(to_tsvector('english', COALESCE(title, '') || ' ' || COALESCE(body, '')));

-- ====================================
-- PROPAGATIONS TABLE
-- ====================================
CREATE TYPE propagation_type AS ENUM (
    'share',
    'repost',
    'quote',
    'mention',
    'link',
    'embed',
    'cross_post'
);

CREATE TABLE propagations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content_id UUID NOT NULL REFERENCES content(id) ON DELETE CASCADE,
    source_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    target_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    propagation_type propagation_type NOT NULL,
    source_platform platform_type NOT NULL,
    target_platform platform_type NOT NULL,
    source_external_id VARCHAR(255),
    target_external_id VARCHAR(255),
    echo_boost DECIMAL(4,2) DEFAULT 1.00,
    reward_amount DECIMAL(18,8) DEFAULT 0.00000000,
    engagement_metrics JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Propagations indexes
CREATE INDEX idx_propagations_content_id ON propagations(content_id);
CREATE INDEX idx_propagations_source_user_id ON propagations(source_user_id);
CREATE INDEX idx_propagations_target_user_id ON propagations(target_user_id);
CREATE INDEX idx_propagations_type ON propagations(propagation_type);
CREATE INDEX idx_propagations_source_platform ON propagations(source_platform);
CREATE INDEX idx_propagations_target_platform ON propagations(target_platform);
CREATE INDEX idx_propagations_created_at ON propagations(created_at DESC);

-- ====================================
-- ECHO INDEX CALCULATIONS TABLE
-- ====================================
CREATE TABLE echo_index_calculations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content_id UUID NOT NULL REFERENCES content(id) ON DELETE CASCADE,
    calculation_version VARCHAR(10) NOT NULL DEFAULT 'v1.0',
    odf_score DECIMAL(6,2) NOT NULL,
    awr_score DECIMAL(6,2) NOT NULL,
    tpm_score DECIMAL(6,2) NOT NULL,
    qf_score DECIMAL(6,2) NOT NULL,
    final_score DECIMAL(6,2) NOT NULL,
    components JSONB NOT NULL,
    calculation_metadata JSONB DEFAULT '{}',
    calculated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Echo index calculations indexes
CREATE INDEX idx_echo_calculations_content_id ON echo_index_calculations(content_id);
CREATE INDEX idx_echo_calculations_final_score ON echo_index_calculations(final_score DESC);
CREATE INDEX idx_echo_calculations_calculated_at ON echo_index_calculations(calculated_at DESC);

-- ====================================
-- REWARDS TABLE
-- ====================================
CREATE TYPE reward_type AS ENUM (
    'content_creation',
    'content_propagation',
    'platform_connection',
    'milestone_achievement',
    'referral_bonus',
    'community_contribution'
);

CREATE TYPE reward_status AS ENUM (
    'pending',
    'processing',
    'completed',
    'failed',
    'cancelled'
);

CREATE TABLE rewards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content_id UUID REFERENCES content(id) ON DELETE SET NULL,
    propagation_id UUID REFERENCES propagations(id) ON DELETE SET NULL,
    reward_type reward_type NOT NULL,
    amount DECIMAL(18,8) NOT NULL,
    token_symbol VARCHAR(10) DEFAULT 'ECH',
    reason TEXT,
    status reward_status DEFAULT 'pending',
    transaction_hash VARCHAR(255),
    blockchain_network VARCHAR(50) DEFAULT 'solana',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Rewards indexes
CREATE INDEX idx_rewards_user_id ON rewards(user_id);
CREATE INDEX idx_rewards_content_id ON rewards(content_id);
CREATE INDEX idx_rewards_type ON rewards(reward_type);
CREATE INDEX idx_rewards_status ON rewards(status);
CREATE INDEX idx_rewards_created_at ON rewards(created_at DESC);
CREATE INDEX idx_rewards_transaction_hash ON rewards(transaction_hash);

-- ====================================
-- ANALYTICS EVENTS TABLE
-- ====================================
CREATE TYPE event_type AS ENUM (
    'user_signup',
    'user_login',
    'platform_connected',
    'content_created',
    'content_shared',
    'reward_earned',
    'reward_withdrawn',
    'echo_index_calculated',
    'api_request'
);

CREATE TABLE analytics_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    session_id UUID,
    event_type event_type NOT NULL,
    event_name VARCHAR(255) NOT NULL,
    properties JSONB DEFAULT '{}',
    user_agent TEXT,
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Analytics events indexes
CREATE INDEX idx_analytics_events_user_id ON analytics_events(user_id);
CREATE INDEX idx_analytics_events_session_id ON analytics_events(session_id);
CREATE INDEX idx_analytics_events_type ON analytics_events(event_type);
CREATE INDEX idx_analytics_events_created_at ON analytics_events(created_at DESC);
CREATE INDEX idx_analytics_events_properties ON analytics_events USING gin(properties);

-- ====================================
-- USER SESSIONS TABLE
-- ====================================
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_token VARCHAR(255) NOT NULL UNIQUE,
    refresh_token VARCHAR(255) NOT NULL UNIQUE,
    ip_address INET,
    user_agent TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User sessions indexes
CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_session_token ON user_sessions(session_token);
CREATE INDEX idx_user_sessions_refresh_token ON user_sessions(refresh_token);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at);
CREATE INDEX idx_user_sessions_is_active ON user_sessions(is_active);

-- ====================================
-- NOTIFICATIONS TABLE
-- ====================================
CREATE TYPE notification_type AS ENUM (
    'reward_earned',
    'content_propagated',
    'rank_changed',
    'platform_connected',
    'system_announcement',
    'security_alert'
);

CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_type notification_type NOT NULL,
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    action_url TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Notifications indexes
CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_type ON notifications(notification_type);
CREATE INDEX idx_notifications_is_read ON notifications(is_read);
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);

-- ====================================
-- SYSTEM SETTINGS TABLE
-- ====================================
CREATE TABLE system_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key VARCHAR(255) NOT NULL UNIQUE,
    value JSONB NOT NULL,
    description TEXT,
    is_public BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- System settings indexes
CREATE INDEX idx_system_settings_key ON system_settings(key);
CREATE INDEX idx_system_settings_is_public ON system_settings(is_public);

-- ====================================
-- TRIGGERS
-- ====================================

-- Update updated_at timestamp trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_social_platforms_updated_at BEFORE UPDATE ON social_platforms FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_content_updated_at BEFORE UPDATE ON content FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_rewards_updated_at BEFORE UPDATE ON rewards FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_user_sessions_updated_at BEFORE UPDATE ON user_sessions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_system_settings_updated_at BEFORE UPDATE ON system_settings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ====================================
-- VIEWS
-- ====================================

-- User statistics view
CREATE VIEW user_stats AS
SELECT 
    u.id,
    u.username,
    u.echo_score,
    u.total_rewards,
    u.rank,
    COUNT(DISTINCT c.id) as content_count,
    COUNT(DISTINCT p.id) as propagation_count,
    COUNT(DISTINCT r.id) as reward_count,
    COALESCE(AVG(c.echo_index), 0) as avg_echo_index,
    COUNT(DISTINCT sp.platform) as connected_platforms_count
FROM users u
LEFT JOIN content c ON u.id = c.user_id AND c.status = 'active'
LEFT JOIN propagations p ON u.id = p.source_user_id
LEFT JOIN rewards r ON u.id = r.user_id AND r.status = 'completed'
LEFT JOIN social_platforms sp ON u.id = sp.user_id AND sp.is_connected = TRUE
GROUP BY u.id, u.username, u.echo_score, u.total_rewards, u.rank;

-- Content analytics view
CREATE VIEW content_analytics AS
SELECT 
    c.id,
    c.title,
    c.platform,
    c.echo_index,
    c.propagation_count,
    c.total_rewards,
    COUNT(p.id) as actual_propagations,
    COALESCE(AVG(eic.final_score), 0) as avg_calculation_score,
    u.username as creator_username
FROM content c
LEFT JOIN propagations p ON c.id = p.content_id
LEFT JOIN echo_index_calculations eic ON c.id = eic.content_id
LEFT JOIN users u ON c.user_id = u.id
WHERE c.status = 'active'
GROUP BY c.id, c.title, c.platform, c.echo_index, c.propagation_count, c.total_rewards, u.username;

-- Platform statistics view
CREATE VIEW platform_stats AS
SELECT 
    platform,
    COUNT(DISTINCT user_id) as connected_users,
    COUNT(DISTINCT CASE WHEN is_connected = TRUE THEN user_id END) as active_connections,
    AVG(CASE WHEN last_sync_at IS NOT NULL THEN EXTRACT(EPOCH FROM NOW() - last_sync_at) END) as avg_sync_interval
FROM social_platforms
GROUP BY platform;

-- ====================================
-- INITIAL DATA
-- ====================================

-- Insert default system settings
INSERT INTO system_settings (key, value, description, is_public) VALUES
('echo_index_weights', '{"odf": 0.30, "awr": 0.25, "tpm": 0.25, "qf": 0.20}', 'Echo Index calculation weights', TRUE),
('reward_rates', '{"content_creation": 10.0, "content_propagation": 5.0, "platform_connection": 25.0}', 'Base reward rates for different actions', FALSE),
('api_rate_limits', '{"general": 100, "analytics": 20, "auth": 10}', 'API rate limits per minute', FALSE),
('feature_flags', '{"echo_drops": true, "mpc_wallet": true, "advanced_analytics": true}', 'Feature toggle flags', TRUE),
('maintenance_mode', 'false', 'System maintenance status', TRUE),
('max_content_per_day', '50', 'Maximum content items per user per day', FALSE);

-- ====================================
-- FUNCTIONS
-- ====================================

-- Function to calculate user rank
CREATE OR REPLACE FUNCTION calculate_user_ranks()
RETURNS void AS $$
BEGIN
    WITH ranked_users AS (
        SELECT id, ROW_NUMBER() OVER (ORDER BY echo_score DESC) as new_rank
        FROM users
        WHERE is_active = TRUE
    )
    UPDATE users 
    SET rank = ranked_users.new_rank
    FROM ranked_users
    WHERE users.id = ranked_users.id;
END;
$$ LANGUAGE plpgsql;

-- Function to update content echo index
CREATE OR REPLACE FUNCTION update_content_echo_index(content_uuid UUID, new_echo_index DECIMAL)
RETURNS void AS $$
BEGIN
    UPDATE content 
    SET echo_index = new_echo_index, updated_at = NOW()
    WHERE id = content_uuid;
    
    -- Update user's total echo score
    UPDATE users 
    SET echo_score = (
        SELECT COALESCE(SUM(echo_index), 0) 
        FROM content 
        WHERE user_id = users.id AND status = 'active'
    )
    WHERE id = (SELECT user_id FROM content WHERE id = content_uuid);
END;
$$ LANGUAGE plpgsql;

-- Function to process reward
CREATE OR REPLACE FUNCTION process_reward(
    p_user_id UUID,
    p_amount DECIMAL,
    p_reward_type reward_type,
    p_reason TEXT DEFAULT NULL,
    p_content_id UUID DEFAULT NULL,
    p_propagation_id UUID DEFAULT NULL
)
RETURNS UUID AS $$
DECLARE
    reward_id UUID;
BEGIN
    INSERT INTO rewards (user_id, amount, reward_type, reason, content_id, propagation_id, status)
    VALUES (p_user_id, p_amount, p_reward_type, p_reason, p_content_id, p_propagation_id, 'pending')
    RETURNING id INTO reward_id;
    
    -- Update user's total rewards
    UPDATE users 
    SET total_rewards = total_rewards + p_amount
    WHERE id = p_user_id;
    
    RETURN reward_id;
END;
$$ LANGUAGE plpgsql;

-- ====================================
-- POLICIES (Row Level Security)
-- ====================================

-- Enable RLS on sensitive tables
ALTER TABLE user_sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE notifications ENABLE ROW LEVEL SECURITY;
ALTER TABLE rewards ENABLE ROW LEVEL SECURITY;

-- User sessions policy
CREATE POLICY user_sessions_policy ON user_sessions
    FOR ALL
    USING (user_id = current_setting('app.current_user_id')::UUID);

-- Notifications policy
CREATE POLICY notifications_policy ON notifications
    FOR ALL
    USING (user_id = current_setting('app.current_user_id')::UUID);

-- Rewards policy (users can only see their own rewards)
CREATE POLICY rewards_policy ON rewards
    FOR SELECT
    USING (user_id = current_setting('app.current_user_id')::UUID);

-- ====================================
-- COMMENTS
-- ====================================

COMMENT ON TABLE users IS 'Core user accounts and profiles';
COMMENT ON TABLE social_platforms IS 'Connected social media platform accounts';
COMMENT ON TABLE content IS 'User-generated content tracked across platforms';
COMMENT ON TABLE propagations IS 'Content propagation events between platforms';
COMMENT ON TABLE echo_index_calculations IS 'Echo Index calculation history and components';
COMMENT ON TABLE rewards IS 'User reward transactions and balances';
COMMENT ON TABLE analytics_events IS 'Platform analytics and user behavior tracking';
COMMENT ON TABLE user_sessions IS 'User authentication sessions and tokens';
COMMENT ON TABLE notifications IS 'User notifications and alerts';
COMMENT ON TABLE system_settings IS 'Platform configuration and feature flags';

-- Migration completed successfully
SELECT 'EchoLayer Database Schema Migration 001 completed successfully' as result; 